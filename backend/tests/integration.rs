use api_contract_backend::handlers::{process_approval, parse_submission_date};

#[test]
fn test_parse_valid_request() {
    let body = r#"{
        "contract_name": "测试合同",
        "sheet_amount": 500000.0,
        "approver": "张三",
        "department": "财务部",
        "submission_date": "2026-05-11",
        "urgent": "yes",
        "notes": null
    }"#;

    let response = process_approval(body);
    assert_eq!(response.status, "accepted");
    assert_eq!(response.priority, "high");
    assert!(response.validation_errors.is_empty());
    assert!(response.parsed_date.is_some());
    let date = response.parsed_date.unwrap();
    assert_eq!(date.year, 2026);
    assert_eq!(date.month, 5);
    assert_eq!(date.day, 11);
}

#[test]
fn test_camelcase_fields_rejected() {
    let body = r#"{
        "contractName": "供应商协议",
        "sheetAmount": 200000.0,
        "approver": "王五",
        "department": "法务部",
        "submissionDate": "2026-03-15",
        "isUrgent": true,
        "notes": "紧急处理"
    }"#;

    let response = process_approval(body);
    assert_eq!(response.status, "accepted", "camelCase fields should be deserialized correctly");
    assert!(
        response.parsed_date.is_some(),
        "submission date should be parsed when sent as camelCase field"
    );
}

#[test]
fn test_iso_datetime_parsing() {
    let iso_date = "2026-05-11T10:30:00Z";
    let result = parse_submission_date(iso_date);
    assert!(
        result.is_ok(),
        "ISO 8601 datetime strings should be parsed successfully, got error: {:?}",
        result.err()
    );
    let parsed = result.unwrap();
    assert_eq!(parsed.year, 2026);
    assert_eq!(parsed.month, 5);
    assert_eq!(parsed.day, 11);
}

#[test]
fn test_boolean_urgent_field() {
    let body = r#"{
        "contract_name": "紧急采购合同",
        "sheet_amount": 1000000.0,
        "approver": "赵六",
        "department": "采购部",
        "submission_date": "2026-06-01",
        "urgent": true,
        "notes": null
    }"#;

    let response = process_approval(body);
    assert_eq!(
        response.status, "accepted",
        "boolean urgent value should be accepted"
    );
    assert_eq!(
        response.priority, "high",
        "boolean true for urgent should result in high priority"
    );
}
