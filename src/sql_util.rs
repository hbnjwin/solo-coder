use anyhow::{anyhow, Result};

pub fn extract_target_table_from_sql(sql: &str) -> Result<String> {
    let raw = sql.trim().trim_end_matches(';');
    let lower = raw.to_lowercase();
    let from_pos = lower.find(" from ").or_else(|| lower.find("from "))
        .ok_or_else(|| anyhow!("sql has no FROM clause: {}", raw))?;
    let after_from = if lower[from_pos..].starts_with(" from ") { &raw[(from_pos + 6)..] } else { &raw[(from_pos + 5)..] };
    let table = after_from.trim_start().split_whitespace().next()
        .ok_or_else(|| anyhow!("cannot parse table name from sql: {}", raw))?;
    Ok(table.trim_matches('`').trim_matches('"').to_string())
}

pub fn split_schema_table(full: &str, default_schema: &str) -> (String, String) {
    if let Some((schema, table)) = full.split_once('.') {
        (schema.trim_matches('`').trim_matches('"').to_string(), table.trim_matches('`').trim_matches('"').to_string())
    } else {
        (default_schema.trim_matches('`').trim_matches('"').to_string(), full.trim_matches('`').trim_matches('"').to_string())
    }
}

pub fn quote_pg_ident(s: &str) -> String { format!("\"{}\"", s.replace('"', "\"\"")) }
pub fn quote_pg_table(schema: &str, table: &str) -> String {
    if schema.is_empty() { quote_pg_ident(table) } else { format!("{}.{}", quote_pg_ident(schema), quote_pg_ident(table)) }
}
pub fn quote_mysql_ident(s: &str) -> String { format!("`{}`", s.replace('`', "``")) }
pub fn quote_mysql_table(schema: &str, table: &str) -> String {
    if schema.is_empty() { quote_mysql_ident(table) } else { format!("{}.{}", quote_mysql_ident(schema), quote_mysql_ident(table)) }
}
pub fn pg_value_literal(v: &str) -> String { sql_value_literal(v) }
pub fn mysql_value_literal(v: &str) -> String { sql_value_literal(v) }
pub fn sql_value_literal(v: &str) -> String {
    let trimmed = v.trim();
    if trimmed.is_empty() { return "NULL".to_string(); }
    if trimmed.starts_with("<unsupported:") && trimmed.ends_with('>') { return "NULL".to_string(); }
    let escaped = v.replace('\\', "\\\\").replace('\'', "''");
    format!("'{}'", escaped)
}
pub fn escape_pg_conn_value(value: &str) -> String { value.replace('\\', "\\\\").replace('\'', "\\'") }
pub fn format_postgres_insert_error(err: &postgres::Error) -> String {
    if let Some(db) = err.as_db_error() {
        let mut msg = db.message().to_string();
        if let Some(detail) = db.detail() { msg.push_str(" | detail: "); msg.push_str(detail); }
        if let Some(table) = db.table() { msg.push_str(" | table: "); msg.push_str(table); }
        return msg;
    }
    err.to_string()
}
