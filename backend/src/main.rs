use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTask {
    pub id: String,
    pub approver: String,
    pub approver_level: u32, // 1=employee, 2=manager, 3=gm, 4=admin
    pub status: String,
    pub submitted_at: String,
    pub escalated: bool,
}

/// BUG: not implemented - should check submitted_at + timeout > now
pub fn check_escalation(tasks: &mut Vec<ApprovalTask>, timeout_hours: u64) -> Vec<String> {
    // TODO: implement
    vec![]
}

/// BUG: no upper limit check - if admin doesn't handle, escalation continues forever
pub fn get_escalation_target(level: u32) -> Option<String> {
    match level {
        1 => Some("manager".into()),
        2 => Some("gm".into()),
        3 => Some("admin".into()),
        // BUG: level 4+ should return None (no more escalation) but doesn't
        _ => Some("super_admin".into()), // BUG: creates infinite escalation chain
    }
}

#[derive(Parser, Debug)]
#[command(name = "workflow-escalation", about = "Approval timeout escalation")]
struct Cli { #[arg(long, default_value = "24")] timeout_hours: u64 }

fn main() -> Result<()> {
    let mut tasks = vec![ApprovalTask { id: "t1".into(), approver: "zhangsan".into(), approver_level: 2, status: "pending".into(), submitted_at: "2026-05-10T08:00:00Z".into(), escalated: false }];
    let escalated = check_escalation(&mut tasks, 24);
    println!("escalated: {:?}", escalated);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escalation_has_upper_limit() {
        // Level 4 (admin) should not escalate further
        let target = get_escalation_target(4);
        assert!(target.is_none(), "admin level should have no further escalation target");
    }

    #[test]
    fn test_check_escalation_not_implemented() {
        let mut tasks = vec![ApprovalTask { id: "t1".into(), approver: "zhangsan".into(), approver_level: 2, status: "pending".into(), submitted_at: "2026-01-01T00:00:00Z".into(), escalated: false }];
        let escalated = check_escalation(&mut tasks, 24);
        // Should detect timeout but returns empty
        assert!(!escalated.is_empty(), "should detect timed-out task");
    }
}
