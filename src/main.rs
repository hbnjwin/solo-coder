use anyhow::{anyhow, Context, Result};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use clap::Parser;
use configparser::ini::Ini;
use csv::{ReaderBuilder, WriterBuilder};
use mysql::{prelude::Queryable, OptsBuilder, Pool, Value as MyValue};
use postgres::{Client, NoTls, Row};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(
    name = "data-import-sql",
    version,
    about = "SSH-friendly data query/import tool for MySQL/PostgreSQL"
)]
struct Cli {
    #[arg(short, long, default_value = "config.ini")]
    config: PathBuf,
    #[arg(short, long, default_value = "output")]
    output: PathBuf,
    #[arg(long, default_value_t = false)]
    serve: bool,
    #[arg(long)]
    tcp_listen: Option<String>,
    #[arg(long)]
    udp_listen: Option<String>,
    #[arg(long, default_value_t = false)]
    send_test_packet: bool,
    #[arg(long, default_value_t = false)]
    send_query_json: bool,
    #[arg(long, default_value = "server_output")]
    server_save_dir: PathBuf,
    #[arg(long)]
    import_csv_dir: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    check_only: bool,
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

#[derive(Debug, Clone)]
struct Endpoint {
    enabled: bool,
    host: String,
    port: u16,
}

impl Endpoint {
    fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone)]
enum DbType {
    PostgreSql,
    MySql,
}

impl DbType {
    fn as_str(&self) -> &'static str {
        match self {
            DbType::PostgreSql => "postgres",
            DbType::MySql => "mysql",
        }
    }
}

#[derive(Debug, Clone)]
struct QueryJob {
    key: String,
    sql: String,
}

#[derive(Debug, Clone)]
struct DataSource {
    section_name: String,
    enabled: bool,
    db_type: DbType,
    host: String,
    port: u16,
    user: String,
    password: String,
    database: Option<String>,
    queries: Vec<QueryJob>,
}

#[derive(Debug)]
struct RuntimeConfig {
    tcp: Option<Endpoint>,
    udp: Option<Endpoint>,
    udp_chunk_size: usize,
    data_sources: Vec<DataSource>,
}

#[derive(Debug, Serialize)]
struct QueryReport {
    timestamp: String,
    source: String,
    db_type: String,
    query_key: String,
    row_count: usize,
    csv_file: Option<String>,
    json_file: Option<String>,
    success: bool,
    error: Option<String>,
}

#[derive(Debug, Clone)]
struct ExportResult {
    row_count: usize,
    csv_file: PathBuf,
    json_file: PathBuf,
}

#[derive(Debug, Serialize)]
struct ServerReceiveRecord {
    timestamp: String,
    protocol: String,
    peer: String,
    bytes: usize,
    payload: String,
}

#[derive(Debug, Serialize)]
struct QueryDataPacket {
    kind: String,
    timestamp: String,
    source: String,
    db_type: String,
    query_key: String,
    row_count: usize,
    data: JsonValue,
}

const UDP_DIRECT_MAX_BYTES: usize = 1200;
const UDP_CHUNK_DATA_DEFAULT_BYTES: usize = 512;
const UDP_CHUNK_DATA_MIN_BYTES: usize = 64;
const UDP_CHUNK_DATA_MAX_BYTES: usize = 1024;
// Keep chunk data conservative because it is wrapped into JSON and escaped.
// The serialized datagram can grow far beyond `data.len()`.
const UDP_CHUNK_MAX_PARTS: usize = 50_000;
const UDP_CHUNK_KIND: &str = "udp_chunk_v1";
const UDP_CHUNK_TTL_SECS: u64 = 120;
const UDP_CHUNK_SEND_PAUSE_EVERY: usize = 8;
const UDP_CHUNK_SEND_PAUSE_MS: u64 = 10;
const UDP_SERVER_RECV_TIMEOUT_SECS: u64 = 1;
const UDP_CHUNK_PROGRESS_LOG_EVERY: usize = 500;
const UDP_SEND_RETRY_TIMES: usize = 3;
const UDP_SEND_RETRY_MS: u64 = 2;
static UDP_MSG_SEQ: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Serialize)]
struct UdpChunkPacketRef<'a> {
    kind: &'static str,
    msg_id: &'a str,
    idx: usize,
    total: usize,
    data: &'a str,
}

#[derive(Debug, Deserialize)]
struct UdpChunkPacketOwned {
    kind: String,
    msg_id: String,
    idx: usize,
    total: usize,
    data: String,
}

#[derive(Debug)]
struct UdpChunkAssembly {
    total: usize,
    received: usize,
    parts: Vec<Option<String>>,
    updated_at: Instant,
}

#[derive(Debug, Default)]
struct UdpChunkMetrics {
    reassembled_ok: usize,
    invalid_chunk: usize,
    stale_drop: usize,
}

#[derive(Debug, Serialize)]
struct UdpChunkMetricRecord {
    timestamp: String,
    event: String,
    msg_key: String,
    total: usize,
    received: usize,
    reassembled_ok: usize,
    invalid_chunk: usize,
    stale_drop: usize,
    error: Option<String>,
}

#[derive(Debug)]
struct ExpiredUdpChunk {
    msg_key: String,
    total: usize,
    received: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.serve {
        let mut tcp_listen = cli.tcp_listen.clone();
        let mut udp_listen = cli.udp_listen.clone();
        if tcp_listen.is_none() && udp_listen.is_none() {
            let runtime_cfg = load_runtime_config(&cli.config)?;
            tcp_listen = runtime_cfg
                .tcp
                .as_ref()
                .filter(|ep| ep.enabled)
                .map(|ep| ep.addr());
            udp_listen = runtime_cfg
                .udp
                .as_ref()
                .filter(|ep| ep.enabled)
                .map(|ep| ep.addr());
        }
        return run_debug_servers(
            tcp_listen.as_deref(),
            udp_listen.as_deref(),
            &cli.server_save_dir,
        );
    }

    let runtime_cfg = load_runtime_config(&cli.config)?;

    if cli.send_test_packet {
        return send_transport_test_packet(&runtime_cfg);
    }

    if cli.dry_run {
        println!("Config check passed.");
        print_enabled_summary(&runtime_cfg);
        return Ok(());
    }

    let health = startup_health_check(&runtime_cfg);
    if cli.check_only {
        if health.failed_items.is_empty() {
            println!(
                "Connection check passed: {}/{}",
                health.ok_count, health.total_count
            );
            return Ok(());
        }
        return Err(anyhow!(
            "connection check failed: {}/{} passed\n{}",
            health.ok_count,
            health.total_count,
            health.failed_items.join("\n")
        ));
    }

    if let Some(import_dir) = &cli.import_csv_dir {
        return run_batch_csv_import(&runtime_cfg, import_dir);
    }

    fs::create_dir_all(&cli.output)
        .with_context(|| format!("failed to create output directory {}", cli.output.display()))?;

    let mut failed_jobs = Vec::new();
    let mut total_jobs = 0usize;
    let mut ok_jobs = 0usize;

    for ds in runtime_cfg.data_sources.iter().filter(|d| d.enabled) {
        println!(
            "Running source [{}] ({})",
            ds.section_name,
            ds.db_type.as_str()
        );

        for query in &ds.queries {
            total_jobs += 1;
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            match run_query_and_export(ds, query, &cli.output) {
                Ok(result) => {
                    ok_jobs += 1;
                    println!(
                        "  - {} OK: {} rows -> csv: {} | json: {}",
                        query.key,
                        result.row_count,
                        result.csv_file.display(),
                        result.json_file.display()
                    );
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
                    send_report(&runtime_cfg, &report);
                    if cli.send_query_json {
                        if let Err(err) = send_query_json_payload(&runtime_cfg, ds, query, &result)
                        {
                            eprintln!(
                                "  - {} JSON FORWARD FAILED: {}",
                                query.key,
                                format_error(&err)
                            );
                        } else {
                            println!("  - {} JSON FORWARD OK", query.key);
                        }
                    }
                }
                Err(err) => {
                    let msg = format_error(&err);
                    eprintln!("  - {} FAILED: {}", query.key, msg);
                    failed_jobs.push(format!("{}::{} => {}", ds.section_name, query.key, msg));
                    let report = QueryReport {
                        timestamp: now,
                        source: ds.section_name.clone(),
                        db_type: ds.db_type.as_str().to_string(),
                        query_key: query.key.clone(),
                        row_count: 0,
                        csv_file: None,
                        json_file: None,
                        success: false,
                        error: Some(msg),
                    };
                    send_report(&runtime_cfg, &report);
                }
            }
        }
    }

    println!("Finished: {ok_jobs}/{total_jobs} jobs succeeded.");
    if failed_jobs.is_empty() {
        return Ok(());
    }

    Err(anyhow!(
        "{} jobs failed:\n{}",
        failed_jobs.len(),
        failed_jobs.join("\n")
    ))
}

