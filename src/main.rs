use anyhow::Result;
use clap::Parser;
use std::fs;

use approval_metrics::calculator;
use approval_metrics::analyzer;
use approval_metrics::models::ApprovalRecord;
use approval_metrics::output;

#[derive(Parser, Debug)]
#[command(name = "approval-metrics", about = "Analyze approval workflow metrics")]
struct Cli {
    /// Path to the JSON file containing approval records
    #[arg(short, long)]
    input: String,

    /// Output format: terminal, csv, json
    #[arg(short, long, default_value = "terminal")]
    format: String,

    /// Field to group results by (department, approver, approval_type)
    #[arg(short, long)]
    group_by: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let content = fs::read_to_string(&cli.input)?;
    let records: Vec<ApprovalRecord> = serde_json::from_str(&content)?;

    let mut metrics = vec![
        calculator::calculate_approval_rate(&records),
        calculator::calculate_avg_duration(&records),
    ];

    let bottlenecks = analyzer::identify_bottlenecks(&records);
    if !bottlenecks.is_empty() {
        println!("Bottleneck approvers: {}", bottlenecks.join(", "));
    }

    if let Some(ref field) = cli.group_by {
        let groups = analyzer::group_by_field(&records, field);
        for (key, group_records) in &groups {
            let rate = calculator::calculate_approval_rate(group_records);
            metrics.push(approval_metrics::models::MetricResult {
                name: format!("approval_rate_{}", key),
                value: rate.value,
                unit: "%".to_string(),
                sample_size: group_records.len(),
            });
        }
    }

    match cli.format.as_str() {
        "csv" => print!("{}", output::to_csv(&metrics)),
        "json" => println!("{}", output::to_json(&metrics)),
        _ => print!("{}", output::format_metrics(&metrics)),
    }

    Ok(())
}
