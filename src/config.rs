use anyhow::{anyhow, Context, Result};
use configparser::ini::Ini;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum DbType { PostgreSql, MySql }

impl DbType {
    pub fn as_str(&self) -> &'static str {
        match self { DbType::PostgreSql => "postgres", DbType::MySql => "mysql" }
    }
}

#[derive(Debug, Clone)]
pub struct QueryJob { pub key: String, pub sql: String }

#[derive(Debug, Clone)]
pub struct DataSource {
    pub section_name: String,
    pub enabled: bool,
    pub db_type: DbType,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: Option<String>,
    pub queries: Vec<QueryJob>,
}

#[derive(Debug)]
pub struct RuntimeConfig {
    pub data_sources: Vec<DataSource>,
}

pub fn resolve_config_path(default: &Path, env: Option<&str>) -> Result<PathBuf> {
    if let Some(env_name) = env {
        let env_path = default.with_file_name(format!("config.{}.ini", env_name));
        if env_path.exists() {
            return Ok(env_path);
        }
        eprintln!("[cfg] env config {} not found, falling back to {}", env_path.display(), default.display());
    }
    if !default.exists() {
        return Err(anyhow!("config file not found: {}", default.display()));
    }
    Ok(default.to_path_buf())
}

pub fn parse_bool(s: &str) -> bool {
    matches!(s.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "y" | "on")
}

pub fn get_optional(props: &std::collections::HashMap<String, Option<String>>, key: &str) -> Option<String> {
    props.get(key).and_then(|v| v.clone()).filter(|v| !v.is_empty())
}

pub fn get_required(props: &std::collections::HashMap<String, Option<String>>, key: &str, section: &str) -> Result<String> {
    get_optional(props, key).ok_or_else(|| anyhow!("missing required key `{}` in section [{}]", key, section))
}

pub fn format_error(err: &anyhow::Error) -> String { format!("{:#}", err) }

pub fn escape_pg_conn_value(value: &str) -> String { value.replace('\\', "\\\\").replace('\'', "\\'") }

pub fn sanitize(s: &str) -> String {
    s.chars().map(|ch| if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' { ch } else { '_' }).collect()
}

pub fn load_runtime_config(path: &Path) -> Result<RuntimeConfig> {
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
