use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;

use report_validator::chain::evaluate_chains;
use report_validator::evaluator::validate_record;
use report_validator::models::*;
use report_validator::reporter::ValidationReport;

#[derive(Parser, Debug)]
#[command(name = "report-validator", about = "Report validation rule engine")]
struct Cli {
    #[arg(long, default_value = "validate")]
    action: String,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    let rules = vec![
        ValidationRule::new("r1", "amount", Operator::Gt, serde_json::json!(0)),
        ValidationRule::new("r2", "title", Operator::NotEmpty, serde_json::json!("")),
        ValidationRule::new("r3", "amount", Operator::Lte, serde_json::json!(10000000)),
    ];

    let chains = vec![
        RuleChain::new("c1", ChainMode::And)
            .add_rule(ValidationRule::new("c1-r1", "amount", Operator::Gte, serde_json::json!(100)))
            .add_rule(ValidationRule::new("c1-r2", "amount", Operator::Lte, serde_json::json!(1000000))),
    ];

    let mut record: Record = HashMap::new();
    record.insert("amount".into(), serde_json::json!(5000));
    record.insert("title".into(), serde_json::json!("Q1 Report"));

    let results = validate_record(&rules, &record);
    let chain_results = evaluate_chains(&chains, &record);
    let report = ValidationReport::from_results(results, chain_results);

    println!("{}", report.summary());
    if report.is_valid() {
        println!("Record is valid.");
    } else {
        println!("Record has validation errors.");
        std::process::exit(1);
    }

    Ok(())
}
