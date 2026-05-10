use anyhow::{anyhow, Context, Result};
use chrono::Local;
use clap::Parser;
use configparser::ini::Ini;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

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

#[derive(Debug, Clone)]
struct Endpoint {
    enabled: bool,
    host: String,
    port: u16,
}

impl Endpoint {
    fn addr(&self) -> String { format!("{}:{}", self.host, self.port) }
}

#[derive(Debug, Clone)]
enum DbType { PostgreSql, MySql }
impl DbType {
    fn as_str(&self) -> &'static str {
        match self { DbType::PostgreSql => "postgres", DbType::MySql => "mysql" }
    }
}

#[derive(Debug, Clone)]
struct QueryJob { key: String, sql: String }

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

#[derive(Debug)]
struct ExportResult {
    row_count: usize,
    csv_file: PathBuf,
    json_file: PathBuf,
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
const UDP_CHUNK_SEND_RETRY_TIMES: usize = 3;
const UDP_CHUNK_SEND_RETRY_MS: u64 = 2;
static UDP_MSG_SEQ: AtomicU64 = AtomicU64::new(1);

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

// --- TCP/UDP send ---

fn send_report(cfg: &RuntimeConfig, report: &QueryReport) {
    let payload = match serde_json::to_string(report) {
        Ok(s) => s,
        Err(err) => { eprintln!("failed to serialize report: {err}"); return; }
    };
    if let Some(tcp) = &cfg.tcp {
        if tcp.enabled {
            match send_tcp(tcp, &payload) {
                Ok(_) => println!("  report sent via tcp to {}", tcp.addr()),
                Err(err) => eprintln!("  report tcp send failed: {}", format_error(&err)),
            }
        }
    }
    if let Some(udp) = &cfg.udp {
        if udp.enabled {
            match send_udp(udp, &payload, cfg.udp_chunk_size) {
                Ok(_) => println!("  report sent via udp to {}", udp.addr()),
                Err(err) => eprintln!("  report udp send failed: {}", format_error(&err)),
            }
        }
    }
}

fn send_tcp(endpoint: &Endpoint, payload: &str) -> Result<()> {
    let addr = endpoint.addr();
    let sock_addr = addr.to_socket_addrs()
        .with_context(|| format!("resolve tcp address failed `{}`", addr))?
        .next()
        .ok_or_else(|| anyhow!("resolve tcp address failed `{}`", addr))?;
    let mut stream = TcpStream::connect_timeout(&sock_addr, Duration::from_secs(5))
        .with_context(|| format!("tcp connect failed: {}", addr))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))
        .context("tcp set_write_timeout failed")?;
    let mut line = payload.to_string();
    line.push('\n');
    stream.write_all(line.as_bytes())
        .with_context(|| format!("tcp write failed: {}", addr))?;
    stream.flush().context("tcp flush failed")?;
    Ok(())
}

