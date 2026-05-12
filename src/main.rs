use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod config;
mod export;
mod health;
mod metrics;
mod metrics_config;

use config::{ConfigManager, format_error, resolve_config_path};
use export::run_query_and_export;
use health::startup_health_check;
use metrics::{calculate_approval_rate, calculate_avg_duration, format_metrics, identify_bottlenecks, load_records, to_json, ApprovalRecord};

#[derive(Parser, Debug)]
#[command(name = "cfg-metrics-fusion", version, about = "Config hot-reload with approval metrics")]
struct Cli {
    #[arg(short, long, default_value = "config.ini")]
    config: PathBuf,
    #[arg(long)]
    env: Option<String>,
    #[arg(long, default_value_t = false)]
    check_only: bool,
    #[arg(long, default_value_t = false)]
    dry_run: bool,
    #[arg(long, default_value_t = false)]
    daemon: bool,
    #[arg(long, default_value_t = 60)]
    interval: u64,
    #[arg(long, default_value_t = false)]
    metrics_only: bool,
}

#[cfg(unix)]
fn setup_signal_handlers(reload_flag: Arc<AtomicBool>, shutdown_flag: Arc<AtomicBool>) -> Result<()> {
    use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
    use signal_hook::flag;
    unsafe {
        flag::register(SIGHUP, reload_flag.clone())?;
        flag::register(SIGINT, shutdown_flag.clone())?;
        flag::register(SIGTERM, shutdown_flag.clone())?;
    }
    Ok(())
}

#[cfg(not(unix))]
fn setup_signal_handlers(_reload_flag: Arc<AtomicBool>, shutdown_flag: Arc<AtomicBool>) -> Result<()> {
    ctrlc::set_handler(move || {
        shutdown_flag.store(true, Ordering::SeqCst);
    }).context("failed to set Ctrl+C handler")?;
    Ok(())
}

fn run_metrics(cfg: &config::RuntimeConfig) -> Result<()> {
    let metrics_cfg = cfg.metrics.as_ref().ok_or_else(|| anyhow!("no [metrics] section in config"))?;
    let records = load_records(&metrics_cfg.data_file)?;

    let mut all_metrics = vec![
        calculate_approval_rate(&records),
        calculate_avg_duration(&records),
    ];

    let bottlenecks = identify_bottlenecks(&records);
    if !bottlenecks.is_empty() {
        println!("Bottleneck approvers: {}", bottlenecks.join(", "));
    }

    for dim in &metrics_cfg.dimensions {
        let groups = metrics::group_by_field(&records, dim);
        for (key, group_records) in &groups {
            let owned: Vec<ApprovalRecord> = group_records.iter().map(|r| (*r).clone()).collect();
            let rate = calculate_approval_rate(&owned);
            all_metrics.push(metrics::MetricResult {
                name: format!("approval_rate_{}_{}", dim, key),
                value: rate.value,
                unit: "%".to_string(),
                sample_size: group_records.len(),
            });
        }
    }

    match metrics_cfg.output_format {
        metrics_config::MetricsOutputFormat::Terminal => print!("{}", format_metrics(&all_metrics)),
        metrics_config::MetricsOutputFormat::Json => println!("{}", to_json(&all_metrics)),
    }

    Ok(())
}

fn run_once(cfg: &config::RuntimeConfig, output: &PathBuf) -> Result<()> {
    let mut failed_jobs = Vec::new();
    let mut total_jobs = 0usize;
    let mut ok_jobs = 0usize;

    for ds in cfg.data_sources.iter().filter(|d| d.enabled) {
        println!("Running source [{}] ({})", ds.section_name, ds.db_type.as_str());
        for query in &ds.queries {
            total_jobs += 1;
            match run_query_and_export(ds, query, output) {
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

fn run_daemon(mut config_manager: ConfigManager, interval: u64, output: PathBuf) -> Result<()> {
    let reload_flag = Arc::new(AtomicBool::new(false));
    let shutdown_flag = Arc::new(AtomicBool::new(false));

    setup_signal_handlers(reload_flag.clone(), shutdown_flag.clone())?;

    let mut last_reload_check = std::time::Instant::now();
    let mut last_metrics_mtime: Option<std::time::SystemTime> = None;

    loop {
        if shutdown_flag.load(Ordering::SeqCst) {
            println!("[daemon] Shutdown signal received, stopping...");
            break;
        }

        if reload_flag.load(Ordering::SeqCst) {
            reload_flag.store(false, Ordering::SeqCst);
            println!("[daemon] SIGHUP received, reloading config...");
            match config_manager.try_reload() {
                Ok(true) => println!("[daemon] Config reloaded successfully."),
                Ok(false) => println!("[daemon] Config reload failed, keeping old config."),
                Err(e) => eprintln!("[daemon] Config reload error: {:#}", e),
            }
        }

        let now = std::time::Instant::now();
        if now.duration_since(last_reload_check) >= std::time::Duration::from_secs(interval) {
            last_reload_check = now;
            if let Err(e) = run_once(config_manager.config(), &output) {
                eprintln!("[daemon] Run failed: {:#}", e);
            }

            if let Some(ref metrics_cfg) = config_manager.config().metrics {
                if let Ok(metadata) = std::fs::metadata(&metrics_cfg.data_file) {
                    let mtime = metadata.modified().ok();
                    if mtime != last_metrics_mtime {
                        last_metrics_mtime = mtime;
                        println!("[daemon] Metrics data file changed, recalculating...");
                        if let Err(e) = run_metrics(config_manager.config()) {
                            eprintln!("[daemon] Metrics calculation failed: {:#}", e);
                        }
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = resolve_config_path(&cli.config, cli.env.as_deref())?;
    println!("Using config: {}", config_path.display());

    let config_manager = ConfigManager::new(&config_path)?;

    if cli.dry_run {
        println!("Config check passed.");
        return Ok(());
    }

    if cli.check_only {
        let health = startup_health_check(config_manager.config());
        if health.failed_items.is_empty() {
            println!("Connection check passed: {}/{}", health.ok_count, health.total_count);
            return Ok(());
        }
        return Err(anyhow!("connection check failed: {}/{} passed\n{}",
            health.ok_count, health.total_count, health.failed_items.join("\n")));
    }

    if cli.metrics_only {
        return run_metrics(config_manager.config());
    }

    let output = PathBuf::from("output");
    fs::create_dir_all(&output)
        .with_context(|| format!("failed to create output directory {}", output.display()))?;

    if cli.daemon {
        run_daemon(config_manager, cli.interval, output)
    } else {
        run_once(config_manager.config(), &output)?;

        if config_manager.config().metrics.is_some() {
            run_metrics(config_manager.config())?;
        }

        Ok(())
    }
}
