use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

mod config;
mod import;
mod sql_util;

use config::{format_error, load_runtime_config};
use import::{discover_csv_files, import_one_query_from_csv};

#[derive(Parser, Debug)]
#[command(name = "csv-import-robust", version, about = "CSV batch import with robustness")]
struct Cli {
    #[arg(short, long, default_value = "config.ini")]
    config: PathBuf,
    #[arg(long)]
    import_csv_dir: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    check_only: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = load_runtime_config(&cli.config)?;

    if cli.check_only {
        println!("Config check passed. {} data sources loaded.", cfg.data_sources.len());
        return Ok(());
    }

    let Some(import_dir) = &cli.import_csv_dir else {
        return Err(anyhow!("--import-csv-dir is required for import mode"));
    };

    run_batch_csv_import(&cfg, import_dir)
}

fn run_batch_csv_import(cfg: &config::RuntimeConfig, import_dir: &Path) -> Result<()> {
    let csv_files = discover_csv_files(import_dir)?;
    if csv_files.is_empty() {
        return Err(anyhow!("no csv file found in {}", import_dir.display()));
    }
    println!("Import mode: found {} csv files in {}", csv_files.len(), import_dir.display());

    let mut total_jobs = 0usize;
    let mut ok_jobs = 0usize;
    let mut errors = Vec::new();

    for ds in cfg.data_sources.iter().filter(|d| d.enabled) {
        println!("Import target [{}] ({})", ds.section_name, ds.db_type.as_str());
        for query in &ds.queries {
            total_jobs += 1;
            match import_one_query_from_csv(ds, query, &csv_files) {
                Ok(out) => {
                    ok_jobs += 1;
                    println!(
                        "  - {} OK: file={} total={} inserted={} skipped={}",
                        query.key, out.file.display(), out.total_rows, out.inserted_rows, out.skipped_rows
                    );
                }
                Err(err) => {
                    let msg = format!("{}::{} => {}", ds.section_name, query.key, format_error(&err));
                    eprintln!("  - {} FAILED: {}", query.key, format_error(&err));
                    errors.push(msg);
                }
            }
        }
    }

    println!("Import finished: {ok_jobs}/{total_jobs} jobs succeeded.");
    if errors.is_empty() { return Ok(()); }
    Err(anyhow!("{} import jobs failed:\n{}", errors.len(), errors.join("\n")))
}
