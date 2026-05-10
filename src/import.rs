use anyhow::{anyhow, Context, Result};
use csv::ReaderBuilder;
use mysql::{prelude::Queryable, OptsBuilder, Pool};
use postgres::{Client, NoTls};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::config::{DataSource, DbType, QueryJob};
use crate::sql_util::*;

#[derive(Debug)]
pub struct CsvFileMeta {
    pub path: std::path::PathBuf,
    pub file_name: String,
    pub headers: Vec<String>,
}

#[derive(Debug)]
pub struct ImportOutcome {
    pub file: std::path::PathBuf,
    pub total_rows: usize,
    pub inserted_rows: usize,
    pub skipped_rows: usize,
}

pub fn discover_csv_files(import_dir: &Path) -> Result<Vec<CsvFileMeta>> {
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

pub fn import_one_query_from_csv(
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

fn get_pg_table_columns(client: &mut Client, schema: &str, table: &str) -> Result<HashSet<String>> {
    let sql = "SELECT column_name FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2";
    let rows = client.query(sql, &[&schema, &table])
        .with_context(|| format!("postgres read table columns failed: {}.{}", schema, table))?;
    Ok(rows.into_iter().filter_map(|r| r.try_get::<usize, String>(0).ok()).map(|x| x.to_ascii_lowercase()).collect::<HashSet<_>>())
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

fn get_mysql_table_columns(conn: &mut mysql::PooledConn, schema: &str, table: &str) -> Result<HashSet<String>> {
    let q = format!("SELECT column_name FROM information_schema.columns WHERE table_schema='{}' AND table_name='{}'",
        schema.replace('\'', "''"), table.replace('\'', "''"));
    let cols: Vec<String> = conn.query(q)
        .with_context(|| format!("mysql read table columns failed: {}.{}", schema, table))?;
    Ok(cols.into_iter().map(|x| x.to_ascii_lowercase()).collect::<HashSet<_>>())
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
