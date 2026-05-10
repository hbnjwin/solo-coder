use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod config;
mod export;
mod health;

use config::{format_error, load_runtime_config, resolve_config_path};
use export::run_query_and_export;
use health::startup_health_check;

#[derive(Parser, Debug)]
#[command(name = "cfg-hotreload", version, about = "Config hot-reload and multi-env support")]
struct Cli {
    #[arg(short, long, default_value = "config.ini")]
    config: PathBuf,
    #[arg(long)]
    env: Option<String>,
    #[arg(long, default_value_t = false)]
    check_only: bool,
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = resolve_config_path(&cli.config, cli.env.as_deref())?;
    println!("Using config: {}", config_path.display());

    let cfg = load_runtime_config(&config_path)?;

    if cli.dry_run {
        println!("Config check passed.");
        return Ok(());
    }

    if cli.check_only {
        let health = startup_health_check(&cfg);
        if health.failed_items.is_empty() {
            println!("Connection check passed: {}/{}", health.ok_count, health.total_count);
            return Ok(());
        }
        return Err(anyhow!("connection check failed: {}/{} passed\n{}",
            health.ok_count, health.total_count, health.failed_items.join("\n")));
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
            match run_query_and_export(ds, query, &output) {
                Ok(result) => {
                    ok_jobs += 1;
                    println!("  - {} OK: {} rows", query.key, result.row_count);
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
