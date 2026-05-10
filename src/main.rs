use anyhow::{anyhow, Context, Result};
use chrono::Local;
use clap::Parser;
use configparser::ini::Ini;
use csv::{ReaderBuilder, WriterBuilder};
use mysql::{prelude::Queryable, OptsBuilder, Pool, Value as MyValue};
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::collections::HashSet;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

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

fn run_batch_csv_import(cfg: &RuntimeConfig, import_dir: &Path) -> Result<()> {
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

fn discover_csv_files(import_dir: &Path) -> Result<Vec<CsvFileMeta>> {
    if !import_dir.exists() {
        return Err(anyhow!("import dir not found: {}", import_dir.display()));
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(import_dir).with_context(|| format!("failed to read dir {}", import_dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("csv") { continue; }
        let file_name = match path.file_name().and_then(|x| x.to_str()) {
            Some(x) => x.to_string(), None => continue,
        };
        let mut rdr = ReaderBuilder::new().has_headers(true).from_path(&path)
            .with_context(|| format!("failed to open csv {}", path.display()))?;
        let headers = rdr.headers()
            .with_context(|| format!("failed to read csv header {}", path.display()))?
            .iter().map(|x| x.to_string()).collect();
        out.push(CsvFileMeta { path, file_name, headers });
    }
    Ok(out)
}

fn import_one_query_from_csv(
    ds: &DataSource, query: &QueryJob, csv_files: &[CsvFileMeta],
) -> Result<ImportOutcome> {
    let table_full = extract_target_table_from_sql(&query.sql)
        .with_context(|| format!("failed to parse target table for query {}", query.key))?;
    match ds.db_type {
        DbType::PostgreSql => import_one_query_csv_pg(ds, query, &table_full, csv_files),
        DbType::MySql => import_one_query_csv_mysql(ds, query, &table_full, csv_files),
    }
}

fn import_one_query_csv_pg(
    ds: &DataSource, query: &QueryJob, table_full: &str, csv_files: &[CsvFileMeta],
) -> Result<ImportOutcome> {
    let db_name = ds.database.as_deref().unwrap_or("postgres");
    let conn_str = format!(
        "host='{}' port={} user='{}' password='{}' dbname='{}' connect_timeout=10",
        escape_pg_conn_value(&ds.host), ds.port,
        escape_pg_conn_value(&ds.user), escape_pg_conn_value(&ds.password),
        escape_pg_conn_value(db_name)
    );
    let mut client = Client::connect(&conn_str, NoTls)
        .with_context(|| format!("postgres connect failed for [{}]", ds.section_name))?;

    let (schema, table) = split_schema_table(table_full, "public");
    let (schema_used, table_cols) = resolve_pg_table_columns(&mut client, &schema, &table)?;
    let selected = choose_csv_for_query(csv_files, &query.key, &table_cols)?;

    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(&selected.path)
        .with_context(|| format!("failed to open csv {}", selected.path.display()))?;
    let headers = rdr.headers()
        .with_context(|| format!("failed to read csv header {}", selected.path.display()))?
        .iter().map(|x| x.to_string()).collect::<Vec<_>>();

    let table_quoted = quote_pg_table(&schema_used, &table);
    let columns_sql = headers.iter().map(|h| quote_pg_ident(h)).collect::<Vec<_>>().join(",");

    let mut total = 0usize;
    let mut inserted = 0usize;
    let mut skipped = 0usize;
    let mut skipped_logged = 0usize;
    for rec in rdr.records() {
        let rec = rec.with_context(|| format!("failed to read csv row {}", selected.path.display()))?;
        total += 1;
        let values_sql = rec.iter().map(pg_value_literal).collect::<Vec<_>>().join(",");
        let sql = format!("INSERT INTO {} ({}) VALUES ({}) ON CONFLICT DO NOTHING", table_quoted, columns_sql, values_sql);
        match client.execute(sql.as_str(), &[]) {
            Ok(n) => inserted += n as usize,
            Err(err) => {
                skipped += 1;
                if skipped_logged < 3 {
                    eprintln!("  - {} WARN row {} skipped: {}", query.key, total, format_postgres_insert_error(&err));
                    skipped_logged += 1;
                }
            }
        }
    }
    if skipped > 3 {
        eprintln!("  - {} WARN skipped rows total={} (only first 3 shown)", query.key, skipped);
    }
    Ok(ImportOutcome { file: selected.path.clone(), total_rows: total, inserted_rows: inserted, skipped_rows: skipped })
}

fn import_one_query_csv_mysql(
    ds: &DataSource, query: &QueryJob, table_full: &str, csv_files: &[CsvFileMeta],
) -> Result<ImportOutcome> {
    let mut opts = OptsBuilder::new()
        .ip_or_hostname(Some(ds.host.clone())).tcp_port(ds.port)
        .user(Some(ds.user.clone())).pass(Some(ds.password.clone()));
    if let Some(db) = &ds.database { opts = opts.db_name(Some(db.clone())); }
    let pool = Pool::new(opts)
        .with_context(|| format!("mysql pool creation failed for [{}] {}:{}", ds.section_name, ds.host, ds.port))?;
    let mut conn = pool.get_conn()
        .with_context(|| format!("mysql connect failed for [{}]", ds.section_name))?;

    let default_schema = ds.database.as_deref().unwrap_or("");
    let (schema, table) = split_schema_table(table_full, default_schema);
    let (schema_used, table_cols) = resolve_mysql_table_columns(&mut conn, &schema, &table)?;
    let selected = choose_csv_for_query(csv_files, &query.key, &table_cols)?;

    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(&selected.path)
        .with_context(|| format!("failed to open csv {}", selected.path.display()))?;
    let headers = rdr.headers()
        .with_context(|| format!("failed to read csv header {}", selected.path.display()))?
        .iter().map(|x| x.to_string()).collect::<Vec<_>>();

    let table_quoted = quote_mysql_table(&schema_used, &table);
    let columns_sql = headers.iter().map(|h| quote_mysql_ident(h)).collect::<Vec<_>>().join(",");

    let mut total = 0usize;
    let mut inserted = 0usize;
    let mut skipped = 0usize;
    let mut skipped_logged = 0usize;
    for rec in rdr.records() {
        let rec = rec.with_context(|| format!("failed to read csv row {}", selected.path.display()))?;
        total += 1;
        let values_sql = rec.iter().map(mysql_value_literal).collect::<Vec<_>>().join(",");
        let sql = format!("INSERT IGNORE INTO {} ({}) VALUES ({})", table_quoted, columns_sql, values_sql);
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
        eprintln!("  - {} WARN skipped rows total={} (only first 3 shown)", query.key, skipped);
    }
    Ok(ImportOutcome { file: selected.path.clone(), total_rows: total, inserted_rows: inserted, skipped_rows: skipped })
}

// --- helper functions ---

fn extract_target_table_from_sql(sql: &str) -> Result<String> {
    let raw = sql.trim().trim_end_matches(';');
    let lower = raw.to_lowercase();
    let from_pos = lower.find(" from ").or_else(|| lower.find("from "))
        .ok_or_else(|| anyhow!("sql has no FROM clause: {}", raw))?;
    let after_from = if lower[from_pos..].starts_with(" from ") { &raw[(from_pos + 6)..] } else { &raw[(from_pos + 5)..] };
    let table = after_from.trim_start().split_whitespace().next()
        .ok_or_else(|| anyhow!("cannot parse table name from sql: {}", raw))?;
    Ok(table.trim_matches('`').trim_matches('"').to_string())
}

fn split_schema_table(full: &str, default_schema: &str) -> (String, String) {
    if let Some((schema, table)) = full.split_once('.') {
        (schema.trim_matches('`').trim_matches('"').to_string(), table.trim_matches('`').trim_matches('"').to_string())
    } else {
        (default_schema.trim_matches('`').trim_matches('"').to_string(), full.trim_matches('`').trim_matches('"').to_string())
    }
}

fn get_pg_table_columns(client: &mut Client, schema: &str, table: &str) -> Result<HashSet<String>> {
    let sql = "SELECT column_name FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2";
    let rows = client.query(sql, &[&schema, &table])
        .with_context(|| format!("postgres read table columns failed: {}.{}", schema, table))?;
    Ok(rows.into_iter().filter_map(|r| r.try_get::<usize, String>(0).ok()).map(|x| x.to_ascii_lowercase()).collect::<HashSet<_>>())
}

fn resolve_pg_table_columns(client: &mut Client, schema: &str, table: &str) -> Result<(String, HashSet<String>)> {
    let direct = get_pg_table_columns(client, schema, table)?;
    if !direct.is_empty() { return Ok((schema.to_string(), direct)); }
    let sql = "SELECT DISTINCT table_schema FROM information_schema.columns WHERE table_name = $1";
    let rows = client.query(sql, &[&table])
        .with_context(|| format!("postgres resolve table schema failed: {}", table))?;
    let schemas = rows.into_iter().filter_map(|r| r.try_get::<usize, String>(0).ok()).collect::<Vec<_>>();
    if schemas.is_empty() { return Err(anyhow!("target table not found: {}.{}", schema, table)); }
    if schemas.len() > 1 { return Err(anyhow!("target table `{}` exists in multiple schemas {:?}", table, schemas)); }
    let picked = schemas[0].clone();
    let cols = get_pg_table_columns(client, picked.as_str(), table)?;
    if cols.is_empty() { return Err(anyhow!("target table not found after resolve: {}.{}", picked, table)); }
    Ok((picked, cols))
}

fn get_mysql_table_columns(conn: &mut mysql::PooledConn, schema: &str, table: &str) -> Result<HashSet<String>> {
    let q = format!("SELECT column_name FROM information_schema.columns WHERE table_schema='{}' AND table_name='{}'",
        schema.replace('\'', "''"), table.replace('\'', "''"));
    let cols: Vec<String> = conn.query(q)
        .with_context(|| format!("mysql read table columns failed: {}.{}", schema, table))?;
    Ok(cols.into_iter().map(|x| x.to_ascii_lowercase()).collect::<HashSet<_>>())
}

fn resolve_mysql_table_columns(conn: &mut mysql::PooledConn, schema: &str, table: &str) -> Result<(String, HashSet<String>)> {
    let direct = get_mysql_table_columns(conn, schema, table)?;
    if !direct.is_empty() { return Ok((schema.to_string(), direct)); }
    let q = format!("SELECT DISTINCT table_schema FROM information_schema.columns WHERE table_name='{}'", table.replace('\'', "''"));
    let schemas: Vec<String> = conn.query(q)
        .with_context(|| format!("mysql resolve table schema failed: {}", table))?;
    if schemas.is_empty() { return Err(anyhow!("target table not found: {}.{}", schema, table)); }
    if schemas.len() > 1 { return Err(anyhow!("target table `{}` exists in multiple schemas {:?}", table, schemas)); }
    let picked = schemas[0].clone();
    let cols = get_mysql_table_columns(conn, picked.as_str(), table)?;
    if cols.is_empty() { return Err(anyhow!("target table not found after resolve: {}.{}", picked, table)); }
    Ok((picked, cols))
}

fn choose_csv_for_query<'a>(csv_files: &'a [CsvFileMeta], query_key: &str, table_cols_lower: &HashSet<String>) -> Result<&'a CsvFileMeta> {
    let token = format!("_{}_", query_key);
    let mut full_matches: Vec<_> = csv_files.iter()
        .filter(|f| f.file_name.contains(&token))
        .filter(|f| !f.headers.is_empty() && f.headers.iter().all(|h| table_cols_lower.contains(&h.to_ascii_lowercase())))
        .collect();
    if !full_matches.is_empty() {
        full_matches.sort_by_key(|f| extract_timestamp_rank(&f.file_name));
        return Ok(*full_matches.last().expect("full_matches should not be empty"));
    }
    Err(anyhow!("no compatible csv found for query key `{}`", query_key))
}

fn extract_timestamp_rank(file_name: &str) -> String {
    let base = file_name.strip_suffix(".csv").unwrap_or(file_name);
    let mut iter = base.rsplit('_');
    let t = iter.next().unwrap_or("");
    let d = iter.next().unwrap_or("");
    if d.len() == 8 && t.len() == 6 { format!("{}_{}", d, t) } else { base.to_string() }
}

fn quote_pg_ident(s: &str) -> String { format!("\"{}\"", s.replace('"', "\"\"")) }
fn quote_pg_table(schema: &str, table: &str) -> String {
    if schema.is_empty() { quote_pg_ident(table) } else { format!("{}.{}", quote_pg_ident(schema), quote_pg_ident(table)) }
}
fn quote_mysql_ident(s: &str) -> String { format!("`{}`", s.replace('`', "``")) }
fn quote_mysql_table(schema: &str, table: &str) -> String {
    if schema.is_empty() { quote_mysql_ident(table) } else { format!("{}.{}", quote_mysql_ident(schema), quote_mysql_ident(table)) }
}
fn pg_value_literal(v: &str) -> String { sql_value_literal(v) }
fn mysql_value_literal(v: &str) -> String { sql_value_literal(v) }
fn sql_value_literal(v: &str) -> String {
    let trimmed = v.trim();
    if trimmed.is_empty() { return "NULL".to_string(); }
    if trimmed.starts_with("<unsupported:") && trimmed.ends_with('>') { return "NULL".to_string(); }
    let escaped = v.replace('\\', "\\\\").replace('\'', "''");
    format!("'{}'", escaped)
}
fn escape_pg_conn_value(value: &str) -> String { value.replace('\\', "\\\\").replace('\'', "\\'") }
fn format_error(err: &anyhow::Error) -> String { format!("{:#}", err) }
fn format_postgres_insert_error(err: &postgres::Error) -> String {
    if let Some(db) = err.as_db_error() {
        let mut msg = db.message().to_string();
        if let Some(detail) = db.detail() { msg.push_str(" | detail: "); msg.push_str(detail); }
        if let Some(table) = db.table() { msg.push_str(" | table: "); msg.push_str(table); }
        return msg;
    }
    err.to_string()
}
fn parse_bool(s: &str) -> bool {
    matches!(s.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "y" | "on")
}
fn get_optional(props: &std::collections::HashMap<String, Option<String>>, key: &str) -> Option<String> {
    props.get(key).and_then(|v| v.clone()).filter(|v| !v.is_empty())
}
fn get_required(props: &std::collections::HashMap<String, Option<String>>, key: &str, section: &str) -> Result<String> {
    get_optional(props, key).ok_or_else(|| anyhow!("missing required key `{}` in section [{}]", key, section))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_target_table_simple() {
        let sql = "SELECT * FROM my_table";
        assert_eq!(extract_target_table_from_sql(sql).unwrap(), "my_table");
    }

    #[test]
    fn test_extract_target_table_quoted() {
        let sql = "SELECT * FROM `schema`.`table`";
        assert_eq!(extract_target_table_from_sql(sql).unwrap(), "schema.table");
    }

    #[test]
    fn test_extract_target_table_double_quoted() {
        let sql = r#"SELECT * FROM "public"."users""#;
        assert_eq!(extract_target_table_from_sql(sql).unwrap(), "public.users");
    }

    #[test]
    fn test_extract_target_table_no_from() {
        let sql = "SELECT 1";
        assert!(extract_target_table_from_sql(sql).is_err());
    }

    #[test]
    fn test_sql_value_literal_null() {
        assert_eq!(sql_value_literal(""), "NULL");
        assert_eq!(sql_value_literal("  "), "NULL");
    }

    #[test]
    fn test_sql_value_literal_unsupported() {
        assert_eq!(sql_value_literal("<unsupported:uuid>"), "NULL");
    }

    #[test]
    fn test_sql_value_literal_string() {
        assert_eq!(sql_value_literal("hello"), "'hello'");
    }

    #[test]
    fn test_sql_value_literal_escape() {
        assert_eq!(sql_value_literal("it's"), "'it''s'");
        assert_eq!(sql_value_literal("path\\dir"), "'path\\\\dir'");
    }

    #[test]
    fn test_split_schema_table_with_dot() {
        let (s, t) = split_schema_table("public.users", "public");
        assert_eq!(s, "public");
        assert_eq!(t, "users");
    }

    #[test]
    fn test_split_schema_table_without_dot() {
        let (s, t) = split_schema_table("users", "public");
        assert_eq!(s, "public");
        assert_eq!(t, "users");
    }

    #[test]
    fn test_choose_csv_for_query_match() {
        let files = vec![CsvFileMeta {
            path: PathBuf::from("/tmp/data_sql_test_20260101_120000.csv"),
            file_name: "data_sql_test_20260101_120000.csv".to_string(),
            headers: vec!["id".to_string(), "name".to_string()],
        }];
        let mut cols = HashSet::new();
        cols.insert("id".to_string());
        cols.insert("name".to_string());
        let result = choose_csv_for_query(&files, "sql_test", &cols);
        assert!(result.is_ok());
    }

    #[test]
    fn test_choose_csv_for_query_no_match() {
        let files = vec![CsvFileMeta {
            path: PathBuf::from("/tmp/other.csv"),
            file_name: "other.csv".to_string(),
            headers: vec!["id".to_string()],
        }];
        let cols = HashSet::new();
        let result = choose_csv_for_query(&files, "sql_test", &cols);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_timestamp_rank() {
        assert_eq!(extract_timestamp_rank("data_sql_test_20260101_120000.csv"), "20260101_120000");
        assert_eq!(extract_timestamp_rank("no_date.csv"), "no_date");
    }
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
