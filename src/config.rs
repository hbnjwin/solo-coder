use anyhow::{anyhow, Context, Result};
use configparser::ini::Ini;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}

impl Endpoint {
    pub fn addr(&self) -> String { format!("{}:{}", self.host, self.port) }
}

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
    pub tcp: Option<Endpoint>,
    pub udp: Option<Endpoint>,
    pub udp_chunk_size: usize,
    pub data_sources: Vec<DataSource>,
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

pub fn build_output_file_base(source: &str, query_key: &str) -> String {
    let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
    format!("{}_{}_{}", sanitize(source), sanitize(query_key), ts)
}

const UDP_CHUNK_DATA_DEFAULT_BYTES: usize = 512;

pub fn parse_endpoint(conf: &Ini, section: &str) -> Result<Option<Endpoint>> {
    let enabled_raw = conf.get(section, "enable");
    if enabled_raw.is_none() { return Ok(None); }
    let enabled = parse_bool(enabled_raw.as_deref().unwrap_or("0"));
    let host = conf.get(section, "host").unwrap_or_default();
    let port: u16 = conf.get(section, "port").unwrap_or_default().parse()
        .with_context(|| format!("invalid port in section [{}]", section))?;
    Ok(Some(Endpoint { enabled, host, port }))
}

pub fn parse_udp_chunk_size(conf: &Ini) -> Result<usize> {
    let Some(raw) = conf.get("paraset", "chunksize") else { return Ok(UDP_CHUNK_DATA_DEFAULT_BYTES); };
    let parsed: usize = raw.parse().with_context(|| format!("invalid [paraset] chunkSize `{raw}`"))?;
    if !(64..=1024).contains(&parsed) {
        return Err(anyhow!("invalid [paraset] chunkSize {}; allowed range is 64..=1024 bytes", parsed));
    }
    Ok(parsed)
}

pub fn load_runtime_config(path: &Path) -> Result<RuntimeConfig> {
    if !path.exists() { return Err(anyhow!("config file not found: {}", path.display())); }
    let mut conf = Ini::new();
    let load_path = path.to_str().ok_or_else(|| anyhow!("config path is not valid UTF-8"))?;
    conf.load(load_path).map_err(|e| anyhow!("failed to load ini config: {e}"))?;
    let tcp = parse_endpoint(&conf, "tcpserver")?;
    let udp = parse_endpoint(&conf, "udpserver")?;
    let udp_chunk_size = parse_udp_chunk_size(&conf)?;
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
    Ok(RuntimeConfig { tcp, udp, udp_chunk_size, data_sources })
}