#[derive(Debug, Default)]
struct HealthCheckSummary {
    total_count: usize,
    ok_count: usize,
    failed_items: Vec<String>,
}

fn print_enabled_summary(cfg: &RuntimeConfig) {
    println!(
        "Enabled data sources: {}",
        cfg.data_sources.iter().filter(|d| d.enabled).count()
    );
    println!(
        "UDP chunk size: {} bytes (config [paraset].chunkSize, default {})",
        cfg.udp_chunk_size, UDP_CHUNK_DATA_DEFAULT_BYTES
    );
}

fn startup_health_check(cfg: &RuntimeConfig) -> HealthCheckSummary {
    println!("=== Startup Status ===");
    let mut summary = HealthCheckSummary::default();

    print_endpoint_health(
        "tcpserver",
        cfg.tcp.as_ref(),
        &mut summary,
        check_tcp_endpoint,
    );
    print_endpoint_health(
        "udpserver",
        cfg.udp.as_ref(),
        &mut summary,
        check_udp_endpoint,
    );

    for ds in &cfg.data_sources {
        if !ds.enabled {
            println!("[{}] enable=0 status=SKIPPED", ds.section_name);
            continue;
        }

        summary.total_count += 1;
        let result = check_database_connection(ds);
        match result {
            Ok(_) => {
                summary.ok_count += 1;
                println!(
                    "[{}] enable=1 type={} target={}:{} status=OK",
                    ds.section_name,
                    ds.db_type.as_str(),
                    ds.host,
                    ds.port
                );
            }
            Err(err) => {
                let msg = format!(
                    "[{}] enable=1 type={} target={}:{} status=FAILED error={}",
                    ds.section_name,
                    ds.db_type.as_str(),
                    ds.host,
                    ds.port,
                    format_error(&err)
                );
                println!("{msg}");
                summary.failed_items.push(msg);
            }
        }
    }

    println!(
        "Startup status summary: {}/{} passed",
        summary.ok_count, summary.total_count
    );
    summary
}

fn print_endpoint_health<F>(
    section_name: &str,
    endpoint: Option<&Endpoint>,
    summary: &mut HealthCheckSummary,
    checker: F,
) where
    F: Fn(&Endpoint) -> Result<()>,
{
    let Some(ep) = endpoint else {
        println!("[{}] enable=0 status=NOT_CONFIGURED", section_name);
        return;
    };

    if !ep.enabled {
        println!("[{}] enable=0 status=SKIPPED", section_name);
        return;
    }

    summary.total_count += 1;
    match checker(ep) {
        Ok(_) => {
            summary.ok_count += 1;
            println!("[{}] enable=1 target={} status=OK", section_name, ep.addr());
        }
        Err(err) => {
            let msg = format!(
                "[{}] enable=1 target={} status=FAILED error={}",
                section_name,
                ep.addr(),
                format_error(&err)
            );
            println!("{msg}");
            summary.failed_items.push(msg);
        }
    }
}

fn run_debug_servers(
    tcp_listen: Option<&str>,
    udp_listen: Option<&str>,
    save_dir: &Path,
) -> Result<()> {
    if tcp_listen.is_none() && udp_listen.is_none() {
        return Err(anyhow!(
            "serve mode requires at least one address: --tcp-listen <host:port> or --udp-listen <host:port>"
        ));
    }
    fs::create_dir_all(save_dir)
        .with_context(|| format!("failed to create server save dir {}", save_dir.display()))?;
    let tcp_log = save_dir.join("tcp_received.ndjson");
    let udp_log = save_dir.join("udp_received.ndjson");
    let udp_chunk_metrics_log = save_dir.join("udp_chunk_metrics.ndjson");

    match (tcp_listen, udp_listen) {
        (Some(tcp_addr), Some(udp_addr)) => {
            let listener = TcpListener::bind(tcp_addr)
                .with_context(|| format!("tcp server bind failed: {}", tcp_addr))?;
            let udp_sock = UdpSocket::bind(udp_addr)
                .with_context(|| format!("udp server bind failed: {}", udp_addr))?;
            println!("Debug server mode started. Press Ctrl+C to stop.");
            println!("[tcp server] listening on {}", tcp_addr);
            println!("[udp server] listening on {}", udp_addr);
            println!("[tcp server] save file {}", tcp_log.display());
            println!("[udp server] save file {}", udp_log.display());
            println!(
                "[udp server] chunk metrics file {}",
                udp_chunk_metrics_log.display()
            );

            let tcp_log_clone = tcp_log.clone();
            thread::spawn(move || run_tcp_server_loop(listener, &tcp_log_clone));
            run_udp_server_loop(udp_sock, &udp_log)?;
        }
        (Some(tcp_addr), None) => {
            let listener = TcpListener::bind(tcp_addr)
                .with_context(|| format!("tcp server bind failed: {}", tcp_addr))?;
            println!("Debug server mode started. Press Ctrl+C to stop.");
            println!("[tcp server] listening on {}", tcp_addr);
            println!("[tcp server] save file {}", tcp_log.display());
            run_tcp_server_loop(listener, &tcp_log)?;
        }
        (None, Some(udp_addr)) => {
            let udp_sock = UdpSocket::bind(udp_addr)
                .with_context(|| format!("udp server bind failed: {}", udp_addr))?;
            println!("Debug server mode started. Press Ctrl+C to stop.");
            println!("[udp server] listening on {}", udp_addr);
            println!("[udp server] save file {}", udp_log.display());
            println!(
                "[udp server] chunk metrics file {}",
                udp_chunk_metrics_log.display()
            );
            run_udp_server_loop(udp_sock, &udp_log)?;
        }
        (None, None) => {}
    }

    Ok(())
}

fn run_tcp_server_loop(listener: TcpListener, save_path: &Path) -> Result<()> {
    loop {
        let (stream, peer_addr) = listener.accept().context("tcp accept failed")?;
        println!("[tcp server] client connected: {}", peer_addr);

        let mut reader = BufReader::new(stream);
        loop {
            let mut raw_line = String::new();
            let n = reader.read_line(&mut raw_line).context("tcp read failed")?;
            if n == 0 {
                break;
            }
            let payload = raw_line.trim_end().to_string();
            if payload.is_empty() {
                continue;
            }
            println!(
                "[tcp server] from={} bytes={} payload={}",
                peer_addr, n, payload
            );
            if let Err(err) =
                append_receive_record(save_path, "tcp", &peer_addr.to_string(), n, &payload)
            {
                eprintln!("[tcp server] save failed: {}", format_error(&err));
            }
            if let Some(parent) = save_path.parent() {
                match maybe_save_query_result_files(parent, &payload) {
                    Ok(Some((json_path, csv_path))) => {
                        println!(
                            "[tcp server] split saved json={} csv={}",
                            json_path.display(),
                            csv_path.display()
                        );
                    }
                    Ok(None) => {}
                    Err(err) => {
                        eprintln!("[tcp server] split save failed: {}", format_error(&err));
                    }
                }
            }
        }
        println!("[tcp server] client disconnected: {}", peer_addr);
    }
}

