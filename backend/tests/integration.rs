use chrono::{Duration, Utc};
use workflow_escalation_backend::checker::{calculate_overdue, check_escalation};
use workflow_escalation_backend::escalator::get_escalation_target;
use workflow_escalation_backend::models::{ApprovalRequest, EscalationRule};

#[test]
fn test_fresh_request_not_overdue() {
    let now = Utc::now();
    let submitted_at = now - Duration::minutes(1);
    let timeout_minutes = 60;

    let is_overdue = calculate_overdue(submitted_at, timeout_minutes, now);
    assert!(!is_overdue, "a request submitted 1 minute ago with 60-minute timeout should not be overdue");
}

#[test]
fn test_old_request_overdue() {
    let now = Utc::now();
    let submitted_at = now - Duration::minutes(120);
    let timeout_minutes = 60;

    let is_overdue = calculate_overdue(submitted_at, timeout_minutes, now);
    assert!(is_overdue, "a request submitted 2 hours ago with 60-minute timeout should be overdue");
}

#[test]
fn test_escalation_caps_at_level3() {
    let target = get_escalation_target(3);
    assert!(
        target.is_none(),
        "level 3 escalation should return None indicating no further escalation"
    );
}

#[test]
fn test_level1_escalation() {
    let target = get_escalation_target(1);
    assert_eq!(target, Some("director"));
}

#[test]
fn test_no_escalation_needed() {
    let now = Utc::now();
    let submitted_at = now - Duration::minutes(5);

    let request = ApprovalRequest::new("req-001", "Purchase order", "zhangsan", submitted_at);
    let rule = EscalationRule {
        timeout_minutes: 60,
        max_level: 3,
        notify_on_escalate: true,
    };

    let overdue = check_escalation(&[request], &rule, now);
    assert!(overdue.is_empty(), "fresh request should not trigger escalation");
}
