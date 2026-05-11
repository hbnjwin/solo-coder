use api_contract_backend::handlers::process_approval;

fn main() {
    let sample_request = r#"{
        "contract_name": "供应商框架协议",
        "sheet_amount": 850000.0,
        "approver": "李明",
        "department": "采购部",
        "submission_date": "2026-05-11",
        "urgent": "yes",
        "notes": "需要在月底前完成审批"
    }"#;

    let response = process_approval(sample_request);
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}
