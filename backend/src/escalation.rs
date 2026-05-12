use chrono::Utc;
use crate::models::{ApprovalRequest, ApprovalStatus, ApprovalStore, EscalationEvent};

pub fn check_timeouts(store: &mut ApprovalStore) -> Vec<EscalationEvent> {
    let now = Utc::now();
    let mut new_events = Vec::new();

    for request in store.requests.iter_mut() {
        if request.status != ApprovalStatus::Pending {
            continue;
        }

        let elapsed = now.signed_duration_since(request.created_at);
        if elapsed.num_minutes() < store.config.timeout_minutes {
            continue;
        }

        if request.escalation_level >= store.config.max_escalation_level {
            continue;
        }

        let current_idx = store
            .approver_hierarchy
            .iter()
            .position(|a| a == &request.current_approver)
            .unwrap_or(0);

        let next_idx = current_idx + 1;
        if next_idx >= store.approver_hierarchy.len() {
            continue;
        }

        let from_approver = request.current_approver.clone();
        let to_approver = store.approver_hierarchy[next_idx].clone();

        request.escalation_level += 1;
        request.current_approver = to_approver.clone();
        request.status = ApprovalStatus::Escalated;

        let event = EscalationEvent {
            id: format!("evt-{}", Utc::now().timestamp_millis()),
            request_id: request.id.clone(),
            from_approver,
            to_approver,
            escalation_level: request.escalation_level,
            reason: format!("timeout after {} minutes", elapsed.num_minutes()),
            created_at: now,
            acknowledged: false,
        };

        new_events.push(event);
    }

    store.events.extend(new_events.clone());
    new_events
}

pub fn add_request(store: &mut ApprovalStore, applicant: &str, approver: &str) -> ApprovalRequest {
    let request = ApprovalRequest {
        id: format!("req-{}", Utc::now().timestamp_millis()),
        applicant: applicant.to_string(),
        current_approver: approver.to_string(),
        original_approver: approver.to_string(),
        escalation_level: 0,
        created_at: Utc::now(),
        status: ApprovalStatus::Pending,
    };
    store.requests.push(request.clone());
    request
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn make_store() -> ApprovalStore {
        let mut store = ApprovalStore::new(vec![
            "team-lead".to_string(),
            "manager".to_string(),
            "director".to_string(),
            "vp".to_string(),
        ]);
        store.config.timeout_minutes = 60;
        store.config.max_escalation_level = 3;
        store
    }

    #[test]
    fn test_no_escalation_when_not_timed_out() {
        let mut store = make_store();
        let req = add_request(&mut store, "alice", "team-lead");
        let events = check_timeouts(&mut store);
        assert!(events.is_empty());
        assert_eq!(store.requests[0].current_approver, "team-lead");
    }

    #[test]
    fn test_escalation_triggered() {
        let mut store = make_store();
        add_request(&mut store, "alice", "team-lead");
        store.requests[0].created_at = Utc::now() - Duration::minutes(120);

        let events = check_timeouts(&mut store);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].from_approver, "team-lead");
        assert_eq!(events[0].to_approver, "manager");
        assert_eq!(store.requests[0].escalation_level, 1);
    }

    #[test]
    fn test_escalation_chain() {
        let mut store = make_store();
        add_request(&mut store, "alice", "team-lead");
        store.requests[0].created_at = Utc::now() - Duration::minutes(120);
        store.requests[0].escalation_level = 1;
        store.requests[0].current_approver = "manager".to_string();

        let events = check_timeouts(&mut store);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].to_approver, "director");
        assert_eq!(store.requests[0].escalation_level, 2);
    }

    #[test]
    fn test_max_escalation_level() {
        let mut store = make_store();
        add_request(&mut store, "alice", "director");
        store.requests[0].created_at = Utc::now() - Duration::minutes(120);
        store.requests[0].escalation_level = 3;

        let events = check_timeouts(&mut store);
        assert!(events.is_empty());
    }

    #[test]
    fn test_no_escalation_for_non_pending() {
        let mut store = make_store();
        add_request(&mut store, "alice", "team-lead");
        store.requests[0].created_at = Utc::now() - Duration::minutes(120);
        store.requests[0].status = ApprovalStatus::Approved;

        let events = check_timeouts(&mut store);
        assert!(events.is_empty());
    }
}
