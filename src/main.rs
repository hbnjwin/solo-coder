use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::process;

use config_validator::parser::parse_config;
use config_validator::validator::validate_config;

#[derive(Parser)]
#[command(name = "config-validator")]
#[command(about = "Validates workflow configuration files")]
struct Cli {
    #[arg(help = "Path to the configuration file")]
    config: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = parse_config(&cli.config)?;
    let errors = validate_config(&config);

    if errors.is_empty() {
        println!("Configuration is valid.");
    } else {
        eprintln!("Validation failed with {} error(s):", errors.len());
        for err in &errors {
            eprintln!("  - {}", err);
        }
        process::exit(1);
    }

    Ok(())
}
