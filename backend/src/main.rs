use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String, pub contract_name: String, pub amount: f64,
    pub status: String, pub approver: String, pub create_time: String,
}

/// TODO: implement paginated export with page_token
pub fn export_page(records: &[ApprovalRecord], page: usize, page_size: usize) -> (Vec<ApprovalRecord>, Option<String>) {
    let start = page * page_size;
    if start >= records.len() { return (vec![], None); }
    let end = std::cmp::min(start + page_size, records.len());
    let page_records = records[start..end].to_vec();
    let next_token = if end < records.len() { Some(format!("page_{}", page + 1)) } else { None };
    (page_records, next_token)
}

#[derive(Parser, Debug)]
#[command(name = "export-retry", about = "Export with pagination and retry")]
struct Cli { #[arg(long, default_value = "1000")] page_size: usize }

fn main() -> Result<()> {
    let records: Vec<ApprovalRecord> = (0..5000).map(|i| ApprovalRecord {
        id: format!("{}", i), contract_name: format!("Contract {}", i), amount: i as f64 * 1000.0,
        status: if i % 3 == 0 { "approved".into() } else { "pending".into() },
        approver: "zhangsan".into(), create_time: "2026-01-01".into(),
    }).collect();
    let (page, token) = export_page(&records, 0, 1000);
    println!("page 0: {} records, next_token: {:?}", page.len(), token);
    Ok(())
}
