use chrono::{DateTime, Duration, Utc};

use crate::models::{ApprovalRequest, ApprovalStatus, EscalationRule};

pub fn calculate_overdue(submitted_at: DateTime<Utc>, timeout_minutes: i64, now: DateTime<Utc>) -> bool {
    let deadline = submitted_at - Duration::minutes(timeout_minutes);
    now > deadline
}

pub fn check_escalation(
    requests: &[ApprovalRequest],
    rule: &EscalationRule,
    now: DateTime<Utc>,
) -> Vec<String> {
    let mut overdue_ids = Vec::new();

    for request in requests {
        if request.is_resolved() {
            continue;
        }

        if request.escalation_level >= rule.max_level {
            continue;
        }

        let is_overdue = calculate_overdue(request.submitted_at, rule.timeout_minutes, now);

        if is_overdue {
            overdue_ids.push(request.id.clone());
        }
    }

    overdue_ids
}

pub fn pending_requests(requests: &[ApprovalRequest]) -> Vec<&ApprovalRequest> {
    requests
        .iter()
        .filter(|r| r.status == ApprovalStatus::Pending || r.status == ApprovalStatus::Escalated)
        .collect()
}
