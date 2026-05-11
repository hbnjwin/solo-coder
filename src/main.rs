use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use deal_content_parser::formatter::{format_deal, summarize, FormatOptions};
use deal_content_parser::parser::parse_deal;

#[derive(Parser, Debug)]
#[command(name = "deal-content-parser")]
#[command(about = "Parse and display DealContent JSON structures")]
struct Cli {
    /// Path to a JSON file to parse. Reads from stdin if not provided.
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Use compact output format
    #[arg(short, long, default_value_t = false)]
    compact: bool,

    /// Show only the summary line
    #[arg(short, long, default_value_t = false)]
    summary: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input = match cli.file {
        Some(path) => {
            std::fs::read_to_string(&path)
                .with_context(|| format!("failed to read file: {}", path.display()))?
        }
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read from stdin")?;
            buf
        }
    };

    let value: serde_json::Value =
        serde_json::from_str(&input).context("invalid JSON input")?;

    let content = parse_deal(&value)?;

    if cli.summary {
        println!("{}", summarize(&content));
    } else {
        let opts = FormatOptions {
            compact: cli.compact,
            ..Default::default()
        };
        print!("{}", format_deal(&content, &opts));
    }

    Ok(())
}
