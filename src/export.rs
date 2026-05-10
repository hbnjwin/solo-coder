use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveDateTime};
use csv::WriterBuilder;
use mysql::{prelude::Queryable, OptsBuilder, Pool, Value as MyValue};
use postgres::{Client, NoTls};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::fs::File;
use std::path::Path;

use crate::config::{DataSource, DbType, QueryJob, build_output_file_base, escape_pg_conn_value};

#[derive(Debug)]
pub struct ExportResult {
    pub row_count: usize,
    pub csv_file: std::path::PathBuf,
    pub json_file: std::path::PathBuf,
}

pub fn run_query_and_export(ds: &DataSource, query: &QueryJob, output_dir: &Path) -> Result<ExportResult> {
    let file_base = build_output_file_base(&ds.section_name, &query.key);
    let csv_path = output_dir.join(format!("{file_base}.csv"));
    let json_path = output_dir.join(format!("{file_base}.json"));
    match ds.db_type {
        DbType::PostgreSql => export_postgres(ds, query, &csv_path, &json_path),
        DbType::MySql => export_mysql(ds, query, &csv_path, &json_path),
    }
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
