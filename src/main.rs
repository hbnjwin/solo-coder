use anyhow::{anyhow, Context, Result};
use chrono::Local;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod config;
mod export;
mod transport;

use config::{format_error, load_runtime_config};
use export::run_query_and_export;
use transport::{send_report, send_transport_test_packet, QueryReport};

#[derive(Parser, Debug)]
#[command(name = "tcp-reconnect", version, about = "TCP report with auto-reconnect")]
struct Cli {
    #[arg(short, long, default_value = "config.ini")]
    config: PathBuf,
    #[arg(long, default_value_t = false)]
    send_test_packet: bool,
    #[arg(long, default_value_t = false)]
    check_only: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = load_runtime_config(&cli.config)?;

    if cli.send_test_packet {
        return send_transport_test_packet(&cfg);
    }

    if cli.check_only {
        println!("Config check passed.");
        return Ok(());
    }

    let output = PathBuf::from("output");
    fs::create_dir_all(&output)
        .with_context(|| format!("failed to create output directory {}", output.display()))?;

    let mut failed_jobs = Vec::new();
    let mut total_jobs = 0usize;
    let mut ok_jobs = 0usize;

    for ds in cfg.data_sources.iter().filter(|d| d.enabled) {
        println!("Running source [{}] ({})", ds.section_name, ds.db_type.as_str());
        for query in &ds.queries {
            total_jobs += 1;
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            match run_query_and_export(ds, query, &output) {
                Ok(result) => {
                    ok_jobs += 1;
                    println!("  - {} OK: {} rows", query.key, result.row_count);
                    let report = QueryReport {
                        timestamp: now,
                        source: ds.section_name.clone(),
                        db_type: ds.db_type.as_str().to_string(),
                        query_key: query.key.clone(),
                        row_count: result.row_count,
                        csv_file: Some(result.csv_file.to_string_lossy().to_string()),
                        json_file: Some(result.json_file.to_string_lossy().to_string()),
                        success: true,
                        error: None,
                    };
                    send_report(&cfg, &report);
                }
                Err(err) => {
                    let msg = format_error(&err);
                    eprintln!("  - {} FAILED: {}", query.key, msg);
                    failed_jobs.push(format!("{}::{} => {}", ds.section_name, query.key, msg));
                }
            }
        }
    }

    println!("Finished: {ok_jobs}/{total_jobs} jobs succeeded.");
    if failed_jobs.is_empty() { return Ok(()); }
    Err(anyhow!("{} jobs failed:\n{}", failed_jobs.len(), failed_jobs.join("\n")))
}