fn run_udp_server_loop(sock: UdpSocket, save_path: &Path) -> Result<()> {
    let mut buf = [0_u8; 65535];
    let mut assemblies: HashMap<String, UdpChunkAssembly> = HashMap::new();
    let mut metrics = UdpChunkMetrics::default();
    let metrics_path = save_path
        .parent()
        .map(|parent| parent.join("udp_chunk_metrics.ndjson"));
    sock.set_read_timeout(Some(Duration::from_secs(UDP_SERVER_RECV_TIMEOUT_SECS)))
        .context("udp set_read_timeout failed")?;
    loop {
        let (n, peer_addr) = match sock.recv_from(&mut buf) {
            Ok(v) => v,
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                for expired in cleanup_expired_udp_assemblies(&mut assemblies) {
                    eprintln!(
                        "[udp server] drop stale chunk msg={} received={}/{}",
                        expired.msg_key, expired.received, expired.total
                    );
                    if let Some(path) = metrics_path.as_deref() {
                        log_udp_chunk_metric_event(
                            path,
                            &mut metrics,
                            "stale_drop",
                            &expired.msg_key,
                            expired.total,
                            expired.received,
                            None,
                        );
                    }
                }
                continue;
            }
            Err(err) => return Err(err).context("udp recv failed"),
        };
        let payload = String::from_utf8_lossy(&buf[..n]).trim_end().to_string();
        for expired in cleanup_expired_udp_assemblies(&mut assemblies) {
            eprintln!(
                "[udp server] drop stale chunk msg={} received={}/{}",
                expired.msg_key, expired.received, expired.total
            );
            if let Some(path) = metrics_path.as_deref() {
                log_udp_chunk_metric_event(
                    path,
                    &mut metrics,
                    "stale_drop",
                    &expired.msg_key,
                    expired.total,
                    expired.received,
                    None,
                );
            }
        }

        if let Some(chunk) = parse_udp_chunk_packet(&payload) {
            let msg_id = chunk.msg_id.clone();
            let part_total = chunk.total;
            let assembly_key = format!("{}|{}", peer_addr, msg_id);
            match ingest_udp_chunk(&mut assemblies, &assembly_key, chunk) {
                Ok(Some(reassembled_payload)) => {
                    let merged_bytes = reassembled_payload.as_bytes().len();
                    println!(
                        "[udp server] from={} reassembled msg_id={} parts={} bytes={}",
                        peer_addr, msg_id, part_total, merged_bytes
                    );
                    if let Some(path) = metrics_path.as_deref() {
                        log_udp_chunk_metric_event(
                            path,
                            &mut metrics,
                            "reassembled_ok",
                            &assembly_key,
                            part_total,
                            part_total,
                            None,
                        );
                    }
                    spawn_udp_payload_processing(
                        save_path,
                        peer_addr.to_string(),
                        merged_bytes,
                        reassembled_payload,
                    );
                }
                Ok(None) => {
                    if let Some(assembly) = assemblies.get(&assembly_key) {
                        if assembly.received == 1
                            || assembly.received % UDP_CHUNK_PROGRESS_LOG_EVERY == 0
                        {
                            println!(
                                "[udp server] chunk progress from={} msg_id={} received={}/{}",
                                peer_addr, msg_id, assembly.received, assembly.total
                            );
                        }
                    }
                }
                Err(err) => {
                    let err_msg = format_error(&err);
                    eprintln!(
                        "[udp server] invalid chunk from={} msg_id={} error={}",
                        peer_addr,
                        msg_id,
                        err_msg
                    );
                    if let Some(path) = metrics_path.as_deref() {
                        log_udp_chunk_metric_event(
                            path,
                            &mut metrics,
                            "invalid_chunk",
                            &assembly_key,
                            part_total,
                            0,
                            Some(err_msg),
                        );
                    }
                }
            }
            continue;
        }

        println!(
            "[udp server] from={} bytes={} payload={}",
            peer_addr, n, payload
        );
        spawn_udp_payload_processing(save_path, peer_addr.to_string(), n, payload);
    }
}

fn spawn_udp_payload_processing(save_path: &Path, peer: String, bytes: usize, payload: String) {
    let save_path = save_path.to_path_buf();
    thread::spawn(move || {
        process_udp_payload(&save_path, &peer, bytes, &payload);
    });
}

fn process_udp_payload(save_path: &Path, peer: &str, bytes: usize, payload: &str) {
    if let Err(err) = append_receive_record(save_path, "udp", peer, bytes, payload) {
        eprintln!("[udp server] save failed: {}", format_error(&err));
    }
    if let Some(parent) = save_path.parent() {
        match maybe_save_query_result_files(parent, payload) {
            Ok(Some((json_path, csv_path))) => {
                println!(
                    "[udp server] split saved json={} csv={}",
                    json_path.display(),
                    csv_path.display()
                );
            }
            Ok(None) => {}
            Err(err) => {
                eprintln!("[udp server] split save failed: {}", format_error(&err));
            }
        }
    }
}

fn parse_udp_chunk_packet(payload: &str) -> Option<UdpChunkPacketOwned> {
    let packet: UdpChunkPacketOwned = serde_json::from_str(payload).ok()?;
    (packet.kind == UDP_CHUNK_KIND).then_some(packet)
}

fn cleanup_expired_udp_assemblies(
    assemblies: &mut HashMap<String, UdpChunkAssembly>,
) -> Vec<ExpiredUdpChunk> {
    let ttl = Duration::from_secs(UDP_CHUNK_TTL_SECS);
    let now = Instant::now();
    let expired: Vec<ExpiredUdpChunk> = assemblies
        .iter()
        .filter(|(_, a)| now.duration_since(a.updated_at) < ttl)
        .map(|(k, a)| ExpiredUdpChunk {
            msg_key: k.clone(),
            total: a.total,
            received: a.received,
        })
        .collect();
    for e in &expired {
        assemblies.remove(&e.msg_key);
    }
    expired
}

fn ingest_udp_chunk(
    assemblies: &mut HashMap<String, UdpChunkAssembly>,
    assembly_key: &str,
    chunk: UdpChunkPacketOwned,
) -> Result<Option<String>> {
    if chunk.total == 0 || chunk.total > UDP_CHUNK_MAX_PARTS {
        return Err(anyhow!(
            "invalid chunk total={} (allowed: 1..={})",
            chunk.total,
            UDP_CHUNK_MAX_PARTS
        ));
    }
    if chunk.idx >= chunk.total {
        return Err(anyhow!(
            "invalid chunk idx={} (total={})",
            chunk.idx,
            chunk.total
        ));
    }

    let completed;
    {
        let assembly = assemblies
            .entry(assembly_key.to_string())
            .or_insert_with(|| UdpChunkAssembly {
                total: chunk.total,
                received: 0,
                parts: vec![None; chunk.total],
                updated_at: Instant::now(),
            });

        if assembly.total != chunk.total {
            return Err(anyhow!(
                "chunk total mismatch: existing={} incoming={}",
                assembly.total,
                chunk.total
            ));
        }

        assembly.updated_at = Instant::now();
        assembly.received += 1;
        assembly.parts[chunk.idx] = Some(chunk.data);
        completed = assembly.received == assembly.total;
    }

    if !completed {
        return Ok(None);
    }

    let assembly = assemblies
        .remove(assembly_key)
        .ok_or_else(|| anyhow!("chunk assembly missing after completion"))?;
    let mut merged = String::new();
    for part in assembly.parts {
        let part = part.ok_or_else(|| anyhow!("chunk assembly has missing part"))?;
        merged.push_str(&part);
    }
    Ok(Some(merged))
}

fn log_udp_chunk_metric_event(
    save_path: &Path,
    metrics: &mut UdpChunkMetrics,
    event: &str,
    msg_key: &str,
    total: usize,
    received: usize,
    error: Option<String>,
) {
    match event {
        "reassembled_ok" => metrics.reassembled_ok += 1,
        "invalid_chunk" => metrics.invalid_chunk += 1,
        "stale_drop" => metrics.stale_drop += 1,
        _ => {}
    }

    let record = UdpChunkMetricRecord {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        event: event.to_string(),
        msg_key: msg_key.to_string(),
        total,
        received,
        reassembled_ok: metrics.reassembled_ok,
        invalid_chunk: metrics.invalid_chunk,
        stale_drop: metrics.stale_drop,
        error,
    };

    if let Err(err) = append_udp_chunk_metric_record(save_path, &record) {
        eprintln!(
            "[udp server] chunk metrics save failed: {}",
            format_error(&err)
        );
    }
}

fn load_runtime_config(path: &Path) -> Result<RuntimeConfig> {
    if !path.exists() {
        return Err(anyhow!("config file not found: {}", path.display()));
    }

    let mut conf = Ini::new();
    let load_path = path
        .to_str()
        .ok_or_else(|| anyhow!("config path is not valid UTF-8"))?;
    conf.load(load_path)
        .map_err(|e| anyhow!("failed to load ini config: {e}"))?;

    let tcp = parse_endpoint(&conf, "tcpserver")?;
    let udp = parse_endpoint(&conf, "udpserver")?;
    let udp_chunk_size = parse_udp_chunk_size(&conf)?;

    let mut data_sources = Vec::new();
    let map = conf.get_map_ref();
    for (section_name, props) in map {
        if !section_name.starts_with("data-") {
            continue;
        }

        let enabled = parse_bool(get_optional(props, "enable").as_deref().unwrap_or("1"));
        let db_type = match get_required(props, "type", section_name)?
            .to_lowercase()
            .as_str()
        {
            "pg" | "postgres" | "postgresql" => DbType::PostgreSql,
            "mysql" => DbType::MySql,
            other => {
                return Err(anyhow!(
                    "unsupported db type `{}` in section [{}]",
                    other,
                    section_name
                ))
            }
        };

        let host = get_required(props, "host", section_name)?;
        let port: u16 = get_required(props, "port", section_name)?
            .parse()
            .with_context(|| format!("invalid port in section [{}]", section_name))?;
        let user = get_required(props, "user", section_name)?;
        let password = get_optional(props, "password").unwrap_or_default();
        let database = get_optional(props, "database");

        let mut query_keys: Vec<String> = props
            .iter()
            .filter_map(|(k, v)| {
                let value = v.as_deref().unwrap_or("").trim();
                (k.starts_with("sql_") && !value.is_empty()).then(|| k.clone())
            })
            .collect();
        query_keys.sort();

        if query_keys.is_empty() {
            return Err(anyhow!(
                "no sql_* query found in section [{}]",
                section_name
            ));
        }

        let queries = query_keys
            .into_iter()
            .map(|k| QueryJob {
                key: k.clone(),
                sql: props.get(&k).and_then(|v| v.clone()).unwrap_or_default(),
            })
            .collect();

        data_sources.push(DataSource {
            section_name: section_name.clone(),
            enabled,
            db_type,
            host,
            port,
            user,
            password,
            database,
            queries,
        });
    }

    if data_sources.is_empty() {
        return Err(anyhow!("no data-* section found in config"));
    }

    Ok(RuntimeConfig {
        tcp,
        udp,
        udp_chunk_size,
        data_sources,
    })
}

