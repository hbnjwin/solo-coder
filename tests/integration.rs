use approval_csv_export::models::{ExportRecord, FilterCondition, FilterOp, SortConfig, SortDirection};
use approval_csv_export::filter::apply_filters;
use approval_csv_export::sorter::sort_records;
use approval_csv_export::exporter::{export_csv, export_json};

fn sample_records() -> Vec<ExportRecord> {
    vec![
        ExportRecord { id: "1".into(), contract_name: "Acme Corp".into(), amount: 500000.0, status: "approved".into(), approver: "zhang".into(), date: "2026-01-15".into() },
        ExportRecord { id: "2".into(), contract_name: "Beta Services".into(), amount: 2000000.0, status: "pending".into(), approver: "li".into(), date: "2026-03-20".into() },
        ExportRecord { id: "3".into(), contract_name: "Gamma Holdings".into(), amount: 750000.0, status: "approved".into(), approver: "wang".into(), date: "2026-02-10".into() },
        ExportRecord { id: "4".into(), contract_name: "Delta Group".into(), amount: 120000.0, status: "rejected".into(), approver: "zhang".into(), date: "2026-04-01".into() },
        ExportRecord { id: "5".into(), contract_name: "Epsilon Ltd".into(), amount: 3500000.0, status: "approved".into(), approver: "li".into(), date: "2026-01-28".into() },
    ]
}

#[test]
fn test_filter_by_status() {
    let records = sample_records();
    let conditions = vec![
        FilterCondition {
            field: "status".into(),
            op: FilterOp::Eq,
            value: "approved".into(),
        },
    ];
    let result = apply_filters(&records, &conditions);
    assert_eq!(result.len(), 3, "expected 3 approved records out of 5");
    assert!(result.iter().all(|r| r.status == "approved"));
}

#[test]
fn test_csv_escape_comma() {
    let records = vec![
        ExportRecord {
            id: "1".into(),
            contract_name: "A, B Corp".into(),
            amount: 100.0,
            status: "approved".into(),
            approver: "zhang".into(),
            date: "2026-01-01".into(),
        },
    ];
    let csv = export_csv(&records).unwrap();
    let lines: Vec<&str> = csv.lines().collect();
    let data_line = lines[1];
    assert!(
        data_line.contains("\"A, B Corp\""),
        "field containing comma must be quoted in CSV output, got: {}",
        data_line
    );
}

#[test]
fn test_csv_escape_quote() {
    let records = vec![
        ExportRecord {
            id: "1".into(),
            contract_name: "A \"Premium\" Deal".into(),
            amount: 200.0,
            status: "pending".into(),
            approver: "li".into(),
            date: "2026-02-15".into(),
        },
    ];
    let csv = export_csv(&records).unwrap();
    let lines: Vec<&str> = csv.lines().collect();
    let data_line = lines[1];
    assert!(
        data_line.contains("\"A \"\"Premium\"\" Deal\""),
        "field containing quotes must have quotes doubled and be wrapped, got: {}",
        data_line
    );
}

#[test]
fn test_sort_by_date() {
    let mut records = sample_records();
    let config = SortConfig {
        field: "date".into(),
        direction: SortDirection::Descending,
    };
    sort_records(&mut records, &config);

    let dates: Vec<&str> = records.iter().map(|r| r.date.as_str()).collect();
    assert_eq!(
        dates,
        vec!["2026-04-01", "2026-03-20", "2026-02-10", "2026-01-28", "2026-01-15"],
        "records should be sorted by date d"
    );
}

#[test]
fn test_export_json() {
    let records = vec![
        ExportRecord {
            id: "1".into(),
            contract_name: "Test Corp".into(),
            amount: 1000.0,
            status: "approved".into(),
            approver: "admin".into(),
            date: "2026-06-01".into(),
        },
    ];
    let json = export_json(&records).unwrap();
    let parsed: Vec<ExportRecord> = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].id, "1");
    assert_eq!(parsed[0].contract_name, "Test Corp");
    assert_eq!(parsed[0].amount, 1000.0);
}

#[test]
fn test_filter_by_amount_range() {
    let records = sample_records();
    let conditions = vec![
        FilterCondition {
            field: "amount".into(),
            op: FilterOp::Gt,
            value: "600000".into(),
        },
        FilterCondition {
            field: "amount".into(),
            op: FilterOp::Lt,
            value: "3000000".into(),
        },
    ];
    let result = apply_filters(&records, &conditions);
    assert_eq!(result.len(), 2, "expected 2 records with amount between 600k and 3M");
    assert!(result.iter().all(|r| r.amount > 600000.0 && r.amount < 3000000.0));
}
