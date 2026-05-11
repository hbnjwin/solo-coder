use report_validator::chain::evaluate_chain;
use report_validator::evaluator::{evaluate_rule, validate_record};
use report_validator::models::*;
use std::collections::HashMap;

fn make_record(fields: Vec<(&str, serde_json::Value)>) -> Record {
    let mut record = HashMap::new();
    for (k, v) in fields {
        record.insert(k.to_string(), v);
    }
    record
}

#[test]
fn test_basic_validation() {
    let rules = vec![
        ValidationRule::new("r1", "amount", Operator::Gt, serde_json::json!(0)),
        ValidationRule::new("r2", "title", Operator::NotEmpty, serde_json::json!("")),
    ];
    let record = make_record(vec![
        ("amount", serde_json::json!(500)),
        ("title", serde_json::json!("Report Q1")),
    ]);
    let results = validate_record(&rules, &record);
    assert!(results.iter().all(|r| r.passed), "all basic rules should pass");
}

#[test]
fn test_boundary_gte() {
    let rule = ValidationRule::new("r1", "amount", Operator::Gte, serde_json::json!(100));
    let record = make_record(vec![("amount", serde_json::json!(100))]);
    let result = evaluate_rule(&rule, &record);
    assert!(
        result.passed,
        "amount=100 with gte(100) should pass: {}",
        result.message
    );
}

#[test]
fn test_boundary_lte() {
    let rule = ValidationRule::new("r1", "amount", Operator::Lte, serde_json::json!(1000));
    let record = make_record(vec![("amount", serde_json::json!(1000))]);
    let result = evaluate_rule(&rule, &record);
    assert!(
        result.passed,
        "amount=1000 with lte(1000) should pass: {}",
        result.message
    );
}

#[test]
fn test_chain_and_mode() {
    let chain = RuleChain::new("c1", ChainMode::And)
        .add_rule(ValidationRule::new("c1-r1", "amount", Operator::Gte, serde_json::json!(100)))
        .add_rule(ValidationRule::new("c1-r2", "amount", Operator::Lte, serde_json::json!(1000)));

    let record = make_record(vec![("amount", serde_json::json!(100))]);
    let result = evaluate_chain(&chain, &record);
    assert!(
        result.passed,
        "amount=100 should satisfy gte(100) AND lte(1000), details: {:?}",
        result.details.iter().map(|d| (&d.rule_id, d.passed)).collect::<Vec<_>>()
    );
}

#[test]
fn test_chain_or_mode() {
    let chain = RuleChain::new("c2", ChainMode::Or)
        .add_rule(ValidationRule::new("c2-r1", "amount", Operator::Gt, serde_json::json!(10000)))
        .add_rule(ValidationRule::new("c2-r2", "dept", Operator::Eq, serde_json::json!("finance")));

    let record = make_record(vec![
        ("amount", serde_json::json!(50)),
        ("dept", serde_json::json!("finance")),
    ]);
    let result = evaluate_chain(&chain, &record);
    assert!(
        result.passed,
        "OR chain should pass when second rule passes"
    );
}

#[test]
fn test_missing_field() {
    let rule = ValidationRule::new("r1", "nonexistent", Operator::Gt, serde_json::json!(0));
    let record = make_record(vec![("amount", serde_json::json!(100))]);
    let result = evaluate_rule(&rule, &record);
    assert!(!result.passed, "missing field should fail validation");
}
