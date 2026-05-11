use query_def_safety::builder::build_parameterized_query;
use query_def_safety::models::{QueryParam, QueryTemplate};
use query_def_safety::template::render_template;
use query_def_safety::validator::validate_param;

#[test]
fn test_basic_query_render() {
    let tmpl = QueryTemplate::new(
        "select_dept",
        "SELECT * FROM contracts WHERE dept = '{{dept}}'",
        vec![QueryParam::text("dept", "finance")],
    );
    let result = render_template(&tmpl).unwrap();
    assert!(result.sql.contains("finance"));
    assert!(!result.is_parameterized);
}

#[test]
fn test_injection_attempt() {
    let payload = "'; DROP TABLE users; --";
    let tmpl = QueryTemplate::new(
        "select_user",
        "SELECT * FROM users WHERE name = '{{name}}'",
        vec![QueryParam::text("name", payload)],
    );
    let result = render_template(&tmpl).unwrap();
    assert!(
        !result.sql.contains("DROP TABLE"),
        "rendered SQL must not contain raw injection payload: {}",
        result.sql
    );
}

#[test]
fn test_unicode_bypass() {
    let fullwidth_payload = "\u{FF27}\u{FF32}\u{FF21}\u{FF2E}\u{FF34}'; DROP TABLE x; --";
    let param = QueryParam::text("value", fullwidth_payload);
    let result = validate_param(&param);
    assert!(
        result.is_err(),
        "validator should reject values containing fullwidth characters that may bypass filters"
    );
}

#[test]
fn test_parameterized_output() {
    let tmpl = QueryTemplate::new(
        "param_query",
        "SELECT * FROM orders WHERE status = {{status}} AND region = {{region}}",
        vec![
            QueryParam::text("status", "active"),
            QueryParam::text("region", "us-east"),
        ],
    );
    let result = build_parameterized_query(&tmpl);
    assert!(result.is_ok(), "build_parameterized_query should succeed");
    let qr = result.unwrap();
    assert!(qr.is_parameterized);
    assert!(qr.sql.contains("$1"));
    assert!(qr.sql.contains("$2"));
}

#[test]
fn test_valid_params_accepted() {
    let params = vec![
        QueryParam::text("name", "Alice Johnson"),
        QueryParam::text("dept", "engineering"),
        QueryParam::text("id", "12345"),
    ];
    for p in &params {
        assert!(
            validate_param(p).is_ok(),
            "valid param '{}' should be accepted",
            p.name
        );
    }
}

#[test]
fn mplate_parsing() {
    let tmpl = QueryTemplate::new(
        "multi_param",
        "INSERT INTO logs (user, action, ts) VALUES ('{{user}}', '{{action}}', {{timestamp}})",
        vec![
            QueryParam::text("user", "admin"),
            QueryParam::text("action", "login"),
            QueryParam::text("timestamp", "NOW()"),
        ],
    );
    let slots = query_def_safety::template::parse_template(&tmpl).unwrap();
    assert_eq!(slots.len(), 3);
    assert!(slots.contains(&"user".to_string()));
    assert!(slots.contains(&"action".to_string()));
    assert!(slots.contains(&"timestamp".to_string()));
}
