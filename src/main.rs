use anyhow::{anyhow, Context, Result};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use clap::Parser;
use configparser::ini::Ini;
use csv::WriterBuilder;
use mysql::{prelude::Queryable, OptsBuilder, Pool, Value as MyValue};
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

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
    data_sources: Vec<DataSource>,
}

#[derive(Debug)]
struct ExportResult {
    row_count: usize,
    csv_file: PathBuf,
    json_file: PathBuf,
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

/// Replace date placeholders in SQL with actual time range based on last_run_at
fn apply_incremental_sql(cfg: &mut RuntimeConfig) -> Result<()> {
    let now = Local::now();
    let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    for ds in &mut cfg.data_sources {
        if !ds.enabled { continue; }
        for query in &mut ds.queries {
            let last_run = read_last_run_at(&ds.section_name, &query.key);
            let from_time = last_run.as_deref().unwrap_or("1970-01-01 00:00:00");
            query.sql = replace_time_range(&query.sql, from_time, &now_str);
            println!(
                "[incremental] {}::{} range: {} ~ {}",
                ds.section_name, query.key, from_time, now_str
            );
        }
    }
    Ok(())
}

/// Try to replace BETWEEN 'xxx' AND 'yyy' patterns in SQL
fn replace_time_range(sql: &str, from: &str, to: &str) -> String {
    let re_str = r#"(?i)BETWEEN\s+'[^']*'\s+AND\s+'[^']*'"#;
    let re = regex::Regex::new(re_str).unwrap();
    re.replace_all(sql, format!("BETWEEN '{}' AND '{}'", from, to)).to_string()
}

/// Read last_run_at from config file for a given section + query key
fn read_last_run_at(section: &str, query_key: &str) -> Option<String> {
    let config_path = PathBuf::from("config.ini");
    if !config_path.exists() { return None; }
    let mut conf = Ini::new();
    conf.load(config_path.to_str()?).ok()?;
    conf.get(section, &format!("last_run_at_{}", query_key))
}

/// Update last_run_at timestamp in config file
fn update_last_run_at(config_path: &Path) -> Result<()> {
    if !config_path.exists() { return Ok(()); }
    let mut conf = Ini::new();
    let load_path = config_path.to_str().ok_or_else(|| anyhow!("config path is not valid UTF-8"))?;
    conf.load(load_path).map_err(|e| anyhow!("failed to load ini config: {e}"))?;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let map = conf.get_map_ref();
    for (section_name, props) in map {
        if !section_name.starts_with("data-") { continue; }
        for (key, _) in props {
            if key.starts_with("sql_") {
                let last_run_key = format!("last_run_at_{}", key);
                conf.set(section_name.as_str(), &last_run_key, Some(now.clone()));
            }
        }
    }
    conf.write(load_path)
        .map_err(|e| anyhow!("failed to write config: {e}"))?;
    println!("[incremental] updated last_run_at in config");
    Ok(())
}

// --- query & export ---

fn run_query_and_export(ds: &DataSource, query: &QueryJob, output_dir: &Path) -> Result<ExportResult> {
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
    s.chars().map(|ch| if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' { ch } else { '_' }).collect()
}

fn export_postgres(ds: &DataSource, query: &QueryJob, csv_path: &Path, json_path: &Path) -> Result<ExportResult> {
    let db_name = ds.database.as_deref().unwrap_or("postgres");
    let conn_str = format!(
        "host='{}' port={} user='{}' password='{}' dbname='{}' connect_timeout=10",
        escape_pg_conn_value(&ds.host), ds.port,
        escape_pg_conn_value(&ds.user), escape_pg_conn_value(&ds.password),
        escape_pg_conn_value(db_name)
    );
    let mut client = Client::connect(&conn_str, NoTls)
        .with_context(|| format!("postgres connect failed for [{}]", ds.section_name))?;
    let stmt = client.prepare(query.sql.as_str())
        .with_context(|| format!("postgres prepare failed for query {}", query.key))?;
    let headers: Vec<String> = stmt.columns().iter().map(|c| c.name().to_string()).collect();
    let rows = client.query(&stmt, &[])
        .with_context(|| format!("postgres query failed for query {}", query.key))?;
    let mut row_values: Vec<Vec<String>> = Vec::with_capacity(rows.len());
    for row in &rows {
        let mut values = Vec::with_capacity(headers.len());
        for i in 0..headers.len() { values.push(pg_cell_to_string(row, i)); }
        row_values.push(values);
    }
    write_csv(csv_path, &headers, &row_values)?;
    write_json(json_path, &headers, &row_values)?;
    Ok(ExportResult { row_count: rows.len(), csv_file: csv_path.to_path_buf(), json_file: json_path.to_path_buf() })
}

fn export_mysql(ds: &DataSource, query: &QueryJob, csv_path: &Path, json_path: &Path) -> Result<ExportResult> {
    let mut opts = OptsBuilder::new()
        .ip_or_hostname(Some(ds.host.clone())).tcp_port(ds.port)
        .user(Some(ds.user.clone())).pass(Some(ds.password.clone()));
    if let Some(db) = &ds.database { opts = opts.db_name(Some(db.clone())); }
    let pool = Pool::new(opts)
        .with_context(|| format!("mysql pool creation failed for [{}] {}:{}", ds.section_name, ds.host, ds.port))?;
    let mut conn = pool.get_conn()
        .with_context(|| format!("mysql connect failed for [{}]", ds.section_name))?;
    let mut result = conn.query_iter(query.sql.as_str())
        .with_context(|| format!("mysql query failed for query {}", query.key))?;
    let columns = result.columns();
    let headers: Vec<String> = columns.as_ref().iter().map(|c| c.name_str().to_string()).collect();
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
    drop(result); drop(conn); drop(pool);
    write_csv(csv_path, &headers, &row_values)?;
    write_json(json_path, &headers, &row_values)?;
    Ok(ExportResult { row_count, csv_file: csv_path.to_path_buf(), json_file: json_path.to_path_buf() })
}

// --- file output ---

fn write_csv(csv_path: &Path, headers: &[String], rows: &[Vec<String>]) -> Result<()> {
    let file = File::create(csv_path).with_context(|| format!("failed to create csv {}", csv_path.display()))?;
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.write_record(headers).with_context(|| format!("failed to write csv header {}", csv_path.display()))?;
    for row in rows { wtr.write_record(row).with_context(|| format!("failed to write csv row {}", csv_path.display()))?; }
    wtr.flush().with_context(|| format!("failed to flush csv {}", csv_path.display()))?;
    Ok(())
}

fn write_json(json_path: &Path, headers: &[String], rows: &[Vec<String>]) -> Result<()> {
    let mut records: Vec<JsonValue> = Vec::with_capacity(rows.len());
    for row in rows {
        let mut obj = JsonMap::with_capacity(headers.len());
        for (col, val) in headers.iter().zip(row.iter()) { obj.insert(col.clone(), JsonValue::String(val.clone())); }
        records.push(JsonValue::Object(obj));
    }
    let file = File::create(json_path).with_context(|| format!("failed to create json {}", json_path.display()))?;
    serde_json::to_writer_pretty(file, &records).with_context(|| format!("failed to write json {}", json_path.display()))?;
    Ok(())
}

// --- cell conversion ---

fn pg_cell_to_string(row: &postgres::Row, idx: usize) -> String {
    if let Ok(v) = row.try_get::<usize, Option<String>>(idx) { return v.unwrap_or_default(); }
    if let Ok(v) = row.try_get::<usize, Option<i64>>(idx) { return v.map(|x| x.to_string()).unwrap_or_default(); }
    if let Ok(v) = row.try_get::<usize, Option<f64>>(idx) { return v.map(|x| x.to_string()).unwrap_or_default(); }
    if let Ok(v) = row.try_get::<usize, Option<bool>>(idx) { return v.map(|x| x.to_string()).unwrap_or_default(); }
    if let Ok(v) = row.try_get::<usize, Option<NaiveDateTime>>(idx) {
        return v.map(|x| x.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default();
    }
    if let Ok(v) = row.try_get::<usize, Option<NaiveDate>>(idx) {
        return v.map(|x| x.format("%Y-%m-%d").to_string()).unwrap_or_default();
    }
    if let Ok(v) = row.try_get::<usize, Option<JsonValue>>(idx) { return v.map(|x| x.to_string()).unwrap_or_default(); }
    format!("<unsupported:{}>", row.columns()[idx].type_().name())
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
            if micros > 0 { format!("{y:04}-{m:02}-{d:02} {hh:02}:{mm:02}:{ss:02}.{:06}", micros) }
            else { format!("{y:04}-{m:02}-{d:02} {hh:02}:{mm:02}:{ss:02}") }
        }
        MyValue::Time(is_neg, days, hh, mm, ss, micros) => {
            let sign = if is_neg { "-" } else { "" };
            if micros > 0 { format!("{sign}{days} {hh:02}:{mm:02}:{ss:02}.{:06}", micros) }
            else { format!("{sign}{days} {hh:02}:{mm:02}:{ss:02}") }
        }
    }
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

fn load_runtime_config(path: &Path) -> Result<RuntimeConfig> {
    if !path.exists() { return Err(anyhow!("config file not found: {}", path.display())); }
    let mut conf = Ini::new();
    let load_path = path.to_str().ok_or_else(|| anyhow!("config path is not valid UTF-8"))?;
    conf.load(load_path).map_err(|e| anyhow!("failed to load ini config: {e}"))?;
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
    Ok(RuntimeConfig { data_sources })
}
