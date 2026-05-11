use approval_metrics::calculator;
use approval_metrics::analyzer;
use approval_metrics::models::ApprovalRecord;

fn make_record(id: &str, dept: &str, approver: &str, status: &str, submitted: &str, completed: Option<&str>) -> ApprovalRecord {
    ApprovalRecord {
        id: id.to_string(),
        department: dept.to_string(),
        approver: approver.to_string(),
        approval_type: "standard".to_string(),
        status: status.to_string(),
        submitted_at: submitted.to_string(),
        completed_at: completed.map(|s| s.to_string()),
    }
}

#[test]
fn test_approval_rate_basic() {
    let records = vec![
        make_record("1", "finance", "alice", "approved", "2026-01-01 09:00:00", Some("2026-01-01 17:00:00")),
        make_record("2", "finance", "alice", "approved", "2026-01-02 09:00:00", Some("2026-01-02 14:00:00")),
        make_record("3", "hr", "bob", "approved", "2026-01-03 09:00:00", Some("2026-01-03 12:00:00")),
        make_record("4", "hr", "bob", "rejected", "2026-01-04 09:00:00", Some("2026-01-04 16:00:00")),
        make_record("5", "finance", "alice", "rejected", "2026-01-05 09:00:00", Some("2026-01-05 11:00:00")),
        make_record("6", "ops", "carol", "pending", "2026-01-06 09:00:00", None),
        make_record("7", "ops", "carol", "pending", "2026-01-07 09:00:00", None),
        make_record("8", "hr", "bob", "pending", "2026-01-08 09:00:00", None),
        make_record("9", "finance", "alice", "pending", "2026-01-09 09:00:00", None),
        make_record("10", "ops", "carol", "pending", "2026-01-10 09:00:00", None),
    ];

    let result = calculator::calculate_approval_rate(&records);
    // 3 approved out of 5 resolved (3 approved + 2 rejected) = 60%
    assert!(
        (result.value - 60.0).abs() < 0.1,
        "Expected 60% approval rate among resolved records, got {:.1}%",
        result.value
    );
}

#[test]
fn test_avg_duration() {
    let records = vec![
        make_record("1", "finance", "alice", "approved", "2026-01-01 09:00:00", Some("2026-01-01 21:00:00")),  // 12 hours
        make_record("2", "hr", "bob", "approved", "2026-01-02 08:00:00", Some("2026-01-02 14:00:00")),         // 6 hours
        make_record("3", "ops", "carol", "rejected", "2026-01-03 10:00:00", Some("2026-01-03 22:00:00")),      // 12 hours
        make_record("4", "finance", "alice", "pending", "2026-01-04 09:00:00", None),                           // no completion
        make_record("5", "hr", "bob", "pending", "2026-01-05 09:00:00", None),                                  // no completion
    ];

    let result = calculator::calculate_avg_duration(&records);
    // Only 3 records have completion times: 12 + 6 + 12 = 30 hours / 3 = 10.0 hours
    assert!(
        (result.value - 10.0).abs() < 0.1,
        "Expected 10.0 hours average duration for completed records, got {:.2}",
        result.value
    );
}

#[test]
fn test_bottleneck_detection() {
    let records = vec![
        // alice: avg 4 hours
        make_record("1", "finance", "alice", "approved", "2026-01-01 09:00:00", Some("2026-01-01 13:00:00")),
        make_record("2", "finance", "alice", "approved", "2026-01-02 09:00:00", Some("2026-01-02 13:00:00")),
        // bob: avg 24 hours (slowest)
        make_record("3", "hr", "bob", "approved", "2026-01-03 09:00:00", Some("2026-01-04 09:00:00")),
        make_record("4", "hr", "bob", "rejected", "2026-01-05 09:00:00", Some("2026-01-06 09:00:00")),
        // carol: avg 8 hours
        make_record("5", "ops", "carol", "approved", "2026-01-07 09:00:00", Some("2026-01-07 17:00:00")),
        make_record("6", "ops", "carol", "approved", "2026-01-08 09:00:00", Some("2026-01-08 17:00:00")),
    ];

    let bottlenecks = analyzer::identify_bottlenecks(&records);
    assert!(!bottlenecks.is_empty(), "Should identify at least one bottleneck");
    assert_eq!(
        bottlenecks[0], "bob",
        "Expected slowest approver (bob) first, got '{}'",
        bottlenecks[0]
    );
}

#[test]
fn test_group_by_department() {
    let records = vec![
        make_record("1", "finance", "alice", "approved", "2026-01-01 09:00:00", Some("2026-01-01 17:00:00")),
        make_record("2", "finance", "alice", "rejected", "2026-01-02 09:00:00", Some("2026-01-02 15:00:00")),
        make_record("3", "hr", "bob", "approved", "2026-01-03 09:00:00", Some("2026-01-03 12:00:00")),
        make_record("4", "ops", "carol", "pending", "2026-01-04 09:00:00", None),
    ];

    let groups = analyzer::group_by_field(&records, "department");
    assert_eq!(groups.len(), 3, "Expected 3 departments");
    assert_eq!(groups["finance"].len(), 2);
    assert_eq!(groups["hr"].len(), 1);
    assert_eq!(groups["ops"].len(), 1);
}

#[test]
fn test_empty_dataset() {
    let records: Vec<ApprovalRecord> = vec![];

    let rate = calculator::calculate_approval_rate(&records);
    let duration = calculator::calculate_avg_duration(&records);
    let bottlenecks = analyzer::identify_bottlenecks(&records);

    assert_eq!(rate.value, 0.0);
    assert_eq!(rate.sample_size, 0);
    assert_eq!(duration.value, 0.0);
    assert_eq!(duration.sample_size, 0);
    assert!(bottlenecks.is_empty());
}
