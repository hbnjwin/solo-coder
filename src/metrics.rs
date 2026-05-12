use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String,
    pub applicant: String,
    pub approver: String,
    pub approval_type: String,
    pub department: String,
    pub status: String,
    pub submitted_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricResult {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub sample_size: usize,
}

pub fn load_records(path: &str) -> Result<Vec<ApprovalRecord>> {
    let content = std::fs::read_to_string(path)?;
    let records: Vec<ApprovalRecord> = serde_json::from_str(&content)?;
    Ok(records)
}

pub fn calculate_approval_rate(records: &[ApprovalRecord]) -> MetricResult {
    let total = records.len();
    let approved = records.iter().filter(|r| r.status == "approved").count();
    let rate = if total > 0 { (approved as f64 / total as f64) * 100.0 } else { 0.0 };
    MetricResult {
        name: "approval_rate".to_string(),
        value: rate,
        unit: "%".to_string(),
        sample_size: total,
    }
}

pub fn calculate_avg_duration(records: &[ApprovalRecord]) -> MetricResult {
    let durations: Vec<f64> = records
        .iter()
        .filter_map(|r| {
            let submitted = chrono::DateTime::parse_from_rfc3339(&r.submitted_at).ok()?;
            let completed = r.completed_at.as_ref()?;
            let completed_dt = chrono::DateTime::parse_from_rfc3339(completed).ok()?;
            let diff = completed_dt.signed_duration_since(submitted);
            Some(diff.num_minutes() as f64)
        })
        .collect();

    let avg = if durations.is_empty() { 0.0 } else { durations.iter().sum::<f64>() / durations.len() as f64 };
    MetricResult {
        name: "avg_duration".to_string(),
        value: avg,
        unit: "minutes".to_string(),
        sample_size: durations.len(),
    }
}

pub fn identify_bottlenecks(records: &[ApprovalRecord]) -> Vec<String> {
    let mut approver_durations: HashMap<String, Vec<f64>> = HashMap::new();
    for record in records {
        if let (Ok(submitted), Some(completed_str)) = (
            chrono::DateTime::parse_from_rfc3339(&record.submitted_at),
            &record.completed_at,
        ) {
            if let Ok(completed) = chrono::DateTime::parse_from_rfc3339(completed_str) {
                let diff = completed.signed_duration_since(submitted).num_minutes() as f64;
                approver_durations.entry(record.approver.clone()).or_default().push(diff);
            }
        }
    }

    let overall_avg = {
        let all_durations: Vec<f64> = approver_durations.values().flatten().copied().collect();
        if all_durations.is_empty() { return vec![]; }
        all_durations.iter().sum::<f64>() / all_durations.len() as f64
    };

    approver_durations
        .iter()
        .filter_map(|(approver, durations)| {
            let avg = durations.iter().sum::<f64>() / durations.len() as f64;
            if avg > overall_avg * 1.5 {
                Some(approver.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn group_by_field<'a>(records: &'a [ApprovalRecord], field: &str) -> HashMap<String, Vec<&'a ApprovalRecord>> {
    let mut groups: HashMap<String, Vec<&ApprovalRecord>> = HashMap::new();
    for record in records {
        let key = match field {
            "department" => record.department.clone(),
            "approver" => record.approver.clone(),
            "approval_type" => record.approval_type.clone(),
            _ => continue,
        };
        groups.entry(key).or_default().push(record);
    }
    groups
}

pub fn format_metrics(metrics: &[MetricResult]) -> String {
    let mut output = String::new();
    for m in metrics {
        output.push_str(&format!("{}: {:.2} {} (n={})\n", m.name, m.value, m.unit, m.sample_size));
    }
    output
}

pub fn to_json(metrics: &[MetricResult]) -> String {
    serde_json::to_string_pretty(metrics).unwrap_or_else(|_| "[]".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_records() -> Vec<ApprovalRecord> {
        vec![
            ApprovalRecord {
                id: "1".to_string(),
                applicant: "alice".to_string(),
                approver: "bob".to_string(),
                approval_type: "contract".to_string(),
                department: "finance".to_string(),
                status: "approved".to_string(),
                submitted_at: "2026-01-01T09:00:00Z".to_string(),
                completed_at: Some("2026-01-01T10:00:00Z".to_string()),
            },
            ApprovalRecord {
                id: "2".to_string(),
                applicant: "carol".to_string(),
                approver: "dave".to_string(),
                approval_type: "expense".to_string(),
                department: "hr".to_string(),
                status: "rejected".to_string(),
                submitted_at: "2026-01-01T09:00:00Z".to_string(),
                completed_at: Some("2026-01-01T12:00:00Z".to_string()),
            },
            ApprovalRecord {
                id: "3".to_string(),
                applicant: "eve".to_string(),
                approver: "bob".to_string(),
                approval_type: "leave".to_string(),
                department: "finance".to_string(),
                status: "approved".to_string(),
                submitted_at: "2026-01-01T09:00:00Z".to_string(),
                completed_at: Some("2026-01-01T09:30:00Z".to_string()),
            },
        ]
    }

    #[test]
    fn test_approval_rate() {
        let records = make_records();
        let result = calculate_approval_rate(&records);
        assert!((result.value - 66.67).abs() < 0.1);
        assert_eq!(result.sample_size, 3);
    }

    #[test]
    fn test_avg_duration() {
        let records = make_records();
        let result = calculate_avg_duration(&records);
        assert!((result.value - 90.0).abs() < 0.1);
    }

    #[test]
    fn test_bottleneck_detection() {
        let records = make_records();
        let bottlenecks = identify_bottlenecks(&records);
        assert!(bottlenecks.contains(&"dave".to_string()));
    }

    #[test]
    fn test_group_by_department() {
        let records = make_records();
        let groups = group_by_field(&records, "department");
        assert_eq!(groups.get("finance").map(|v| v.len()), Some(2));
        assert_eq!(groups.get("hr").map(|v| v.len()), Some(1));
    }
}