fn send_udp(endpoint: &Endpoint, payload: &str, chunk_size: usize) -> Result<()> {
    let addr = endpoint.addr();
    let sock = UdpSocket::bind("0.0.0.0:0").context("udp bind failed")?;
    sock.connect(addr.as_str())
        .with_context(|| format!("udp connect failed: {}", addr))?;

    if payload.len() <= UDP_DIRECT_MAX_BYTES {
        let mut tried = 0;
        loop {
            tried += 1;
            match sock.send(payload.as_bytes()) {
                Ok(_) => return Ok(()),
                Err(e) if tried < UDP_CHUNK_SEND_RETRY_TIMES => {
                    eprintln!("[udp] send retry {}/{} error={}", tried, UDP_CHUNK_SEND_RETRY_TIMES, e);
                    std::thread::sleep(Duration::from_millis(UDP_CHUNK_SEND_RETRY_MS));
                }
                Err(e) => return Err(e).context("udp send failed after retries"),
            }
        }
    }

    // chunked send
    let msg_id = format!("{}_{}", Local::now().format("%Y%m%d%H%M%S"), UDP_MSG_SEQ.fetch_add(1, Ordering::Relaxed));
    let total = (payload.len() + chunk_size - 1) / chunk_size;
    for (idx, chunk) in payload.as_bytes().chunks(chunk_size).enumerate() {
        let chunk_data = String::from_utf8_lossy(chunk).to_string();
        let packet = serde_json::json!({
            "kind": "udp_chunk_v1",
            "msg_id": msg_id,
            "idx": idx,
            "total": total,
            "data": chunk_data,
        });
        let line = serde_json::to_string(&packet).context("failed to serialize udp chunk")?;
        let mut tried = 0;
        loop {
            tried += 1;
            match sock.send(line.as_bytes()) {
                Ok(_) => break,
                Err(e) if tried < UDP_CHUNK_SEND_RETRY_TIMES => {
                    eprintln!("[udp chunk] send retry {}/{} idx={}/{} error={}", tried, UDP_CHUNK_SEND_RETRY_TIMES, idx+1, total, e);
                    std::thread::sleep(Duration::from_millis(UDP_CHUNK_SEND_RETRY_MS));
                }
                Err(e) => return Err(e).context("udp chunk send failed after retries"),
            }
        }
        if (idx + 1) % 8 == 0 {
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    Ok(())
}

fn send_transport_test_packet(cfg: &RuntimeConfig) -> Result<()> {
    let payload = format!(
        "{{\"kind\":\"transport_test\",\"timestamp\":\"{}\",\"message\":\"hello from tcp-reconnect\"}}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    let mut tried = 0usize;
    let mut ok = 0usize;
    let mut errors = Vec::new();
    if let Some(tcp) = &cfg.tcp {
        if tcp.enabled {
            tried += 1;
            match send_tcp(tcp, &payload) {
                Ok(_) => { ok += 1; println!("[tcpserver] enable=1 target={} send_test=OK", tcp.addr()); }
                Err(err) => { let msg = format!("[tcpserver] enable=1 target={} send_test=FAILED error={}", tcp.addr(), format_error(&err)); println!("{msg}"); errors.push(msg); }
            }
        }
    }
    if let Some(udp) = &cfg.udp {
        if udp.enabled {
            tried += 1;
            match send_udp(udp, &payload, cfg.udp_chunk_size) {
                Ok(_) => { ok += 1; println!("[udpserver] enable=1 target={} send_test=OK", udp.addr()); }
                Err(err) => { let msg = format!("[udpserver] enable=1 target={} send_test=FAILED error={}", udp.addr(), format_error(&err)); println!("{msg}"); errors.push(msg); }
            }
        }
    }
    if tried == 0 { return Err(anyhow!("no enabled tcp/udp endpoint found")); }
    if ok == tried { println!("Transport test passed: {ok}/{tried}"); return Ok(()); }
    Err(anyhow!("transport test failed: {ok}/{tried} passed\n{}", errors.join("\n")))
}

// --- query & export (stub — uses postgres/mysql) ---

fn run_query_and_export(ds: &DataSource, query: &QueryJob, output_dir: &Path) -> Result<ExportResult> {
    let file_base = build_output_file_base(&ds.section_name, &query.key);
    let csv_path = output_dir.join(format!("{file_base}.csv"));
    let json_path = output_dir.join(format!("{file_base}.json"));
    match ds.db_type {
        DbType::PostgreSql => {
            let db_name = ds.database.as_deref().unwrap_or("postgres");
            let conn_str = format!(
                "host='{}' port={} user='{}' password='{}' dbname='{}' connect_timeout=10",
                escape_pg_conn_value(&ds.host), ds.port,
                escape_pg_conn_value(&ds.user), escape_pg_conn_value(&ds.password),
                escape_pg_conn_value(db_name)
            );
            let mut client = postgres::Client::connect(&conn_str, postgres::NoTls)
                .with_context(|| format!("postgres connect failed for [{}]", ds.section_name))?;
            let rows = client.query(query.sql.as_str(), &[])
                .with_context(|| format!("postgres query failed for query {}", query.key))?;
            let row_count = rows.len();
            // simplified: write empty output
            let mut f = fs::File::create(&csv_path)?;
            writeln!(f, "row_count")?;
            writeln!(f, "{}", row_count)?;
            let mut f = fs::File::create(&json_path)?;
            writeln!(f, "{{\"row_count\": {}}}", row_count)?;
            Ok(ExportResult { row_count, csv_file: csv_path, json_file: json_path })
        }
        DbType::MySql => {
            let mut opts = mysql::OptsBuilder::new()
                .ip_or_hostname(Some(ds.host.clone())).tcp_port(ds.port)
                .user(Some(ds.user.clone())).pass(Some(ds.password.clone()));
            if let Some(db) = &ds.database { opts = opts.db_name(Some(db.clone())); }
            let pool = mysql::Pool::new(opts)
                .with_context(|| format!("mysql pool creation failed for [{}]", ds.section_name))?;
            let mut conn = pool.get_conn()
                .with_context(|| format!("mysql connect failed for [{}]", ds.section_name))?;
            let result = conn.query_iter(query.sql.as_str())
                .with_context(|| format!("mysql query failed for query {}", query.key))?;
            let row_count = result.affected_rows();
            drop(result); drop(conn); drop(pool);
            let mut f = fs::File::create(&csv_path)?;
            writeln!(f, "row_count")?;
            writeln!(f, "{}", row_count)?;
            let mut f = fs::File::create(&json_path)?;
            writeln!(f, "{{\"row_count\": {}}}", row_count)?;
            Ok(ExportResult { row_count: row_count as usize, csv_file: csv_path, json_file: json_path })
        }
    }
}

fn build_output_file_base(source: &str, query_key: &str) -> String {
    let ts = Local::now().format("%Y%m%d_%H%M%S");
    format!("{}_{}_{}", sanitize(source), sanitize(query_key), ts)
}

fn sanitize(s: &str) -> String {
    s.chars().map(|ch| if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' { ch } else { '_' }).collect()
}

// --- utility ---

fn escape_pg_conn_value(value: &str) -> String { value.replace('\\', "\\\\").replace('\'', "\\'") }
fn format_error(err: &anyhow::Error) -> String { format!("{:#}", err) }
fn parse_bool(s: &str) -> bool {
    matches!(s.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "y" | "on")
}
fn get_optional(props: &std::collections::HashMap<String, Option<String>>, key: &str) -> Option<String> {
    props.get(key).and_then(|v| v.clone()).filter(|v| !v.is_empty())
}
fn get_required(props: &std::collections::HashMap<String, Option<String>>, key: &str, section: &str) -> Result<String> {
    get_optional(props, key).ok_or_else(|| anyhow!("missing required key `{}` in section [{}]", key, section))
}

fn parse_endpoint(conf: &Ini, section: &str) -> Result<Option<Endpoint>> {
    let enabled_raw = conf.get(section, "enable");
    if enabled_raw.is_none() { return Ok(None); }
    let enabled = parse_bool(enabled_raw.as_deref().unwrap_or("0"));
    let host = conf.get(section, "host").unwrap_or_default();
    let port: u16 = conf.get(section, "port").unwrap_or_default().parse()
        .with_context(|| format!("invalid port in section [{}]", section))?;
    Ok(Some(Endpoint { enabled, host, port }))
}

fn parse_udp_chunk_size(conf: &Ini) -> Result<usize> {
    let Some(raw) = conf.get("paraset", "chunksize") else { return Ok(UDP_CHUNK_DATA_DEFAULT_BYTES); };
    let parsed: usize = raw.parse().with_context(|| format!("invalid [paraset] chunkSize `{raw}`"))?;
    if !(64..=1024).contains(&parsed) {
        return Err(anyhow!("invalid [paraset] chunkSize {}; allowed range is 64..=1024 bytes", parsed));
    }
    Ok(parsed)
}

fn load_runtime_config(path: &Path) -> Result<RuntimeConfig> {
    if !path.exists() { return Err(anyhow!("config file not found: {}", path.display())); }
    let mut conf = Ini::new();
    let load_path = path.to_str().ok_or_else(|| anyhow!("config path is not valid UTF-8"))?;
    conf.load(load_path).map_err(|e| anyhow!("failed to load ini config: {e}"))?;
    let tcp = parse_endpoint(&conf, "tcpserver")?;
    let udp = parse_endpoint(&conf, "udpserver")?;
    let udp_chunk_size = parse_udp_chunk_size(&conf)?;
    let mut data_sources = Vec::new();
    let map = conf.get_map_ref();
    for (section_name, props) in map {
        if !section_name.starts_with("data-") { continue; }
        let enabled = parse_bool(get_optional(props, "enable").as_deref().unwrap_or("1"));
        let db_type = match get_required(props, "type", section_name)?.to_lowercase().as_str() {
            "pg" | "postgres" | "postgresql" => DbType::PostgreSql,
            "mysql" => DbType::MySql,
            other => return Err(anyhow!("unsupported db type `{}` in section [{}]", other, section_name)),
        };
        let host = get_required(props, "host", section_name)?;
        let port: u16 = get_required(props, "port", section_name)?.parse()
            .with_context(|| format!("invalid port in section [{}]", section_name))?;
        let user = get_required(props, "user", section_name)?;
        let password = get_optional(props, "password").unwrap_or_default();
        let database = get_optional(props, "database");
        let mut query_keys: Vec<String> = props.iter()
            .filter_map(|(k, v)| {
                let value = v.as_deref().unwrap_or("").trim();
                (k.starts_with("sql_") && !value.is_empty()).then(|| k.clone())
            }).collect();
        query_keys.sort();
        if query_keys.is_empty() { return Err(anyhow!("no sql_* query found in section [{}]", section_name)); }
        let queries = query_keys.into_iter().map(|k| QueryJob {
            key: k.clone(), sql: props.get(&k).and_then(|v| v.clone()).unwrap_or_default(),
        }).collect();
        data_sources.push(DataSource { section_name: section_name.clone(), enabled, db_type, host, port, user, password, database, queries });
    }
    if data_sources.is_empty() { return Err(anyhow!("no data-* section found in config")); }
    Ok(RuntimeConfig { tcp, udp, udp_chunk_size, data_sources })
}
