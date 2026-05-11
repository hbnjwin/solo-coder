use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String, pub dept: String, pub approval_type: String,
    pub status: String, pub submit_time: String, pub complete_time: Option<String>,
}

/// BUG: includes pending records in denominator, lowering the rate
pub fn calculate_approval_rate(records: &[ApprovalRecord]) -> f64 {
    if records.is_empty() { return 0.0; }
    let approved = records.iter().filter(|r| r.status == "approved").count();
    (approved as f64) / (records.len() as f64) * 100.0  // BUG: should exclude pending
}

/// TODO: not implemented
pub fn calculate_avg_duration(_records: &[ApprovalRecord]) -> f64 { 0.0 }

/// TODO: not implemented
pub fn identify_bottlenecks(_records: &[ApprovalRecord]) -> Vec<String> { vec![] }

/// TODO: not implemented
pub fn group_by(_records: &[ApprovalRecord], _field: &str) -> std::collections::HashMap<String, Vec<ApprovalRecord>> {
    std::collections::HashMap::new()
}

#[derive(Parser, Debug)]
#[command(name = "approval-metrics", about = "Approval metrics calculator")]
struct Cli { #[arg(long, default_value = "terminal")] output: String }

fn main() -> Result<()> {
    let records = vec![
        ApprovalRecord { id: "1".into(), dept: "finance".into(), approval_type: "contract".into(), status: "approved".into(), submit_time: "2026-01-01 09:00:00".into(), complete_time: Some("2026-01-02 15:00:00".into()) },
        ApprovalRecord { id: "2".into(), dept: "finance".into(), approval_type: "contract".into(), status: "rejected".into(), submit_time: "2026-01-03 10:00:00".into(), complete_time: Some("2026-01-03 16:00:00".into()) },
        ApprovalRecord { id: "3".into(), dept: "hr".into(), approval_type: "leave".into(), status: "pending".into(), submit_time: "2026-01-05 08:00:00".into(), complete_time: None },
    ];
    println!("approval rate: {:.1}%", calculate_approval_rate(&records));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approval_rate_excludes_pending() {
        // 1 approved, 1 rejected, 1 pending => rate should be 50% (1/2), not 33% (1/3)
        let records = vec![
            ApprovalRecord { id: "1".into(), dept: "f".into(), approval_type: "c".into(), status: "approved".into(), submit_time: "".into(), complete_time: Some("".into()) },
            ApprovalRecord { id: "2".into(), dept: "f".into(), approval_type: "c".into(), status: "rejected".into(), submit_time: "".into(), complete_time: Some("".into()) },
            ApprovalRecord { id: "3".into(), dept: "h".into(), approval_type: "l".into(), status: "pending".into(), submit_time: "".into(), complete_time: None },
        ];
        let rate = calculate_approval_rate(&records);
        // BUG: currently returns 33.3% (1/3), should be 50% (1/2)
        assert!((rate - 50.0).abs() < 0.1, "pending should be excluded from denominator, expected 50% got {:.1}%", rate);
    }

    #[test]
    fn test_avg_duration_not_implemented() {
        let records = vec![ApprovalRecord { id: "1".into(), dept: "f".into(), approval_type: "c".into(), status: "approved".into(), submit_time: "2026-01-01 09:00:00".into(), complete_time: Some("2026-01-02 15:00:00".into()) }];
        let duration = calculate_avg_duration(&records);
        assert!(duration > 0.0, "avg duration should be calculated");
    }

    #[test]
    fn test_group_by_not_implemented() {
        let records = vec![
            ApprovalRecord { id: "1".into(), dept: "finance".into(), approval_type: "contract".into(), status: "approved".into(), submit_time: "".into(), complete_time: None },
            ApprovalRecord { id: "2".into(), dept: "hr".into(), approval_type: "leave".into(), status: "approved".into(), submit_time: "".into(), complete_time: None },
        ];
        let groups = group_by(&records, "dept");
        assert!(!groups.is_empty(), "group_by should return grouped records");
        assert!(groups.contains_key("finance"), "should have finance group");
    }
}