fn parse_udp_chunk_size(conf: &Ini) -> Result<usize> {
    let Some(raw) = conf.get("paraset", "chunksize") else {
        return Ok(UDP_CHUNK_DATA_DEFAULT_BYTES);
    };
    let parsed: usize = raw.parse().with_context(|| {
        format!("invalid [paraset] chunkSize `{raw}`; expect integer bytes")
    })?;
    if !(UDP_CHUNK_DATA_MIN_BYTES..=UDP_CHUNK_DATA_MAX_BYTES).contains(&parsed) {
        return Err(anyhow!(
            "invalid [paraset] chunkSize {}; allowed range is {}..={} bytes",
            parsed,
            UDP_CHUNK_DATA_MIN_BYTES,
            UDP_CHUNK_DATA_MAX_BYTES
        ));
    }
    Ok(parsed)
}

fn parse_endpoint(conf: &Ini, section: &str) -> Result<Option<Endpoint>> {
    let enabled_raw = conf.get(section, "enable");
    if enabled_raw.is_none() {
        return Ok(None);
    }

    let enabled = parse_bool(enabled_raw.as_deref().unwrap_or("0"));
    if !enabled {
        return Ok(Some(Endpoint {
            enabled: false,
            host: String::new(),
            port: 0,
        }));
    }

    let host = conf
        .get(section, "host")
        .ok_or_else(|| anyhow!("missing host in section [{}]", section))?;
    let port: u16 = conf
        .get(section, "port")
        .ok_or_else(|| anyhow!("missing port in section [{}]", section))?
        .parse()
        .with_context(|| format!("invalid port in section [{}]", section))?;

    Ok(Some(Endpoint {
        enabled: true,
        host,
        port,
    }))
}

fn get_required(
    props: &std::collections::HashMap<String, Option<String>>,
    key: &str,
    section: &str,
) -> Result<String> {
    match props.get(key).and_then(|v| v.clone()) {
        Some(v) if !v.trim().is_empty() => Ok(v),
        _ => Err(anyhow!("missing key `{}` in section [{}]", key, section)),
    }
}

fn get_optional(
    props: &std::collections::HashMap<String, Option<String>>,
    key: &str,
) -> Option<String> {
    props
        .get(key)
        .and_then(|v| v.clone())
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn parse_bool(s: &str) -> bool {
    matches!(
        s.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "y" | "on"
    )
}

fn run_query_and_export(
    ds: &DataSource,
    query: &QueryJob,
    output_dir: &Path,
) -> Result<ExportResult> {
    let file_base = build_output_file_base(&ds.section_name, &query.key);
    let csv_path = output_dir.join(format!("{file_base}.csv"));
    let json_path = output_dir.join(format!("{file_base}.json"));

    match ds.db_type {
        DbType::PostgreSql => export_postgres(ds, query, &csv_path, &json_path),
        DbType::MySql => export_mysql(ds, query, &csv_path, &json_path),
    }
}

fn build_output_file_base(source: &str, query_key: &str) -> String {
    let ts = Local::now().format("%Y%m%d_%H%M%S");
    format!("{}_{}_{}", sanitize(source), sanitize(query_key), ts)
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

#[derive(Debug)]
struct CsvFileMeta {
    path: PathBuf,
    file_name: String,
    headers: Vec<String>,
}

#[derive(Debug)]
struct ImportOutcome {
    file: PathBuf,
    total_rows: usize,
    inserted_rows: usize,
    skipped_rows: usize,
}

fn run_batch_csv_import(cfg: &RuntimeConfig, import_dir: &Path) -> Result<()> {
    let csv_files = discover_csv_files(import_dir)?;
    if csv_files.is_empty() {
        return Err(anyhow!("no csv file found in {}", import_dir.display()));
    }

    println!(
        "Import mode: found {} csv files in {}",
        csv_files.len(),
        import_dir.display()
    );

    let mut total_jobs = 0usize;
    let mut ok_jobs = 0usize;
    let mut errors = Vec::new();

    for ds in cfg.data_sources.iter().filter(|d| d.enabled) {
        println!(
            "Import target [{}] ({})",
            ds.section_name,
            ds.db_type.as_str()
        );
        for query in &ds.queries {
            total_jobs += 1;
            match import_one_query_from_csv(ds, query, &csv_files) {
                Ok(out) => {
                    ok_jobs += 1;
                    println!(
                        "  - {} OK: file={} total={} inserted={} skipped={}",
                        query.key,
                        out.file.display(),
                        out.total_rows,
                        out.inserted_rows,
                        out.skipped_rows
                    );
                }
                Err(err) => {
                    let msg = format!(
                        "{}::{} => {}",
                        ds.section_name,
                        query.key,
                        format_error(&err)
                    );
                    eprintln!("  - {} FAILED: {}", query.key, format_error(&err));
                    errors.push(msg);
                }
            }
        }
    }

    println!("Import finished: {ok_jobs}/{total_jobs} jobs succeeded.");
    if errors.is_empty() {
        return Ok(());
    }
    Err(anyhow!(
        "{} import jobs failed:\n{}",
        errors.len(),
        errors.join("\n")
    ))
}

fn discover_csv_files(import_dir: &Path) -> Result<Vec<CsvFileMeta>> {
    if !import_dir.exists() {
        return Err(anyhow!("import dir not found: {}", import_dir.display()));
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(import_dir)
        .with_context(|| format!("failed to read dir {}", import_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("csv") {
            continue;
        }
        let file_name = match path.file_name().and_then(|x| x.to_str()) {
            Some(x) => x.to_string(),
            None => continue,
        };
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&path)
            .with_context(|| format!("failed to open csv {}", path.display()))?;
        let headers = rdr
            .headers()
            .with_context(|| format!("failed to read csv header {}", path.display()))?
            .iter()
            .map(|x| x.to_string())
            .collect();
        out.push(CsvFileMeta {
            path,
            file_name,
            headers,
        });
    }
    Ok(out)
}

fn import_one_query_from_csv(
    ds: &DataSource,
    query: &QueryJob,
    csv_files: &[CsvFileMeta],
) -> Result<ImportOutcome> {
    let table_full = extract_target_table_from_sql(&query.sql)
        .with_context(|| format!("failed to parse target table for query {}", query.key))?;
    match ds.db_type {
        DbType::PostgreSql => import_one_query_csv_pg(ds, query, &table_full, csv_files),
        DbType::MySql => import_one_query_csv_mysql(ds, query, &table_full, csv_files),
    }
}

fn import_one_query_csv_pg(
    ds: &DataSource,
    query: &QueryJob,
    table_full: &str,
    csv_files: &[CsvFileMeta],
) -> Result<ImportOutcome> {
    let db_name = ds.database.as_deref().unwrap_or("postgres");
    let conn_str = format!(
        "host='{}' port={} user='{}' password='{}' dbname='{}' connect_timeout=10",
        escape_pg_conn_value(&ds.host),
        ds.port,
        escape_pg_conn_value(&ds.user),
        escape_pg_conn_value(&ds.password),
        escape_pg_conn_value(db_name)
    );
    let mut client = Client::connect(&conn_str, NoTls)
        .with_context(|| format!("postgres connect failed for [{}]", ds.section_name))?;

    let (schema, table) = split_schema_table(table_full, "public");
    let (schema_used, table_cols) = resolve_pg_table_columns(&mut client, &schema, &table)?;
    let selected = choose_csv_for_query(csv_files, &query.key, &table_cols)?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&selected.path)
        .with_context(|| format!("failed to open csv {}", selected.path.display()))?;
    let headers = rdr
        .headers()
        .with_context(|| format!("failed to read csv header {}", selected.path.display()))?
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let table_quoted = quote_pg_table(&schema_used, &table);
    let columns_sql = headers
        .iter()
        .map(|h| quote_pg_ident(h))
        .collect::<Vec<_>>()
        .join(",");

    let mut total = 0usize;
    let mut inserted = 0usize;
    let mut skipped = 0usize;
    let mut skipped_logged = 0usize;
    for rec in rdr.records() {
        let rec =
            rec.with_context(|| format!("failed to read csv row {}", selected.path.display()))?;
        total += 1;
        let values_sql = rec
            .iter()
            .map(pg_value_literal)
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT DO NOTHING",
            table_quoted, columns_sql, values_sql
        );
        match client.execute(sql.as_str(), &[]) {
            Ok(n) => inserted += n as usize,
            Err(err) => {
                skipped += 1;
                if skipped_logged < 3 {
                    eprintln!(
                        "  - {} WARN row {} skipped: {}",
                        query.key,
                        total,
                        format_postgres_insert_error(&err)
                    );
                    skipped_logged += 1;
                }
            }
        }
    }
    if skipped > 3 {
        eprintln!(
            "  - {} WARN skipped rows total={} (only first 3 shown)",
            query.key, skipped
        );
    }

    Ok(ImportOutcome {
        file: selected.path.clone(),
        total_rows: total,
        inserted_rows: inserted,
        skipped_rows: skipped,
    })
}

fn import_one_query_csv_mysql(
    ds: &DataSource,
    query: &QueryJob,
    table_full: &str,
    csv_files: &[CsvFileMeta],
) -> Result<ImportOutcome> {
    let mut opts = OptsBuilder::new()
        .ip_or_hostname(Some(ds.host.clone()))
        .tcp_port(ds.port)
        .user(Some(ds.user.clone()))
        .pass(Some(ds.password.clone()));
    if let Some(db) = &ds.database {
        opts = opts.db_name(Some(db.clone()));
    }
    let pool = Pool::new(opts).with_context(|| {
        format!(
            "mysql pool creation failed for [{}] {}:{}",
            ds.section_name, ds.host, ds.port
        )
    })?;
    let mut conn = pool
        .get_conn()
        .with_context(|| format!("mysql connect failed for [{}]", ds.section_name))?;

    let default_schema = ds.database.as_deref().unwrap_or("");
    let (schema, table) = split_schema_table(table_full, default_schema);
    let (schema_used, table_cols) = resolve_mysql_table_columns(&mut conn, &schema, &table)?;
    let selected = choose_csv_for_query(csv_files, &query.key, &table_cols)?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&selected.path)
        .with_context(|| format!("failed to open csv {}", selected.path.display()))?;
    let headers = rdr
        .headers()
        .with_context(|| format!("failed to read csv header {}", selected.path.display()))?
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let table_quoted = quote_mysql_table(&schema_used, &table);
    let columns_sql = headers
        .iter()
        .map(|h| quote_mysql_ident(h))
        .collect::<Vec<_>>()
        .join(",");

    let mut total = 0usize;
    let mut inserted = 0usize;
    let mut skipped = 0usize;
    let mut skipped_logged = 0usize;
    for rec in rdr.records() {
        let rec =
            rec.with_context(|| format!("failed to read csv row {}", selected.path.display()))?;
        total += 1;
        let values_sql = rec
            .iter()
            .map(mysql_value_literal)
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            "INSERT IGNORE INTO {} ({}) VALUES ({})",
            table_quoted, columns_sql, values_sql
        );
        match conn.query_drop(sql) {
            Ok(_) => inserted += conn.affected_rows() as usize,
            Err(err) => {
                skipped += 1;
                if skipped_logged < 3 {
                    eprintln!("  - {} WARN row {} skipped: {}", query.key, total, err);
                    skipped_logged += 1;
                }
            }
        }
    }
    if skipped > 3 {
        eprintln!(
            "  - {} WARN skipped rows total={} (only first 3 shown)",
            query.key, skipped
        );
    }
    drop(conn);
    drop(pool);

    Ok(ImportOutcome {
        file: selected.path.clone(),
        total_rows: total,
        inserted_rows: inserted,
        skipped_rows: skipped,
    })
}

