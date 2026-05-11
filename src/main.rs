use std::fs;
use std::process;

use anyhow::Result;
use clap::Parser;
use serde_json::Value;

use audit_trail::differ::compute_diff;
use audit_trail::formatter::format_changes;
use audit_trail::models::DiffOptions;

#[derive(Parser, Debug)]
#[command(name = "audit-trail", about = "Compute field-level diffs between JSON documents")]
struct Cli {
    /// Path to the original JSON file
    #[arg(short, long)]
    old: String,

    /// Path to the updated JSON file
    #[arg(short, long)]
    new: String,

    /// Output format: text, json, table
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let old_content = fs::read_to_string(&cli.old).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", cli.old, e);
        process::exit(1);
    });
    let new_content = fs::read_to_string(&cli.new).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", cli.new, e);
        process::exit(1);
    });

    let old_val: Value = serde_json::from_str(&old_content)?;
    let new_val: Value = serde_json::from_str(&new_content)?;

    let options = DiffOptions::default();
    let changes = compute_diff(&old_val, &new_val, &options);

    let output = match cli.format.as_str() {
        "json" => audit_trail::formatter::to_json(&changes)?,
        "table" => audit_trail::formatter::to_table(&changes),
        _ => format_changes(&changes),
    };

    println!("{}", output);
    Ok(())
}
