use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub contract_name: String,
    pub sheet_amount: f64,  // BUG: backend expects snake_case "sheet_amount"
    pub approver: String,
    pub date: String,       // BUG: backend expects YYYY-MM-DD, frontend sends ISO
    pub urgent: String,     // BUG: should be bool but is String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub id: String,
    pub status: String,
    pub message: String,
}

/// BUG: urgent is String but should be bool, uses == "true" hack
pub fn handle_approval(req: &ApprovalRequest) -> Result<ApprovalResponse> {
    let _is_urgent = req.urgent == "true"; // BUG: fragile string comparison
    // BUG: date parsing assumes YYYY-MM-DD but frontend sends ISO format
    Ok(ApprovalResponse { id: "1".into(), status: "created".into(), message: "ok".into() })
}

#[derive(Parser, Debug)]
#[command(name = "api-contract-backend", about = "Approval API backend")]
struct Cli {}

fn main() -> Result<()> {
    let req = ApprovalRequest {
        contract_name: "Contract A".into(), sheet_amount: 500000.0,
        approver: "zhangsan".into(), date: "2026-05-11T10:00:00Z".into(), urgent: "true".into(),
    };
    let resp = handle_approval(&req)?;
    println!("{}", serde_json::to_string_pretty(&resp)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urgent_should_be_bool() {
        let req = ApprovalRequest {
            contract_name: "test".into(), sheet_amount: 100.0,
            approver: "a".into(), date: "2026-01-01".into(), urgent: "true".into(),
        };
        // BUG: urgent field is String, should be bool
        // After fix: req.urgent should be true (bool), not "true" (String)
        assert!(req.urgent == "true", "urgent should be bool type, not string comparison");
    }

    #[test]
    fn test_date_format_mismatch() {
        // Frontend sends ISO: 2026-05-11T10:00:00Z
        // Backend expects: 2026-05-11
        let req = ApprovalRequest {
            contract_name: "test".into(), sheet_amount: 100.0,
            approver: "a".into(), date: "2026-05-11T10:00:00Z".into(), urgent: "true".into(),
        };
        // BUG: date contains T which means ISO format, backend should normalize
        assert!(!req.date.contains('T'), "date should be normalized to YYYY-MM-DD format");
    }
}
