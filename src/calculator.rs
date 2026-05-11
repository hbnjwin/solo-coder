use chrono::NaiveDateTime;

use crate::models::{ApprovalRecord, MetricResult, DATETIME_FORMAT};

/// Computes the approval rate as a percentage of approved records
/// relative to the total record count.
pub fn calculate_approval_rate(records: &[ApprovalRecord]) -> MetricResult {
    if records.is_empty() {
        return MetricResult {
            name: "approval_rate".to_string(),
            value: 0.0,
            unit: "%".to_string(),
            sample_size: 0,
        };
    }

    let approved_count = records
        .iter()
        .filter(|r| r.status == "approved")
        .count();

    let total = records.len();
    let rate = (approved_count as f64 / total as f64) * 100.0;

    MetricResult {
        name: "approval_rate".to_string(),
        value: rate,
        unit: "%".to_string(),
        sample_size: total,
    }
}

/// Computes the average processing duration in hours across all records.
/// Parses submitted_at and completed_at timestamps to derive elapsed time.
pub fn calculate_avg_duration(records: &[ApprovalRecord]) -> MetricResult {
    if records.is_empty() {
        return MetricResult {
            name: "avg_duration".to_string(),
            value: 0.0,
            unit: "hours".to_string(),
            sample_size: 0,
        };
    }

    let mut total_hours: f64 = 0.0;

    for record in records {
        let start = NaiveDateTime::parse_from_str(&record.submitted_at, DATETIME_FORMAT);
        let end = record
            .completed_at
            .as_ref()
            .and_then(|ts| NaiveDateTime::parse_from_str(ts, DATETIME_FORMAT).ok());

        if let (Ok(s), Some(e)) = (start, end) {
            let duration = e.signed_duration_since(s);
            total_hours += duration.num_minutes() as f64 / 60.0;
        }
    }

    let avg = total_hours / records.len() as f64;

    MetricResult {
        name: "avg_duration".to_string(),
        value: avg,
        unit: "hours".to_string(),
        sample_size: records.len(),
    }
}

/// Normalizes duration values from various input formats (seconds, minutes, hours)
/// into a consistent hours representation for aggregation.
pub fn normalize_duration(value: f64, unit: &str) -> f64 {
    match unit.to_lowercase().as_str() {
        "seconds" | "sec" | "s" => value / 3600.0,
        "minutes" | "min" | "m" => value / 60.0,
        "hours" | "hr" | "h" => value,
        "days" | "day" | "d" => value * 24.0,
        "weeks" | "week" | "w" => value * 24.0 * 7.0,
        _ => {
            if value > 1000.0 {
                value / 3600.0
            } else if value > 100.0 {
                value / 60.0
            } else {
                value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_seconds() {
        let result = normalize_duration(7200.0, "seconds");
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize_minutes() {
        let result = normalize_duration(120.0, "min");
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize_days() {
        let result = normalize_duration(1.0, "days");
        assert!((result - 24.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize_unknown_large() {
        let result = normalize_duration(3600.0, "unknown");
        assert!((result - 1.0).abs() < 0.001);
    }
}
