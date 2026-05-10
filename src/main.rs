use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod config;
mod export;
mod incremental;

use config::{format_error, load_runtime_config};
use export::run_query_and_export;
use incremental::{apply_incremental_sql, update_last_run_at};

#[derive(Parser, Debug)]
#[command(name = "sql-incremental", version, about = "SQL query with incremental export")]
struct Cli {
    #[arg(short, long, default_value = "config.ini")]
    config: PathBuf,
    #[arg(short, long, default_value = "output")]
    output: PathBuf,
    #[arg(long, default_value_t = false)]
    incremental: bool,
    #[arg(long, default_value_t = false)]
    check_only: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut cfg = load_runtime_config(&cli.config)?;

    if cli.check_only {
        println!("Config check passed. {} data sources loaded.", cfg.data_sources.len());
        return Ok(());
    }

    if cli.incremental {
        apply_incremental_sql(&mut cfg)?;
    }

    fs::create_dir_all(&cli.output)
        .with_context(|| format!("failed to create output directory {}", cli.output.display()))?;

    let mut failed_jobs = Vec::new();
    let mut total_jobs = 0usize;
    let mut ok_jobs = 0usize;

    for ds in cfg.data_sources.iter().filter(|d| d.enabled) {
        println!("Running source [{}] ({})", ds.section_name, ds.db_type.as_str());
        for query in &ds.queries {
            total_jobs += 1;
            match run_query_and_export(ds, query, &cli.output) {
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

    // After successful run, update last_run_at in config
    if cli.incremental && ok_jobs > 0 {
        update_last_run_at(&cli.config)?;
    }

    println!("Finished: {ok_jobs}/{total_jobs} jobs succeeded.");
    if failed_jobs.is_empty() { return Ok(()); }
    Err(anyhow!("{} jobs failed:\n{}", failed_jobs.len(), failed_jobs.join("\n")))
}
