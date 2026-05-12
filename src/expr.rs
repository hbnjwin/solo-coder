use anyhow::{bail, Result};
use std::collections::HashMap;

pub fn evaluate_expression(
    expr: &str,
    attrs: &HashMap<String, serde_json::Value>,
) -> Result<bool> {
    let expr = expr.trim();

    if let Some(or_pos) = find_top_level_or(expr) {
        let left = &expr[..or_pos];
        let right = &expr[or_pos + 2..];
        return Ok(evaluate_expression(left.trim(), attrs)? || evaluate_expression(right.trim(), attrs)?);
    }

    if let Some(and_pos) = find_top_level_and(expr) {
        let left = &expr[..and_pos];
        let right = &expr[and_pos + 3..];
        return Ok(evaluate_expression(left.trim(), attrs)? && evaluate_expression(right.trim(), attrs)?);
    }

    if expr.starts_with('(') && expr.ends_with(')') {
        return evaluate_expression(&expr[1..expr.len() - 1], attrs);
    }

    evaluate_comparison(expr, attrs)
}

fn find_top_level_or(expr: &str) -> Option<usize> {
    let mut depth = 0i32;
    let chars: Vec<char> = expr.chars().collect();
    for i in 0..chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => depth -= 1,
            'O' | 'o' if depth == 0 && i + 1 < chars.len() && (chars[i + 1] == 'R' || chars[i + 1] == 'r') => {
                let before = if i > 0 { chars[i - 1] } else { ' ' };
                let after = if i + 2 < chars.len() { chars[i + 2] } else { ' ' };
                if !before.is_alphanumeric() && !after.is_alphanumeric() {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_top_level_and(expr: &str) -> Option<usize> {
    let mut depth = 0i32;
    let bytes = expr.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'A' | b'a' if depth == 0 && i + 2 < bytes.len() && expr[i..i + 3].eq_ignore_ascii_case("AND") => {
                let before = if i > 0 { expr.as_bytes()[i - 1] } else { b' ' };
                let after = if i + 3 < bytes.len() { bytes[i + 3] } else { b' ' };
                if !before.is_ascii_alphanumeric() && !after.is_ascii_alphanumeric() {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn evaluate_comparison(expr: &str, attrs: &HashMap<String, serde_json::Value>) -> Result<bool> {
    let operators = [">=", "<=", "!=", ">", "<", "==", "="];
    for op in &operators {
        if let Some(pos) = find_operator(expr, op) {
            let left = expr[..pos].trim();
            let right = expr[pos + op.len()..].trim();
            let left_val = resolve_value(left, attrs)?;
            let right_val = resolve_value(right, attrs)?;
            return compare_values(&left_val, &right_val, op);
        }
    }
    bail!("no comparison operator found in expression: {}", expr)
}

fn find_operator(expr: &str, op: &str) -> Option<usize> {
    let mut depth = 0i32;
    let bytes = expr.as_bytes();
    let op_bytes = op.as_bytes();
    let mut i = 0;
    while i <= bytes.len().saturating_sub(op_bytes.len()) {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if depth == 0 && expr[i..].starts_with(op) => {
                if op == ">" || op == "<" || op == "=" {
                    if i + 1 < bytes.len() && (bytes[i + 1] == b'=' || bytes[i + 1] == b'!') {
                        i += 1;
                        continue;
                    }
                }
                return Some(i);
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn resolve_value(token: &str, attrs: &HashMap<String, serde_json::Value>) -> Result<serde_json::Value> {
    let token = token.trim();
    if token.starts_with('"') && token.ends_with('"') {
        return Ok(serde_json::Value::String(token[1..token.len() - 1].to_string()));
    }
    if let Ok(n) = token.parse::<f64>() {
        return Ok(serde_json::Value::from(n));
    }
    if let Some(val) = attrs.get(token) {
        return Ok(val.clone());
    }
    bail!("cannot resolve value: {}", token)
}

fn compare_values(left: &serde_json::Value, right: &serde_json::Value, op: &str) -> Result<bool> {
    match op {
        "==" | "=" => Ok(left == right),
        "!=" => Ok(left != right),
        ">" => compare_numeric(left, right, |a, b| a > b),
        "<" => compare_numeric(left, right, |a, b| a < b),
        ">=" => compare_numeric(left, right, |a, b| a >= b),
        "<=" => compare_numeric(left, right, |a, b| a <= b),
        _ => bail!("unsupported operator: {}", op),
    }
}

fn compare_numeric(
    left: &serde_json::Value,
    right: &serde_json::Value,
    cmp: fn(f64, f64) -> bool,
) -> Result<bool> {
    let l = left.as_f64().ok_or_else(|| anyhow::anyhow!("left operand is not a number"))?;
    let r = right.as_f64().ok_or_else(|| anyhow::anyhow!("right operand is not a number"))?;
    Ok(cmp(l, r))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_attrs() -> HashMap<String, serde_json::Value> {
        let mut m = HashMap::new();
        m.insert("amount".to_string(), json!(15000));
        m.insert("department".to_string(), json!("finance"));
        m.insert("level".to_string(), json!(3));
        m.insert("name".to_string(), json!("alice"));
        m
    }

    #[test]
    fn test_simple_gt() {
        let attrs = make_attrs();
        assert!(evaluate_expression("amount > 10000", &attrs).unwrap());
        assert!(!evaluate_expression("amount > 20000", &attrs).unwrap());
    }

    #[test]
    fn test_string_equals() {
        let attrs = make_attrs();
        assert!(evaluate_expression("department == \"finance\"", &attrs).unwrap());
        assert!(!evaluate_expression("department == \"hr\"", &attrs).unwrap());
    }

    #[test]
    fn test_and_expression() {
        let attrs = make_attrs();
        assert!(evaluate_expression("amount > 10000 AND department == \"finance\"", &attrs).unwrap());
        assert!(!evaluate_expression("amount > 20000 AND department == \"finance\"", &attrs).unwrap());
    }

    #[test]
    fn test_or_expression() {
        let attrs = make_attrs();
        assert!(evaluate_expression("amount > 20000 OR department == \"finance\"", &attrs).unwrap());
        assert!(!evaluate_expression("amount > 20000 OR department == \"hr\"", &attrs).unwrap());
    }

    #[test]
    fn test_parenthesized() {
        let attrs = make_attrs();
        assert!(evaluate_expression("(amount > 10000 OR level > 5) AND department == \"finance\"", &attrs).unwrap());
    }

    #[test]
    fn test_nested_parens() {
        let attrs = make_attrs();
        assert!(evaluate_expression("((amount > 10000))", &attrs).unwrap());
    }

    #[test]
    fn test_gte_lte() {
        let attrs = make_attrs();
        assert!(evaluate_expression("amount >= 15000", &attrs).unwrap());
        assert!(evaluate_expression("amount <= 15000", &attrs).unwrap());
        assert!(!evaluate_expression("amount >= 20000", &attrs).unwrap());
    }

    #[test]
    fn test_ne() {
        let attrs = make_attrs();
        assert!(evaluate_expression("department != \"hr\"", &attrs).unwrap());
    }
}