fn extract_target_table_from_sql(sql: &str) -> Result<String> {
    let raw = sql.trim().trim_end_matches(';');
    let lower = raw.to_lowercase();
    let from_pos = lower
        .find(" from ")
        .or_else(|| lower.find("from "))
        .ok_or_else(|| anyhow!("sql has no FROM clause: {}", raw))?;
    let after_from = if lower[from_pos..].starts_with(" from ") {
        &raw[(from_pos + 6)..]
    } else {
        &raw[(from_pos + 5)..]
    };
    let table = after_from
        .trim_start()
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow!("cannot parse table name from sql: {}", raw))?;
    Ok(table.trim_matches('`').trim_matches('"').to_string())
}

fn split_schema_table(full: &str, default_schema: &str) -> (String, String) {
    if let Some((schema, table)) = full.split_once('.') {
        (
            schema.trim_matches('`').trim_matches('"').to_string(),
            table.trim_matches('`').trim_matches('"').to_string(),
        )
    } else {
        (
            default_schema
                .trim_matches('`')
                .trim_matches('"')
                .to_string(),
            full.trim_matches('`').trim_matches('"').to_string(),
        )
    }
}

fn get_pg_table_columns(client: &mut Client, schema: &str, table: &str) -> Result<HashSet<String>> {
    let sql = "SELECT column_name FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2";
    let rows = client
        .query(sql, &[&schema, &table])
        .with_context(|| format!("postgres read table columns failed: {}.{}", schema, table))?;
    Ok(rows
        .into_iter()
        .filter_map(|r| r.try_get::<usize, String>(0).ok())
        .map(|x| x.to_ascii_lowercase())
        .collect::<HashSet<_>>())
}

fn resolve_pg_table_columns(
    client: &mut Client,
    schema: &str,
    table: &str,
) -> Result<(String, HashSet<String>)> {
    let direct = get_pg_table_columns(client, schema, table)?;
    if !direct.is_empty() {
        return Ok((schema.to_string(), direct));
    }

    let sql = "SELECT DISTINCT table_schema FROM information_schema.columns WHERE table_name = $1";
    let rows = client
        .query(sql, &[&table])
        .with_context(|| format!("postgres resolve table schema failed: {}", table))?;
    let schemas = rows
        .into_iter()
        .filter_map(|r| r.try_get::<usize, String>(0).ok())
        .collect::<Vec<_>>();

    if schemas.is_empty() {
        return Err(anyhow!("target table not found: {}.{}", schema, table));
    }
    if schemas.len() > 1 {
        return Err(anyhow!(
            "target table `{}` exists in multiple schemas {:?}; please specify exact schema in sql",
            table,
            schemas
        ));
    }

    let picked = schemas[0].clone();
    let cols = get_pg_table_columns(client, picked.as_str(), table)?;
    if cols.is_empty() {
        return Err(anyhow!(
            "target table not found after resolve: {}.{}",
            picked,
            table
        ));
    }
    eprintln!(
        "[import] postgres table {}.{} not found, fallback to {}.{}",
        schema, table, picked, table
    );
    Ok((picked, cols))
}

fn get_mysql_table_columns(
    conn: &mut mysql::PooledConn,
    schema: &str,
    table: &str,
) -> Result<HashSet<String>> {
    let q = format!(
        "SELECT column_name FROM information_schema.columns WHERE table_schema='{}' AND table_name='{}'",
        schema.replace('\'', "''"),
        table.replace('\'', "''")
    );
    let cols: Vec<String> = conn
        .query(q)
        .with_context(|| format!("mysql read table columns failed: {}.{}", schema, table))?;
    Ok(cols
        .into_iter()
        .map(|x| x.to_ascii_lowercase())
        .collect::<HashSet<_>>())
}

fn resolve_mysql_table_columns(
    conn: &mut mysql::PooledConn,
    schema: &str,
    table: &str,
) -> Result<(String, HashSet<String>)> {
    let direct = get_mysql_table_columns(conn, schema, table)?;
    if !direct.is_empty() {
        return Ok((schema.to_string(), direct));
    }

    let q = format!(
        "SELECT DISTINCT table_schema FROM information_schema.columns WHERE table_name='{}'",
        table.replace('\'', "''")
    );
    let schemas: Vec<String> = conn
        .query(q)
        .with_context(|| format!("mysql resolve table schema failed: {}", table))?;

    if schemas.is_empty() {
        return Err(anyhow!("target table not found: {}.{}", schema, table));
    }
    if schemas.len() > 1 {
        return Err(anyhow!(
            "target table `{}` exists in multiple schemas {:?}; please specify exact schema in sql",
            table,
            schemas
        ));
    }

    let picked = schemas[0].clone();
    let cols = get_mysql_table_columns(conn, picked.as_str(), table)?;
    if cols.is_empty() {
        return Err(anyhow!(
            "target table not found after resolve: {}.{}",
            picked,
            table
        ));
    }
    eprintln!(
        "[import] mysql table {}.{} not found, fallback to {}.{}",
        schema, table, picked, table
    );
    Ok((picked, cols))
}

