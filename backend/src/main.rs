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
    pub urgent: String,     // BUG: backend expects bool, frontend sends string "true"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub id: String,
    pub status: String,
    pub message: String,
}

/// BUG: field names don't match frontend expectations
pub fn handle_approval(req: &ApprovalRequest) -> Result<ApprovalResponse> {
    // BUG: urgent is String but should be bool
    let _is_urgent = req.urgent == "true";
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
