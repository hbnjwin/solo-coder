use anyhow::{anyhow, Result};
use chrono::Local;
use configparser::ini::Ini;
use std::path::{Path, PathBuf};

use crate::config::RuntimeConfig;

pub fn apply_incremental_sql(cfg: &mut RuntimeConfig) -> Result<()> {
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

pub fn replace_time_range(sql: &str, from: &str, to: &str) -> String {
    let re_str = r#"(?i)BETWEEN\s+'[^']*'\s+AND\s+'[^']*'"#;
    let re = regex::Regex::new(re_str).unwrap();
    re.replace_all(sql, format!("BETWEEN '{}' AND '{}'", from, to)).to_string()
}

fn read_last_run_at(section: &str, query_key: &str) -> Option<String> {
    let config_path = PathBuf::from("config.ini");
    if !config_path.exists() { return None; }
    let mut conf = Ini::new();
    conf.load(config_path.to_str()?).ok()?;
    conf.get(section, &format!("last_run_at_{}", query_key))
}

pub fn update_last_run_at(config_path: &Path) -> Result<()> {
    if !config_path.exists() { return Ok(()); }
    let mut conf = Ini::new();
    let load_path = config_path.to_str().ok_or_else(|| anyhow!("config path is not valid UTF-8"))?;
    conf.load(load_path).map_err(|e| anyhow!("failed to load ini config: {e}"))?;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let updates: Vec<(String, String)> = {
        let map = conf.get_map_ref();
        let mut out = Vec::new();
        for (section_name, props) in map {
            if !section_name.starts_with("data-") { continue; }
            for (key, _) in props {
                if key.starts_with("sql_") {
                    out.push((section_name.clone(), format!("last_run_at_{}", key)));
                }
            }
        }
        out
    };
    for (section_name, last_run_key) in updates {
        conf.set(section_name.as_str(), &last_run_key, Some(now.clone()));
    }
    conf.write(load_path)
        .map_err(|e| anyhow!("failed to write config: {e}"))?;
    println!("[incremental] updated last_run_at in config");
    Ok(())
}
