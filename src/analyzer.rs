use std::collections::HashMap;

use crate::models::{ApprovalRecord, GroupKey, MetricResult, TimeRange};

/// Identifies approvers with the longest average processing times.
/// Returns a list of approver names sorted by their average duration.
pub fn identify_bottlenecks(records: &[ApprovalRecord]) -> Vec<String> {
    let mut approver_durations: HashMap<String, Vec<f64>> = HashMap::new();

    for record in records {
        if let Some(hours) = record.duration_hours() {
            approver_durations
                .entry(record.approver.clone())
                .or_default()
                .push(hours);
        }
    }

    let mut averages: Vec<(String, f64)> = approver_durations
        .into_iter()
        .map(|(approver, durations)| {
            let avg = durations.iter().sum::<f64>() / durations.len() as f64;
            (approver, avg)
        })
        .collect();

    averages.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    averages.into_iter().map(|(name, _)| name).collect()
}

/// Groups records by a specified field and returns the mapping.
pub fn group_by_field(records: &[ApprovalRecord], field: &str) -> HashMap<String, Vec<ApprovalRecord>> {
    let mut groups: HashMap<String, Vec<ApprovalRecord>> = HashMap::new();

    for record in records {
        if let Some(key) = GroupKey::from_field(record, field) {
            groups
                .entry(key.label().to_string())
                .or_default()
                .push(record.clone());
        }
    }

    groups
}

/// Analyzes approval rate trends over time by splitting records into
/// equal-sized time windows within the given range.
pub fn trend_analysis(
    records: &[ApprovalRecord],
    range: &TimeRange,
    window_count: usize,
) -> Vec<MetricResult> {
    use chrono::NaiveDateTime;
    use crate::models::DATETIME_FORMAT;

    let start = match NaiveDateTime::parse_from_str(&range.start, DATETIME_FORMAT) {
        Ok(dt) => dt,
        Err(_) => return vec![],
    };
    let end = match NaiveDateTime::parse_from_str(&range.end, DATETIME_FORMAT) {
        Ok(dt) => dt,
        Err(_) => return vec![],
    };

    let total_duration = end.signed_duration_since(start);
    let window_minutes = total_duration.num_minutes() / window_count as i64;

    if window_minutes <= 0 {
        return vec![];
    }

    let mut results = Vec::new();

    for i in 0..window_count {
        let w_start = start + chrono::Duration::minutes(window_minutes * i as i64);
        let w_end = start + chrono::Duration::minutes(window_minutes * (i + 1) as i64);

        let window_records: Vec<&ApprovalRecord> = records
            .iter()
            .filter(|r| {
                if let Some(ts) = r.submitted_datetime() {
                    ts >= w_start && ts < w_end
                } else {
                    false
                }
            })
            .collect();

        let approved = window_records.iter().filter(|r| r.status == "approved").count();
        let resolved = window_records.iter().filter(|r| r.is_resolved()).count();

        let rate = if resolved > 0 {
            (approved as f64 / resolved as f64) * 100.0
        } else {
            0.0
        };

        results.push(MetricResult {
            name: format!("window_{}", i + 1),
            value: rate,
            unit: "%".to_string(),
            sample_size: window_records.len(),
        });
    }

    results
}
