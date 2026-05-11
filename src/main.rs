use anyhow::Result;
use clap::Parser;

use approval_csv_export::models::{ExportFormat, ExportRecord, FilterCondition, FilterOp, SortConfig};
use approval_csv_export::filter::apply_filters;
use approval_csv_export::sorter::sort_records;
use approval_csv_export::exporter::export_records;

#[derive(Parser, Debug)]
#[command(name = "approval-csv-export", about = "Export approval records to CSV or JSON")]
struct Cli {
    #[arg(long, default_value = "csv")]
    format: String,

    #[arg(long)]
    status: Option<String>,

    #[arg(long)]
    approver: Option<String>,

    #[arg(long)]
    min_amount: Option<String>,

    #[arg(long)]
    max_amount: Option<String>,

    #[arg(long, default_value = "date:asc")]
    sort: String,
}

fn build_filters(cli: &Cli) -> Vec<FilterCondition> {
    let mut conditions = Vec::new();

    if let Some(ref status) = cli.status {
        conditions.push(FilterCondition {
            field: "status".into(),
            op: FilterOp::Eq,
            value: status.clone(),
        });
    }

    if let Some(ref approver) = cli.approver {
        conditions.push(FilterCondition {
            field: "approver".into(),
            op: FilterOp::Eq,
            value: approver.clone(),
        });
    }

    if let Some(ref min) = cli.min_amount {
        conditions.push(FilterCondition {
            field: "amount".into(),
            op: FilterOp::Gt,
            value: min.clone(),
        });
    }

    if let Some(ref max) = cli.max_amount {
        conditions.push(FilterCondition {
            field: "amount".into(),
            op: FilterOp::Lt,
            value: max.clone(),
        });
    }

    conditions
}

fn sample_data() -> Vec<ExportRecord> {
    vec![
        ExportRecord { id: "AP-001".into(), contract_name: "Acme Corp".into(), amount: 500000.0, status: "approved".into(), approver: "zhang".into(), date: "2026-01-15".into() },
        ExportRecord { id: "AP-002".into(), contract_name: "Beta, Inc.".into(), amount: 2000000.0, status: "pending".into(), approver: "li".into(), date: "2026-03-20".into() },
        ExportRecord { id: "AP-003".into(), contract_name: "Gamma Holdings".into(), amount: 750000.0, status: "approved".into(), approver: "wang".into(), date: "2026-02-10".into() },
        ExportRecord { id: "AP-004".into(), contract_name: "Delta \"Premium\" Services".into(), amount: 120000.0, status: "rejected".into(), approver: "zhang".into(), date: "2026-04-01".into() },
        ExportRecord { id: "AP-005".into(), contract_name: "Epsilon Ltd".into(), amount: 3500000.0, status: "approved".into(), approver: "li".into(), date: "2026-01-28".into() },
    ]
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let records = sample_data();
    let conditions = build_filters(&cli);
    let mut filtered = apply_filters(&records, &conditions);

    let sort_config = SortConfig::parse(&cli.sort)?;
    sort_records(&mut filtered, &sort_config);

    let format = ExportFormat::from_str(&cli.format)?;
    let output = export_records(&filtered, &format)?;
    print!("{}", output);

    Ok(())
}