fn choose_csv_for_query<'a>(
    csv_files: &'a [CsvFileMeta],
    query_key: &str,
    table_cols_lower: &HashSet<String>,
) -> Result<&'a CsvFileMeta> {
    let token = format!("_{}_", query_key);
    let mut full_matches = csv_files
        .iter()
        .filter(|f| f.file_name.contains(&token))
        .filter(|f| {
            !f.headers.is_empty()
                && f.headers
                    .iter()
                    .all(|h| table_cols_lower.contains(&h.to_ascii_lowercase()))
        })
        .collect::<Vec<_>>();

    if !full_matches.is_empty() {
        full_matches.sort_by_key(|f| extract_timestamp_rank(&f.file_name));
        return Ok(*full_matches
            .last()
            .expect("full_matches should not be empty"));
    }

    let candidates = csv_files
        .iter()
        .filter(|f| f.file_name.contains(&token))
        .map(|f| f.file_name.clone())
        .collect::<Vec<_>>();
    Err(anyhow!(
        "no compatible csv found for query key `{}`; candidates={:?}",
        query_key,
        candidates
    ))
}

fn extract_timestamp_rank(file_name: &str) -> String {
    let base = file_name.strip_suffix(".csv").unwrap_or(file_name);
    let mut iter = base.rsplit('_');
    let t = iter.next().unwrap_or("");
    let d = iter.next().unwrap_or("");
    if d.len() == 8 && t.len() == 6 {
        format!("{}_{}", d, t)
    } else {
        base.to_string()
    }
}

fn quote_pg_ident(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

fn quote_pg_table(schema: &str, table: &str) -> String {
    if schema.is_empty() {
        quote_pg_ident(table)
    } else {
        format!("{}.{}", quote_pg_ident(schema), quote_pg_ident(table))
    }
}

fn quote_mysql_ident(s: &str) -> String {
    format!("`{}`", s.replace('`', "``"))
}

fn quote_mysql_table(schema: &str, table: &str) -> String {
    if schema.is_empty() {
        quote_mysql_ident(table)
    } else {
        format!("{}.{}", quote_mysql_ident(schema), quote_mysql_ident(table))
    }
}

fn pg_value_literal(v: &str) -> String {
    sql_value_literal(v)
}

fn mysql_value_literal(v: &str) -> String {
    sql_value_literal(v)
}

fn sql_value_literal(v: &str) -> String {
    let trimmed = v.trim();
    if trimmed.is_empty() {
        return "NULL".to_string();
    }
    if trimmed.starts_with("<unsupported:") && trimmed.ends_with('>') {
        return "NULL".to_string();
    }
    let escaped = v.replace('\\', "\\\\").replace('\'', "''");
    format!("'{}'", escaped)
}

fn export_postgres(
    ds: &DataSource,
    query: &QueryJob,
    csv_path: &Path,
    json_path: &Path,
) -> Result<ExportResult> {
    let db_name = ds.database.as_deref().unwrap_or("postgres");
    let conn_str = format!(
        "host='{}' port={} user='{}' password='{}' dbname='{}' connect_timeout=10",
        escape_pg_conn_value(&ds.host),
        ds.port,
        escape_pg_conn_value(&ds.user),
        escape_pg_conn_value(&ds.password),
        escape_pg_conn_value(db_name)
    );
    let mut client = Client::connect(&conn_str, NoTls)
        .with_context(|| format!("postgres connect failed for [{}]", ds.section_name))?;
    let stmt = client
        .prepare(query.sql.as_str())
        .with_context(|| format!("postgres prepare failed for query {}", query.key))?;
    let headers: Vec<String> = stmt
        .columns()
        .iter()
        .map(|c| c.name().to_string())
        .collect();
    let rows = client
        .query(&stmt, &[])
        .with_context(|| format!("postgres query failed for query {}", query.key))?;

    let mut row_values: Vec<Vec<String>> = Vec::with_capacity(rows.len());
    for row in &rows {
        let mut values = Vec::with_capacity(headers.len());
        for i in 0..headers.len() {
            values.push(pg_cell_to_string(row, i));
        }
        row_values.push(values);
    }

    write_csv(csv_path, &headers, &row_values)?;
    write_json(json_path, &headers, &row_values)?;

    Ok(ExportResult {
        row_count: rows.len(),
        csv_file: csv_path.to_path_buf(),
        json_file: json_path.to_path_buf(),
    })
}

fn export_mysql(
    ds: &DataSource,
    query: &QueryJob,
    csv_path: &Path,
    json_path: &Path,
) -> Result<ExportResult> {
    let mut opts = OptsBuilder::new()
        .ip_or_hostname(Some(ds.host.clone()))
        .tcp_port(ds.port)
        .user(Some(ds.user.clone()))
        .pass(Some(ds.password.clone()));
    if let Some(db) = &ds.database {
        opts = opts.db_name(Some(db.clone()));
    }

    let pool = Pool::new(opts).with_context(|| {
        format!(
            "mysql pool creation failed for [{}] {}:{}",
            ds.section_name, ds.host, ds.port
        )
    })?;
    let mut conn = pool
        .get_conn()
        .with_context(|| format!("mysql connect failed for [{}]", ds.section_name))?;

    let mut result = conn
        .query_iter(query.sql.as_str())
        .with_context(|| format!("mysql query failed for query {}", query.key))?;
    let columns = result.columns();
    let headers: Vec<String> = columns
        .as_ref()
        .iter()
        .map(|c| c.name_str().to_string())
        .collect();

    let mut row_count = 0usize;
    let mut row_values: Vec<Vec<String>> = Vec::new();
    for row_result in result.by_ref() {
        let row = row_result.with_context(|| format!("failed to read mysql row {}", query.key))?;
        let mut values = Vec::with_capacity(headers.len());
        for i in 0..headers.len() {
            let cell = row.as_ref(i).cloned().unwrap_or(MyValue::NULL);
            values.push(mysql_value_to_string(cell));
        }
        row_values.push(values);
        row_count += 1;
    }

    drop(result);
    drop(conn);
    drop(pool);
    write_csv(csv_path, &headers, &row_values)?;
    write_json(json_path, &headers, &row_values)?;

    Ok(ExportResult {
        row_count,
        csv_file: csv_path.to_path_buf(),
        json_file: json_path.to_path_buf(),
    })
}

fn write_csv(csv_path: &Path, headers: &[String], rows: &[Vec<String>]) -> Result<()> {
    let file = File::create(csv_path)
        .with_context(|| format!("failed to create csv {}", csv_path.display()))?;
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.write_record(headers)
        .with_context(|| format!("failed to write csv header {}", csv_path.display()))?;

    for row in rows {
        wtr.write_record(row)
            .with_context(|| format!("failed to write csv row {}", csv_path.display()))?;
    }
    wtr.flush()
        .with_context(|| format!("failed to flush csv {}", csv_path.display()))?;
    Ok(())
}

fn write_json(json_path: &Path, headers: &[String], rows: &[Vec<String>]) -> Result<()> {
    let mut records: Vec<JsonValue> = Vec::with_capacity(rows.len());
    for row in rows {
        let mut obj = JsonMap::with_capacity(headers.len());
        for (col, val) in headers.iter().zip(row.iter()) {
            obj.insert(col.clone(), JsonValue::String(val.clone()));
        }
        records.push(JsonValue::Object(obj));
    }

    let file = File::create(json_path)
        .with_context(|| format!("failed to create json {}", json_path.display()))?;
    serde_json::to_writer_pretty(file, &records)
        .with_context(|| format!("failed to write json {}", json_path.display()))?;
    Ok(())
}

fn append_receive_record(
    save_path: &Path,
    protocol: &str,
    peer: &str,
    bytes: usize,
    payload: &str,
) -> Result<()> {
    let record = ServerReceiveRecord {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        protocol: protocol.to_string(),
        peer: peer.to_string(),
        bytes,
        payload: payload.to_string(),
    };
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(save_path)
        .with_context(|| format!("failed to open receive log {}", save_path.display()))?;
    let line = serde_json::to_string(&record).context("failed to serialize receive record")?;
    writeln!(file, "{line}").context("failed to append receive record")?;
    Ok(())
}

fn append_udp_chunk_metric_record(save_path: &Path, record: &UdpChunkMetricRecord) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(save_path)
        .with_context(|| format!("failed to open chunk metrics log {}", save_path.display()))?;
    let line =
        serde_json::to_string(record).context("failed to serialize udp chunk metric record")?;
    writeln!(file, "{line}").context("failed to append udp chunk metric record")?;
    Ok(())
}

