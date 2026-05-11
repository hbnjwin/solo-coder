use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String, pub contract_name: String, pub amount: f64,
    pub status: String, pub approver: String, pub create_time: String,
}

/// BUG: uses page number instead of cursor, causing data drift when new records are inserted during export
pub fn export_page(records: &[ApprovalRecord], page: usize, page_size: usize) -> (Vec<ApprovalRecord>, Option<String>) {
    let start = page * page_size;
    if start >= records.len() { return (vec![], None); }
    let end = std::cmp::min(start + page_size, records.len());
    let page_records = records[start..end].to_vec();
    // BUG: page_token is just "page_N" which breaks if data changes between pages
    let next_token = if end < records.len() { Some(format!("page_{}", page + 1)) } else { None };
    (page_records, next_token)
}

/// TODO: should use cursor-based pagination (last record ID as token)
pub fn export_cursor(records: &[ApprovalRecord], cursor: Option<&str>, page_size: usize) -> (Vec<ApprovalRecord>, Option<String>) {
    // TODO: implement cursor-based pagination
    (vec![], None)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_based_drift() {
        // Simulate: page 0 returns records 0-9, then a new record is inserted at position 0
        // Page 1 would return records 10-19 shifted by 1, causing record 10 to appear twice
        // This test documents the bug
        let records: Vec<ApprovalRecord> = (0..20).map(|i| ApprovalRecord {
            id: format!("{}", i), contract_name: format!("C{}", i), amount: 100.0,
            status: "ok".into(), approver: "z".into(), create_time: "2026-01-01".into(),
        }).collect();
        let (page0, _) = export_page(&records, 0, 10);
        let (page1, _) = export_page(&records, 1, 10);
        // Check for overlap
        let ids0: Vec<_> = page0.iter().map(|r| r.id.clone()).collect();
        let ids1: Vec<_> = page1.iter().map(|r| r.id.clone()).collect();
        let overlap: Vec<_> = ids0.iter().filter(|id| ids1.contains(id)).collect();
        assert!(overlap.is_empty(), "page-based pagination should not have overlap but does: {:?}", overlap);
    }

    #[test]
    fn test_cursor_based_not_implemented() {
        let records: Vec<ApprovalRecord> = (0..20).map(|i| ApprovalRecord {
            id: format!("{}", i), contract_name: format!("C{}", i), amount: 100.0,
            status: "ok".into(), approver: "z".into(), create_time: "2026-01-01".into(),
        }).collect();
        let (page, token) = export_cursor(&records, None, 10);
        assert!(!page.is_empty(), "cursor-based export should return records");
        assert!(token.is_some(), "cursor-based export should return next cursor");
    }
}
