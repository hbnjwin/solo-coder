use crate::config::RuntimeConfig;

pub struct HealthCheckResult {
    pub ok_count: usize,
    pub total_count: usize,
    pub failed_items: Vec<String>,
}

pub fn startup_health_check(cfg: &RuntimeConfig) -> HealthCheckResult {
    let mut ok_count = 0;
    let mut total_count = 0;
    let mut failed_items = Vec::new();

    for ds in cfg.data_sources.iter().filter(|d| d.enabled) {
        total_count += 1;

        let result = match ds.db_type {
            crate::config::DbType::PostgreSql => {
                let conn_str = format!("host={} port={} user={} password='{}'{}", ds.host, ds.port, ds.user, ds.password,
                    ds.database.as_ref().map(|d| format!(" dbname={}", d)).unwrap_or_default());
                match postgres::Client::connect(&conn_str, postgres::NoTls) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            crate::config::DbType::MySql => {
                let conn_str = format!("mysql://{}:{}@{}:{}/{}", ds.user, ds.password, ds.host, ds.port,
                    ds.database.as_ref().map(|d| d.as_str()).unwrap_or_default());
                match mysql::Opts::from_url(&conn_str) {
                    Ok(opts) => match mysql::Pool::new(opts) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("{}", e)),
                    },
                    Err(e) => Err(format!("{}", e)),
                }
            }
        };

        match result {
            Ok(()) => {
                ok_count += 1;
                println!("  [OK] {} ({})", ds.section_name, ds.db_type.as_str());
            }
            Err(e) => {
                let msg = format!("  [FAIL] {} ({}): {}", ds.section_name, ds.db_type.as_str(), e);
                eprintln!("{}", msg);
                failed_items.push(msg);
            }
        }
    }

    HealthCheckResult { ok_count, total_count, failed_items }
}