fn maybe_save_query_result_files(
    save_root: &Path,
    payload: &str,
) -> Result<Option<(PathBuf, PathBuf)>> {
    let parsed: JsonValue = match serde_json::from_str(payload) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    let obj = match parsed.as_object() {
        Some(o) => o,
        None => return Ok(None),
    };
    if obj.get("kind").and_then(|v| v.as_str()) != Some("query_json") {
        return Ok(None);
    }

    let source = obj
        .get("source")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("query_json missing field `source`"))?;
    let query_key = obj
        .get("query_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("query_json missing field `query_key`"))?;
    let timestamp = obj
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let data = obj
        .get("data")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("query_json missing array field `data`"))?;

    let out_dir = save_root.join("query_results");
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed to create query result dir {}", out_dir.display()))?;

    let ts = normalize_filename_timestamp(timestamp);
    let base = format!("{}_{}_{}", sanitize(source), sanitize(query_key), ts);
    let json_path = out_dir.join(format!("{base}.json"));
    let csv_path = out_dir.join(format!("{base}.csv"));

    if !json_path.exists() {
        let jf = File::create(&json_path)
            .with_context(|| format!("failed to create split json {}", json_path.display()))?;
        serde_json::to_writer_pretty(jf, data)
            .with_context(|| format!("failed to write split json {}", json_path.display()))?;
    }

    if !csv_path.exists() {
        let (headers, rows) = json_array_to_csv_rows(data)?;
        write_csv(&csv_path, &headers, &rows)?;
    }

    Ok(Some((json_path, csv_path)))
}

fn normalize_filename_timestamp(ts: &str) -> String {
    if let Ok(t) = NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S") {
        return t.format("%Y%m%d_%H%M%S").to_string();
    }
    sanitize(ts)
}

fn json_array_to_csv_rows(data: &[JsonValue]) -> Result<(Vec<String>, Vec<Vec<String>>)> {
    let mut headers: Vec<String> = Vec::new();
    let mut header_set: HashSet<String> = HashSet::new();

    for item in data {
        if let Some(obj) = item.as_object() {
            for key in obj.keys() {
                if header_set.insert(key.clone()) {
                    headers.push(key.clone());
                }
            }
        }
    }

    let mut rows: Vec<Vec<String>> = Vec::with_capacity(data.len());
    for item in data {
        if let Some(obj) = item.as_object() {
            let mut row = Vec::with_capacity(headers.len());
            for h in &headers {
                row.push(json_cell_to_string(obj.get(h)));
            }
            rows.push(row);
        }
    }
    Ok((headers, rows))
}

fn json_cell_to_string(v: Option<&JsonValue>) -> String {
    match v {
        None | Some(JsonValue::Null) => String::new(),
        Some(JsonValue::String(s)) => s.clone(),
        Some(JsonValue::Bool(b)) => b.to_string(),
        Some(JsonValue::Number(n)) => n.to_string(),
        Some(other) => other.to_string(),
    }
}

fn pg_cell_to_string(row: &Row, idx: usize) -> String {
    if let Ok(v) = row.try_get::<usize, Option<String>>(idx) {
        return option_to_string(v);
    }
    if let Ok(v) = row.try_get::<usize, Option<i64>>(idx) {
        return option_to_string(v);
    }
    if let Ok(v) = row.try_get::<usize, Option<i32>>(idx) {
        return option_to_string(v);
    }
    if let Ok(v) = row.try_get::<usize, Option<i16>>(idx) {
        return option_to_string(v);
    }
    if let Ok(v) = row.try_get::<usize, Option<f64>>(idx) {
        return option_to_string(v);
    }
    if let Ok(v) = row.try_get::<usize, Option<f32>>(idx) {
        return option_to_string(v);
    }
    if let Ok(v) = row.try_get::<usize, Option<bool>>(idx) {
        return option_to_string(v);
    }
    if let Ok(v) = row.try_get::<usize, Option<NaiveDateTime>>(idx) {
        return v
            .map(|x| x.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_default();
    }
    if let Ok(v) = row.try_get::<usize, Option<NaiveDate>>(idx) {
        return v
            .map(|x| x.format("%Y-%m-%d").to_string())
            .unwrap_or_default();
    }
    if let Ok(v) = row.try_get::<usize, Option<NaiveTime>>(idx) {
        return v
            .map(|x| x.format("%H:%M:%S").to_string())
            .unwrap_or_default();
    }
    if let Ok(v) = row.try_get::<usize, Option<JsonValue>>(idx) {
        return v.map(|x| x.to_string()).unwrap_or_default();
    }
    if let Ok(v) = row.try_get::<usize, Option<Vec<u8>>>(idx) {
        return v
            .map(|x| String::from_utf8_lossy(&x).to_string())
            .unwrap_or_default();
    }
    let type_name = row.columns()[idx].type_().name();
    format!("<unsupported:{}>", type_name)
}

fn mysql_value_to_string(v: MyValue) -> String {
    match v {
        MyValue::NULL => String::new(),
        MyValue::Bytes(b) => String::from_utf8_lossy(&b).to_string(),
        MyValue::Int(n) => n.to_string(),
        MyValue::UInt(n) => n.to_string(),
        MyValue::Float(f) => f.to_string(),
        MyValue::Double(f) => f.to_string(),
        MyValue::Date(y, m, d, hh, mm, ss, micros) => {
            if micros > 0 {
                format!("{y:04}-{m:02}-{d:02} {hh:02}:{mm:02}:{ss:02}.{:06}", micros)
            } else {
                format!("{y:04}-{m:02}-{d:02} {hh:02}:{mm:02}:{ss:02}")
            }
        }
        MyValue::Time(is_neg, days, hh, mm, ss, micros) => {
            let sign = if is_neg { "-" } else { "" };
            if micros > 0 {
                format!("{sign}{days} {hh:02}:{mm:02}:{ss:02}.{:06}", micros)
            } else {
                format!("{sign}{days} {hh:02}:{mm:02}:{ss:02}")
            }
        }
    }
}

fn option_to_string<T: ToString>(v: Option<T>) -> String {
    v.map(|x| x.to_string()).unwrap_or_default()
}

fn format_error(err: &anyhow::Error) -> String {
    format!("{:#}", err)
}

fn format_postgres_insert_error(err: &postgres::Error) -> String {
    if let Some(db) = err.as_db_error() {
        let mut msg = db.message().to_string();
        if let Some(detail) = db.detail() {
            msg.push_str(" | detail: ");
            msg.push_str(detail);
        }
        if let Some(table) = db.table() {
            msg.push_str(" | table: ");
            msg.push_str(table);
        }
        return msg;
    }
    err.to_string()
}

fn escape_pg_conn_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\'', "\\'")
}

fn check_tcp_endpoint(endpoint: &Endpoint) -> Result<()> {
    let addr = endpoint.addr();
    let sock_addr = addr
        .to_socket_addrs()
        .with_context(|| format!("resolve tcp address failed `{}`", addr))?
        .next()
        .ok_or_else(|| anyhow!("resolve tcp address failed `{}`", addr))?;
    let mut stream = TcpStream::connect_timeout(&sock_addr, Duration::from_secs(3))
        .with_context(|| format!("tcp connect failed: {}", addr))?;
    stream
        .write_all(b"health_check\n")
        .context("tcp health payload write failed")?;
    Ok(())
}

fn check_udp_endpoint(endpoint: &Endpoint) -> Result<()> {
    let addr = endpoint.addr();
    let sock = UdpSocket::bind("0.0.0.0:0").context("udp bind failed")?;
    sock.connect(addr.as_str())
        .with_context(|| format!("udp connect failed: {}", addr))?;
    let _ = sock
        .send(b"health_check\n")
        .with_context(|| format!("udp send failed: {}", addr))?;
    Ok(())
}

fn check_database_connection(ds: &DataSource) -> Result<()> {
    match ds.db_type {
        DbType::PostgreSql => check_postgres_connection(ds),
        DbType::MySql => check_mysql_connection(ds),
    }
}

fn check_postgres_connection(ds: &DataSource) -> Result<()> {
    let db_name = ds.database.as_deref().unwrap_or("postgres");
    let conn_str = format!(
        "host='{}' port={} user='{}' password='{}' dbname='{}' connect_timeout=5",
        escape_pg_conn_value(&ds.host),
        ds.port,
        escape_pg_conn_value(&ds.user),
        escape_pg_conn_value(&ds.password),
        escape_pg_conn_value(db_name)
    );
    let mut client = Client::connect(&conn_str, NoTls)
        .with_context(|| format!("postgres connect failed for [{}]", ds.section_name))?;
    let _rows = client
        .query("SELECT 1", &[])
        .with_context(|| format!("postgres ping query failed for [{}]", ds.section_name))?;
    Ok(())
}

fn check_mysql_connection(ds: &DataSource) -> Result<()> {
    let mut opts = OptsBuilder::new()
        .ip_or_hostname(Some(ds.host.clone()))
        .tcp_port(ds.port)
        .user(Some(ds.user.clone()))
        .pass(Some(ds.password.clone()));
    if let Some(db) = &ds.database {
        opts = opts.db_name(Some(db.clone()));
    }

    let pool = Pool::new(opts)
        .with_context(|| format!("mysql pool creation failed for [{}]", ds.section_name))?;
    let mut conn = pool
        .get_conn()
        .with_context(|| format!("mysql connect failed for [{}]", ds.section_name))?;
    let _ping: Option<u8> = conn
        .query_first("SELECT 1")
        .with_context(|| format!("mysql ping query failed for [{}]", ds.section_name))?;
    drop(conn);
    drop(pool);
    Ok(())
}

