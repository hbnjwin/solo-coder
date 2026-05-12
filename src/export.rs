use anyhow::Result;
use std::path::PathBuf;

use crate::config::{DataSource, QueryJob};

pub struct ExportResult {
    pub row_count: usize,
    pub csv_file: std::path::PathBuf,
    pub json_file: std::path::PathBuf,
}

pub fn run_query_and_export(ds: &DataSource, query: &QueryJob, output_dir: &PathBuf) -> Result<ExportResult> {
    let csv_path = output_dir.join(format!("{}_{}.csv", sanitize(&ds.section_name), sanitize(&query.key)));
    let json_path = output_dir.join(format!("{}_{}.json", sanitize(&ds.section_name), sanitize(&query.key)));

    let row_count = match ds.db_type {
        crate::config::DbType::PostgreSql => {
            let conn_str = format!("host={} port={} user={} password='{}'{}", ds.host, ds.port, ds.user, ds.password,
                ds.database.as_ref().map(|d| format!(" dbname={}", d)).unwrap_or_default());
            export_postgres(&conn_str, &query.sql, &csv_path, &json_path)?
        }
        crate::config::DbType::MySql => {
            let conn_str = format!("mysql://{}:{}@{}:{}/{}", ds.user, ds.password, ds.host, ds.port,
                ds.database.as_ref().map(|d| d.as_str()).unwrap_or_default());
            export_mysql(&conn_str, &query.sql, &csv_path, &json_path)?
        }
    };

    Ok(ExportResult {
        row_count,
        csv_file: csv_path,
        json_file: json_path,
    })
}

fn sanitize(s: &str) -> String {
    s.chars().map(|ch| if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' { ch } else { '_' }).collect()
}

fn export_postgres(conn_str: &str, sql: &str, csv_path: &PathBuf, json_path: &PathBuf) -> Result<usize> {
    let mut client = postgres::Client::connect(conn_str, postgres::NoTls)
        .map_err(|e| anyhow::anyhow!("postgres connect failed: {}", e))?;
    let rows = client.query(sql, &[]).map_err(|e| anyhow::anyhow!("postgres query failed: {}", e))?;
    let row_count = rows.len();

    if !rows.is_empty() {
        let columns: Vec<String> = rows[0].columns().iter().map(|c| c.name().to_string()).collect();
        let mut wtr = csv::Writer::from_path(csv_path)?;
        wtr.write_record(&columns)?;
        for row in &rows {
            let record: Vec<String> = columns.iter().map(|col| {
                let val: Option<String> = row.get(col.as_str());
                val.unwrap_or_default()
            }).collect();
            wtr.write_record(&record)?;
        }
        wtr.flush()?;

        let json_data: Vec<serde_json::Map<String, serde_json::Value>> = rows.iter().map(|row| {
            let mut map = serde_json::Map::new();
            for col in &columns {
                let val: Option<String> = row.get(col.as_str());
                map.insert(col.clone(), serde_json::Value::String(val.unwrap_or_default()));
            }
            map
        }).collect();
        let json_str = serde_json::to_string_pretty(&json_data)?;
        std::fs::write(json_path, json_str)?;
    }

    Ok(row_count)
}

fn export_mysql(conn_str: &str, sql: &str, csv_path: &PathBuf, json_path: &PathBuf) -> Result<usize> {
    let opts = mysql::Opts::from_url(conn_str)
        .map_err(|e| anyhow::anyhow!("invalid mysql connection string: {}", e))?;
    let pool = mysql::Pool::new(opts)
        .map_err(|e| anyhow::anyhow!("mysql connect failed: {}", e))?;
    let mut conn = pool.get_conn()
        .map_err(|e| anyhow::anyhow!("mysql get connection failed: {}", e))?;

    use mysql::prelude::Queryable;
    let rows: Vec<mysql::Row> = conn.query(sql)
        .map_err(|e| anyhow::anyhow!("mysql query failed: {}", e))?;
    let row_count = rows.len();

    if !rows.is_empty() {
        let columns: Vec<String> = rows[0].columns_ref().iter().map(|c| c.name_str().to_string()).collect();
        let mut wtr = csv::Writer::from_path(csv_path)?;
        wtr.write_record(&columns)?;
        for row in &rows {
            let record: Vec<String> = columns.iter().enumerate().map(|(i, _)| {
                row.get::<String, usize>(i).unwrap_or_default()
            }).collect();
            wtr.write_record(&record)?;
        }
        wtr.flush()?;

        let json_data: Vec<serde_json::Map<String, serde_json::Value>> = rows.iter().map(|row| {
            let mut map = serde_json::Map::new();
            for (i, col) in columns.iter().enumerate() {
                let val: Option<String> = row.get(i);
                map.insert(col.clone(), serde_json::Value::String(val.unwrap_or_default()));
            }
            map
        }).collect();
        let json_str = serde_json::to_string_pretty(&json_data)?;
        std::fs::write(json_path, json_str)?;
    }

    Ok(row_count)
}
