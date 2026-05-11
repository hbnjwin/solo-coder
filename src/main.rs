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

/// TODO: filter not implemented - returns all records
pub fn filter_records(records: &[ApprovalRecord], _filter: &ExportFilter) -> Vec<ApprovalRecord> {
    records.to_vec()
}

/// TODO: sort not implemented
pub fn sort_records(_records: &mut [ApprovalRecord], _field: &str, _dir: &str) {}

pub fn export_csv(records: &[ApprovalRecord]) -> Result<String> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for rec in records { wtr.serialize(rec)?; }
    Ok(String::from_utf8(wtr.into_inner()?)?)
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
        ApprovalRecord { id: "2".into(), contract_name: "Contract B".into(), amount: 2000000.0, status: "pending".into(), approver: "lisi".into(), create_time: "2026-03-20".into() },
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
