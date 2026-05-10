use anyhow::{Context, Result};
use mysql::{prelude::Queryable, OptsBuilder, Pool};
use postgres::{Client, NoTls};

use crate::config::{DataSource, DbType, escape_pg_conn_value, format_error};

#[derive(Debug, Default)]
pub struct HealthCheckSummary {
    pub total_count: usize,
    pub ok_count: usize,
    pub failed_items: Vec<String>,
}

pub fn startup_health_check(cfg: &crate::config::RuntimeConfig) -> HealthCheckSummary {
    println!("=== Startup Status ===");
    let mut summary = HealthCheckSummary::default();
    for ds in &cfg.data_sources {
        if !ds.enabled {
            println!("[{}] enable=0 status=SKIPPED", ds.section_name);
            continue;
        }
        summary.total_count += 1;
        match check_database_connection(ds) {
            Ok(_) => {
                summary.ok_count += 1;
                println!("[{}] enable=1 type={} target={}:{} status=OK", ds.section_name, ds.db_type.as_str(), ds.host, ds.port);
            }
            Err(err) => {
                let msg = format!("[{}] enable=1 type={} target={}:{} status=FAILED error={}",
                    ds.section_name, ds.db_type.as_str(), ds.host, ds.port, format_error(&err));
                println!("{msg}");
                summary.failed_items.push(msg);
            }
        }
    }
    println!("Startup status summary: {}/{} passed", summary.ok_count, summary.total_count);
    summary
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
        escape_pg_conn_value(&ds.host), ds.port,
        escape_pg_conn_value(&ds.user), escape_pg_conn_value(&ds.password),
        escape_pg_conn_value(db_name)
    );
    let mut client = Client::connect(&conn_str, NoTls)
        .with_context(|| format!("postgres connect failed for [{}]", ds.section_name))?;
    let _rows = client.query("SELECT 1", &[])
        .with_context(|| format!("postgres ping query failed for [{}]", ds.section_name))?;
    Ok(())
}

fn check_mysql_connection(ds: &DataSource) -> Result<()> {
    let mut opts = OptsBuilder::new()
        .ip_or_hostname(Some(ds.host.clone())).tcp_port(ds.port)
        .user(Some(ds.user.clone())).pass(Some(ds.password.clone()));
    if let Some(db) = &ds.database { opts = opts.db_name(Some(db.clone())); }
    let pool = Pool::new(opts)
        .with_context(|| format!("mysql pool creation failed for [{}]", ds.section_name))?;
    let mut conn = pool.get_conn()
        .with_context(|| format!("mysql connect failed for [{}]", ds.section_name))?;
    let _ping: Option<u8> = conn.query_first("SELECT 1")
        .with_context(|| format!("mysql ping query failed for [{}]", ds.section_name))?;
    Ok(())
}
