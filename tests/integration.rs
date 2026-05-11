use workflow_state_machine::engine::{auto_advance, find_path, StateMachine};
use workflow_state_machine::models::*;
use workflow_state_machine::store::{cleanup_stale_workflows, WorkflowStore};

fn make_instance(id: &str, state: ApprovalState, updated_at: &str) -> WorkflowInstance {
    WorkflowInstance {
        id: id.into(),
        state,
        title: format!("Test workflow {}", id),
        created_at: "2026-01-01T00:00:00Z".into(),
        updated_at: updated_at.into(),
        version: 1,
        assignee: Some("tester".into()),
        priority: Priority::Normal,
    }
}

#[test]
fn test_workflow_scenario_a() {
    let machine = StateMachine::new();
    let mut instance = make_instance("wf-1", ApprovalState::Submitted, "2026-05-11T10:00:00Z");

    auto_advance(&machine, &mut instance, ApprovalState::UnderReview).unwrap();
    assert_eq!(instance.state, ApprovalState::UnderReview);

    auto_advance(&machine, &mut instance, ApprovalState::Rejected).unwrap();
    assert_eq!(instance.state, ApprovalState::Rejected);

    let result = auto_advance(&machine, &mut instance, ApprovalState::Approved);
    assert!(
        result.is_err(),
        "expected cycle detection error, got {:?}",
        result
    );
}

#[test]
fn test_workflow_scenario_b() {
    let machine = StateMachine::new();
    let mut instance = make_instance("wf-2", ApprovalState::Draft, "2026-05-11T10:00:00Z");

    let events = auto_advance(&machine, &mut instance, ApprovalState::Completed).unwrap();
    assert_eq!(instance.state, ApprovalState::Completed);
    assert!(
        events.len() >= 4,
        "expected at least 4 transitions, got {}",
        events.len()
    );

    // Resubmit from Rejected is a valid single-step transition
    let mut resubmit = make_instance("wf-3", ApprovalState::Rejected, "2026-05-11T10:00:00Z");
    let r = auto_advance(&machine, &mut resubmit, ApprovalState::Submitted);
    assert!(
        r.is_ok(),
        "single-step resubmit from Rejected should succeed, got {:?}",
        r
    );
    assert_eq!(resubmit.state, ApprovalState::Submitted);
}

#[test]
fn test_path_resolution() {
    let machine = StateMachine::new();

    let path = find_path(&machine, &ApprovalState::Draft, &ApprovalState::Completed);
    assert!(path.is_some(), "should find path from Draft to Completed");

    let path = find_path(&machine, &ApprovalState::Completed, &ApprovalState::Draft);
    assert!(path.is_none(), "should not find path from Completed to Draft");
}

#[test]
fn test_store_maintenance() {
    let mut store = WorkflowStore::new();

    store.insert(make_instance(
        "old-wf",
        ApprovalState::Draft,
        "2020-01-01T00:00:00Z",
    ));
    store.insert(make_instance(
        "new-wf",
        ApprovalState::Submitted,
        "2026-05-11T10:00:00Z",
    ));

    assert_eq!(store.count(), 2);

    let removed = cleanup_stale_workflows(&mut store, 3600);

    assert!(
        removed.contains(&"old-wf".to_string()),
        "stale workflow should be removed, removed: {:?}",
        removed
    );
    assert!(
        !removed.contains(&"new-wf".to_string()),
        "fresh workflow should NOT be removed, removed: {:?}",
        removed
    );
    assert_eq!(store.count(), 1);
}

#[test]
fn test_batch_processing() {
    let machine = StateMachine::new();
    let mut store = WorkflowStore::new();

    store.insert(make_instance(
        "b-1",
        ApprovalState::Draft,
        "2026-05-11T10:00:00Z",
    ));
    store.insert(make_instance(
        "b-2",
        ApprovalState::Draft,
        "2026-05-11T10:00:00Z",
    ));
    store.insert(make_instance(
        "b-3",
        ApprovalState::Completed,
        "2026-05-11T10:00:00Z",
    ));

    let instance1 = store.get_mut("b-1").unwrap();
    let r1 = auto_advance(&machine, instance1, ApprovalState::Submitted);
    assert!(r1.is_ok());

    let instance3 = store.get_mut("b-3").unwrap();
    let r3 = auto_advance(&machine, instance3, ApprovalState::Submitted);
    assert!(r3.is_err());
}
