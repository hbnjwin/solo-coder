use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub id: String,
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub rule_id: String,
    pub passed: bool,
    pub message: String,
}

pub fn evaluate_rule(rule: &ValidationRule, record: &HashMap<String, serde_json::Value>) -> ValidationResult {
    let field_val = record.get(&rule.field);
    match field_val {
        None => ValidationResult { rule_id: rule.id.clone(), passed: false, message: format!("field '{}' not found", rule.field) },
        Some(val) => {
            let passed = match rule.operator.as_str() {
                "gt" => val.as_f64().map_or(false, |v| v > rule.value.as_f64().unwrap_or(0.0)),
                "lt" => val.as_f64().map_or(false, |v| v < rule.value.as_f64().unwrap_or(0.0)),
                "eq" => val == &rule.value,
                "not_empty" => !val.to_string().is_empty(),
                _ => false,
            };
            ValidationResult { rule_id: rule.id.clone(), passed, message: if passed { "OK".into() } else { format!("{} {} {:?} failed", rule.field, rule.operator, rule.value) } }
        }
    }
}

/// TODO: add RuleChain with and/or combinations and short-circuit
pub fn validate_record(rules: &[ValidationRule], record: &HashMap<String, serde_json::Value>) -> Vec<ValidationResult> {
    rules.iter().map(|r| evaluate_rule(r, record)).collect()
}

#[derive(Parser, Debug)]
#[command(name = "report-validator", about = "Report validation rule engine")]
struct Cli {}

fn main() -> Result<()> {
    let rules = vec![
        ValidationRule { id: "r1".into(), field: "amount".into(), operator: "gt".into(), value: serde_json::json!(0) },
        ValidationRule { id: "r2".into(), field: "name".into(), operator: "not_empty".into(), value: serde_json::json!("") },
    ];
    let mut record = HashMap::new();
    record.insert("amount".into(), serde_json::json!(-100));
    record.insert("name".into(), serde_json::json!(""));
    let results = validate_record(&rules, &record);
    for r in &results {
        println!("{}: {} - {}", r.rule_id, if r.passed { "PASS" } else { "FAIL" }, r.message);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rule_chain_not_supported() {
        let rules = vec![
            ValidationRule { id: "r1".into(), field: "amount".into(), operator: "gt".into(), value: serde_json::json!(0) },
            ValidationRule { id: "r2".into(), field: "amount".into(), operator: "lt".into(), value: serde_json::json!(1000000) },
        ];
        let mut rec = HashMap::new();
        rec.insert("amount".into(), serde_json::json!(5000000));
        let results = validate_record(&rules, &rec);
        assert_eq!(results.len(), 2);
        assert!(results[0].passed);
        assert!(!results[1].passed);
    }
}
