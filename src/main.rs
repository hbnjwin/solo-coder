use anyhow::Result;
use chrono::Local;
use clap::Parser;

use approval_delegation::models::{Approver, DelegationRule, TimeRange};
use approval_delegation::registry::DelegationRegistry;
use approval_delegation::resolver::resolve_effective_approver;

#[derive(Parser, Debug)]
#[command(name = "approval-delegation", about = "Resolve effective approver through delegation chains")]
struct Cli {
    /// The approver to resolve
    #[arg(long, default_value = "zhangsan")]
    approver: String,

    /// Path to a JSON file containing delegation rules
    #[arg(long)]
    rules_file: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let today = Local::now().date_naive();

    let mut registry = DelegationRegistry::new();

    if let Some(path) = &cli.rules_file {
        let content = std::fs::read_to_string(path)?;
        let rules: Vec<DelegationRule> = serde_json::from_str(&content)?;
        for rule in rules {
            registry.add_rule(rule);
        }
    } else {
        registry.add_rule(DelegationRule::new(
            Approver::new("zhangsan"),
            Approver::new("lisi"),
            TimeRange::new(
                chrono::NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
                chrono::NaiveDate::from_ymd_opt(2026, 5, 31).unwrap(),
            ),
            "vacation coverage",
        ));
    }

    let original = Approver::new(&cli.approver);
    let chain = resolve_effective_approver(&registry, &original, today)?;

    println!("effective approver for {}: {}", chain.original, chain.effective);
    if !chain.hops.is_empty() {
        let hop_ids: Vec<&str> = chain.hops.iter().map(|a| a.id.as_str()).collect();
        println!("  delegation path: {}", hop_ids.join(" -> "));
    }

    Ok(())
}
