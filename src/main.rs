use anyhow::Result;
use chrono::Local;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRule {
    pub from_approver: String,
    pub to_approver: String,
    pub start_date: String,
    pub end_date: String,
    pub reason: String,
}

/// Resolve effective approver. TODO: support delegation chains (A->B->C)
pub fn resolve_approver(original: &str, delegations: &[DelegationRule], today: &str) -> Result<String> {
    for d in delegations {
        if d.from_approver == original && today >= &d.start_date && today <= &d.end_date {
            return Ok(d.to_approver.clone());
        }
    }
    Ok(original.to_string())
}

/// BUG: always returns Ok, never detects circular delegation
pub fn detect_circular_delegation(_delegations: &[DelegationRule]) -> Result<()> {
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "approval-delegation", about = "Approval delegation manager")]
struct Cli {
    #[arg(long, default_value = "zhangsan")]
    approver: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let delegations = vec![DelegationRule {
        from_approver: "zhangsan".into(), to_approver: "lisi".into(),
        start_date: "2026-05-01".into(), end_date: "2026-05-10".into(), reason: "vacation".into(),
    }];
    let today = Local::now().format("%Y-%m-%d").to_string();
    let effective = resolve_approver(&cli.approver, &delegations, &today)?;
    println!("effective approver for {}: {}", cli.approver, effective);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_circular_not_detected() {
        let delegations = vec![
            DelegationRule { from_approver: "a".into(), to_approver: "b".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
            DelegationRule { from_approver: "b".into(), to_approver: "a".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
        ];
        assert!(detect_circular_delegation(&delegations).is_err(), "circular should be detected");
    }
    #[test]
    fn test_chain_not_supported() {
        let delegations = vec![
            DelegationRule { from_approver: "a".into(), to_approver: "b".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
            DelegationRule { from_approver: "b".into(), to_approver: "c".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
        ];
        let result = resolve_approver("a", &delegations, "2026-06-01").unwrap();
        assert_eq!(result, "c", "should follow chain to final approver");
    }
}
