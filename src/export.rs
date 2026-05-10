use anyhow::{Context, Result};
use mysql::prelude::Queryable;
use std::fs;
use std::io::Write;
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