fn send_query_json_payload(
    cfg: &RuntimeConfig,
    ds: &DataSource,
    query: &QueryJob,
    result: &ExportResult,
) -> Result<()> {
    let file = File::open(&result.json_file)
        .with_context(|| format!("failed to open json file {}", result.json_file.display()))?;
    let data: JsonValue = serde_json::from_reader(file)
        .with_context(|| format!("failed to parse json file {}", result.json_file.display()))?;

    let packet = QueryDataPacket {
        kind: "query_json".to_string(),
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        source: ds.section_name.clone(),
        db_type: ds.db_type.as_str().to_string(),
        query_key: query.key.clone(),
        row_count: result.row_count,
        data,
    };
    let payload =
        serde_json::to_string(&packet).context("failed to serialize query json transfer packet")?;
    println!(
        "[query_json] source={} key={} bytes={}",
        ds.section_name,
        query.key,
        payload.as_bytes().len()
    );

    let mut tried = 0usize;
    let mut ok = 0usize;
    let mut errs = Vec::new();

    if let Some(tcp) = &cfg.tcp {
        if tcp.enabled {
            tried += 1;
            match send_tcp(tcp, &payload) {
                Ok(_) => ok += 1,
                Err(err) => errs.push(format!("tcp {} => {}", tcp.addr(), format_error(&err))),
            }
        }
    }
    if let Some(udp) = &cfg.udp {
        if udp.enabled {
            tried += 1;
            match send_udp(udp, &payload, cfg.udp_chunk_size) {
                Ok(_) => ok += 1,
                Err(err) => errs.push(format!("udp {} => {}", udp.addr(), format_error(&err))),
            }
        }
    }

    if tried == 0 {
        return Err(anyhow!(
            "no enabled transport for query json forwarding ([tcpserver]/[udpserver] enable=1 required)"
        ));
    }
    if ok == tried {
        return Ok(());
    }
    Err(anyhow!(
        "query json forwarding failed: {ok}/{tried} passed; {}",
        errs.join(" | ")
    ))
}

fn send_transport_test_packet(cfg: &RuntimeConfig) -> Result<()> {
    let payload = format!(
        "{{\"kind\":\"transport_test\",\"timestamp\":\"{}\",\"message\":\"hello from data-import-sql\"}}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    let mut tried = 0usize;
    let mut ok = 0usize;
    let mut errors = Vec::new();

    if let Some(tcp) = &cfg.tcp {
        if tcp.enabled {
            tried += 1;
            match send_tcp(tcp, &payload) {
                Ok(_) => {
                    ok += 1;
                    println!("[tcpserver] enable=1 target={} send_test=OK", tcp.addr());
                }
                Err(err) => {
                    let msg = format!(
                        "[tcpserver] enable=1 target={} send_test=FAILED error={}",
                        tcp.addr(),
                        format_error(&err)
                    );
                    println!("{msg}");
                    errors.push(msg);
                }
            }
        } else {
            println!("[tcpserver] enable=0 send_test=SKIPPED");
        }
    } else {
        println!("[tcpserver] enable=0 send_test=NOT_CONFIGURED");
    }

    if let Some(udp) = &cfg.udp {
        if udp.enabled {
            tried += 1;
            match send_udp(udp, &payload, cfg.udp_chunk_size) {
                Ok(_) => {
                    ok += 1;
                    println!("[udpserver] enable=1 target={} send_test=OK", udp.addr());
                }
                Err(err) => {
                    let msg = format!(
                        "[udpserver] enable=1 target={} send_test=FAILED error={}",
                        udp.addr(),
                        format_error(&err)
                    );
                    println!("{msg}");
                    errors.push(msg);
                }
            }
        } else {
            println!("[udpserver] enable=0 send_test=SKIPPED");
        }
    } else {
        println!("[udpserver] enable=0 send_test=NOT_CONFIGURED");
    }

    if tried == 0 {
        return Err(anyhow!(
            "no enabled tcp/udp endpoint found; set [tcpserver] or [udpserver] enable=1 first"
        ));
    }
    if ok == tried {
        println!("Transport test passed: {ok}/{tried}");
        return Ok(());
    }

    Err(anyhow!(
        "transport test failed: {ok}/{tried} passed\n{}",
        errors.join("\n")
    ))
}

fn send_report(cfg: &RuntimeConfig, report: &QueryReport) {
    let payload = match serde_json::to_string(report) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("failed to serialize report: {err}");
            return;
        }
    };

    if let Some(tcp) = &cfg.tcp {
        if tcp.enabled {
            if let Err(err) = send_tcp(tcp, &payload) {
                eprintln!("tcp report send failed to {}: {}", tcp.addr(), err);
            }
        }
    }

    if let Some(udp) = &cfg.udp {
        if udp.enabled {
            if let Err(err) = send_udp(udp, &payload, cfg.udp_chunk_size) {
                eprintln!("udp report send failed to {}: {}", udp.addr(), err);
            }
        }
    }
}

fn send_tcp(endpoint: &Endpoint, payload: &str) -> Result<()> {
    let addr = endpoint.addr();
    let mut stream = TcpStream::connect(addr.as_str())
        .with_context(|| format!("tcp connect failed: {}", addr))?;
    stream
        .write_all(payload.as_bytes())
        .context("tcp write failed")?;
    stream.write_all(b"\n").context("tcp write failed")?;
    Ok(())
}

fn send_udp(endpoint: &Endpoint, payload: &str, chunk_size: usize) -> Result<()> {
    let addr = endpoint.addr();
    let sock = UdpSocket::bind("0.0.0.0:0").context("udp bind failed")?;
    if payload.as_bytes().len() <= UDP_DIRECT_MAX_BYTES {
        send_udp_datagram_with_retry(
            &sock,
            addr.as_str(),
            payload.as_bytes(),
            format!("udp send failed: {}", addr).as_str(),
        )?;
        return Ok(());
    }

    let chunks = split_utf8_chunks(payload, chunk_size)?;
    if chunks.len() > UDP_CHUNK_MAX_PARTS {
        return Err(anyhow!(
            "udp payload too large after split: {} parts (max {})",
            chunks.len(),
            UDP_CHUNK_MAX_PARTS
        ));
    }

    let seq = UDP_MSG_SEQ.fetch_add(1, Ordering::Relaxed);
    let msg_id = format!("{}-{}", Local::now().format("%Y%m%d%H%M%S%3f"), seq);
    let total = chunks.len();

    for (idx, data) in chunks.into_iter().enumerate() {
        let packet = UdpChunkPacketRef {
            kind: UDP_CHUNK_KIND,
            msg_id: &msg_id,
            idx,
            total,
            data,
        };
        let serialized =
            serde_json::to_string(&packet).context("failed to serialize udp chunk packet")?;
        let err_context = format!("udp send failed: {} (chunk {}/{})", addr, idx + 1, total);
        send_udp_datagram_with_retry(
            &sock,
            addr.as_str(),
            serialized.as_bytes(),
            err_context.as_str(),
        )?;
        if UDP_CHUNK_SEND_PAUSE_EVERY > 0
            && (idx + 1) % UDP_CHUNK_SEND_PAUSE_EVERY == 0
            && idx + 1 < total
        {
            thread::sleep(Duration::from_millis(UDP_CHUNK_SEND_PAUSE_MS));
        }
    }
    Ok(())
}

fn send_udp_datagram_with_retry(
    sock: &UdpSocket,
    addr: &str,
    buf: &[u8],
    err_context: &str,
) -> Result<()> {
    let mut last_err: Option<std::io::Error> = None;
    for attempt in 1..=UDP_SEND_RETRY_TIMES {
        match sock.send_to(buf, addr) {
            Ok(_) => return Ok(()),
            Err(err) => {
                last_err = Some(err);
                if attempt < UDP_SEND_RETRY_TIMES {
                    thread::sleep(Duration::from_millis(UDP_SEND_RETRY_MS));
                }
            }
        }
    }
    let last_err = last_err.ok_or_else(|| anyhow!("{err_context}: unknown error"))?;
    Err(anyhow!("{err_context}: {last_err}"))
}

fn split_utf8_chunks<'a>(s: &'a str, max_bytes: usize) -> Result<Vec<&'a str>> {
    if max_bytes == 0 {
        return Err(anyhow!("split_utf8_chunks max_bytes must be > 0"));
    }
    if s.is_empty() {
        return Ok(vec![""]);
    }

    let mut chunks = Vec::new();
    let mut start = 0usize;
    while start < s.len() {
        let mut end = (start + max_bytes).min(s.len());
        while end > start && !s.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            return Err(anyhow!("failed to split utf-8 payload at byte offset {}", start));
        }
        chunks.push(&s[start..end]);
        start = end;
    }
    Ok(chunks)
}
