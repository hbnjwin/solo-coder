use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTemplate {
    pub id: String,
    pub sql: String,
    pub params: HashMap<String, String>,
}

/// BUG: directly interpolates user input into SQL string - injection risk!
pub fn parse_template(template: &QueryTemplate) -> String {
    let mut sql = template.sql.clone();
    for (key, value) in &template.params {
        sql = sql.replace(&format!("{{{}}}", key), value);
    }
    sql
}

/// BUG: no validation at all
pub fn validate_param_value(_value: &str) -> Result<()> {
    Ok(())
}

/// TODO: not yet implemented - should use $1, $2 placeholders
pub fn build_parameterized_query(_template: &QueryTemplate) -> Result<(String, Vec<String>)> {
    anyhow::bail!("not implemented")
}

#[derive(Parser, Debug)]
#[command(name = "query-def-safety", about = "SQL query template engine")]
struct Cli {
    #[arg(long, default_value = "SELECT * FROM contracts WHERE dept = {dept}")]
    template: String,
    #[arg(long, default_value = "finance")]
    dept: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut params = HashMap::new();
    params.insert("dept".into(), cli.dept.clone());
    let tmpl = QueryTemplate { id: "q1".into(), sql: cli.template, params };
    let sql = parse_template(&tmpl);
    println!("generated SQL: {}", sql);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sql_injection_vulnerability() {
        let mut params = HashMap::new();
        params.insert("dept".into(), "finance; DROP TABLE contracts; --".into());
        let tmpl = QueryTemplate { id: "q1".into(), sql: "SELECT * FROM t WHERE dept = {dept}".into(), params };
        let sql = parse_template(&tmpl);
        assert!(sql.contains("DROP TABLE"), "injection should be blocked but isn't");
    }
    #[test]
    fn test_validate_allows_everything() {
        assert!(validate_param_value("'; DROP TABLE x; --").is_ok());
    }
}
