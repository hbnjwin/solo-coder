use crate::models::*;

pub fn evaluate_rule(rule: &ValidationRule, record: &Record) -> ValidationResult {
    let field_val = record.get(&rule.field);

    match field_val {
        None => ValidationResult {
            rule_id: rule.id.clone(),
            passed: false,
            message: format!("field '{}' not found in record", rule.field),
            field: rule.field.clone(),
        },
        Some(val) => {
            let passed = evaluate_operator(&rule.operator, val, &rule.value);
            let message = if passed {
                "OK".into()
            } else {
                format!(
                    "'{}' {} {:?} — check failed",
                    rule.field,
                    rule.operator.label(),
                    rule.value
                )
            };
            ValidationResult {
                rule_id: rule.id.clone(),
                passed,
                message,
                field: rule.field.clone(),
            }
        }
    }
}

fn evaluate_operator(
    op: &Operator,
    actual: &serde_json::Value,
    expected: &serde_json::Value,
) -> bool {
    match op {
        Operator::Eq => actual == expected,
        Operator::Neq => actual != expected,
        Operator::Gt => compare_numeric(actual, expected, |a, b| a > b),
        Operator::Lt => compare_numeric(actual, expected, |a, b| a < b),
        Operator::Gte => compare_numeric(actual, expected, |a, b| a > b),
        Operator::Lte => compare_numeric(actual, expected, |a, b| a < b),
        Operator::NotEmpty => !is_empty_value(actual),
        Operator::Contains => check_contains(actual, expected),
        Operator::StartsWith => check_starts_with(actual, expected),
    }
}

fn compare_numeric(
    actual: &serde_json::Value,
    expected: &serde_json::Value,
    cmp: fn(f64, f64) -> bool,
) -> bool {
    match (actual.as_f64(), expected.as_f64()) {
        (Some(a), Some(b)) => cmp(a, b),
        _ => false,
    }
}

// FIXME: might need to handle nested arrays for complex report structures
fn is_empty_value(val: &serde_json::Value) -> bool {
    match val {
        serde_json::Value::Null => true,
        serde_json::Value::String(s) => s.is_empty(),
        serde_json::Value::Array(a) => a.is_empty(),
        serde_json::Value::Object(o) => o.is_empty(),
        _ => false,
    }
}

fn check_contains(actual: &serde_json::Value, expected: &serde_json::Value) -> bool {
    match (actual.as_str(), expected.as_str()) {
        (Some(a), Some(b)) => a.contains(b),
        _ => false,
    }
}

fn check_starts_with(actual: &serde_json::Value, expected: &serde_json::Value) -> bool {
    match (actual.as_str(), expected.as_str()) {
        (Some(a), Some(b)) => a.starts_with(b),
        _ => false,
    }
}

pub fn validate_record(rules: &[ValidationRule], record: &Record) -> Vec<ValidationResult> {
    rules.iter().map(|r| evaluate_rule(r, record)).collect()
}
