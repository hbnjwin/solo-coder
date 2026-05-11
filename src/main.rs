use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String, pub contract_name: String, pub amount: f64,
    pub status: String, pub approver: String, pub create_time: String,
}

pub struct ExportFilter {
    pub status: Option<String>, pub start_date: Option<String>,
    pub end_date: Option<String>, pub approver: Option<String>,
}

/// BUG: filter not implemented - returns all records
pub fn filter_records(records: &[ApprovalRecord], _filter: &ExportFilter) -> Vec<ApprovalRecord> {
    records.to_vec()
}

/// BUG: sort not implemented
pub fn sort_records(_records: &mut [ApprovalRecord], _field: &str, _dir: &str) {}

/// BUG: CSV export doesn't escape commas, quotes, or newlines in field values
pub fn export_csv(records: &[ApprovalRecord]) -> Result<String> {
    let mut lines = Vec::new();
    lines.push("id,contract_name,amount,status,approver,create_time".to_string());
    for rec in records {
        // BUG: no escaping - commas/quotes/newlines in values break CSV format
        lines.push(format!("{},{},{},{},{},{}", rec.id, rec.contract_name, rec.amount, rec.status, rec.approver, rec.create_time));
    }
    Ok(lines.join("\n"))
}

pub fn export_json(records: &[ApprovalRecord]) -> Result<String> {
    Ok(serde_json::to_string_pretty(records)?)
}

#[derive(Parser, Debug)]
#[command(name = "approval-csv-export", about = "Export approval data to CSV/JSON")]
struct Cli {
    #[arg(long, default_value = "csv")] format: String,
    #[arg(long)] status: Option<String>,
    #[arg(long)] start_date: Option<String>,
    #[arg(long)] end_date: Option<String>,
    #[arg(long)] approver: Option<String>,
    #[arg(long, default_value = "create_time:asc")] sort_by: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let records = vec![
        ApprovalRecord { id: "1".into(), contract_name: "Contract A".into(), amount: 500000.0, status: "approved".into(), approver: "zhangsan".into(), create_time: "2026-01-15".into() },
        ApprovalRecord { id: "2".into(), contract_name: "Contract B, Inc.".into(), amount: 2000000.0, status: "pending".into(), approver: "lisi".into(), create_time: "2026-03-20".into() },
    ];
    let filter = ExportFilter { status: cli.status, start_date: cli.start_date, end_date: cli.end_date, approver: cli.approver };
    let filtered = filter_records(&records, &filter);
    match cli.format.as_str() {
        "csv" => println!("{}", export_csv(&filtered)?),
        "json" => println!("{}", export_json(&filtered)?),
        _ => anyhow::bail!("unsupported format"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_escape_comma() {
        let records = vec![ApprovalRecord { id: "1".into(), contract_name: "A, B Corp".into(), amount: 100.0, status: "ok".into(), approver: "zhang".into(), create_time: "2026-01-01".into() }];
        let csv = export_csv(&records).unwrap();
        // Value with comma must be quoted
        assert!(csv.contains("\"A, B Corp\""), "CSV should escape commas with quotes");
    }

    #[test]
    fn test_csv_escape_quote() {
        let records = vec![ApprovalRecord { id: "1".into(), contract_name: "A \"Premium\" Deal".into(), amount: 100.0, status: "ok".into(), approver: "zhang".into(), create_time: "2026-01-01".into() }];
        let csv = export_csv(&records).unwrap();
        // Internal quotes must be doubled
        assert!(csv.contains("\"A \"\"Premium\"\" Deal\""), "CSV should escape internal quotes by doubling");
    }

    #[test]
    fn test_filter_by_status() {
        let records = vec![
            ApprovalRecord { id: "1".into(), contract_name: "A".into(), amount: 100.0, status: "approved".into(), approver: "z".into(), create_time: "2026-01-01".into() },
            ApprovalRecord { id: "2".into(), contract_name: "B".into(), amount: 200.0, status: "pending".into(), approver: "z".into(), create_time: "2026-01-02".into() },
        ];
        let filter = ExportFilter { status: Some("approved".into()), start_date: None, end_date: None, approver: None };
        let filtered = filter_records(&records, &filter);
        assert_eq!(filtered.len(), 1, "should filter to only approved records");
        assert_eq!(filtered[0].id, "1");
    }
}
