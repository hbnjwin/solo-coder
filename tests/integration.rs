use std::sync::{Arc, Mutex};
use std::thread;

use concurrent_approval::audit::AuditLog;
use concurrent_approval::models::ApprovalRecord;
use concurrent_approval::service::ApprovalService;
use concurrent_approval::store::ApprovalStore;

fn create_service() -> (ApprovalService, Arc<Mutex<ApprovalStore>>, Arc<Mutex<AuditLog>>) {
    let store = Arc::new(Mutex::new(ApprovalStore::new()));
    let audit = Arc::new(Mutex::new(AuditLog::new()));
    let service = ApprovalService::new(store.clone(), audit.clone());
    (service, store, audit)
}

#[test]
fn test_concurrent_ops() {
    let store = Arc::new(Mutex::new(ApprovalStore::new()));
    let audit = Arc::new(Mutex::new(AuditLog::new()));

    // Insert a pending record
    let record = ApprovalRecord::new("rec-concurrent", "Concurrent Test");
    store.lock().unwrap().insert(record).unwrap();

    let mut handles = vec![];
    for i in 0..5 {
        let s = store.clone();
        let a = audit.clone();
        handles.push(thread::spawn(move || {
            let svc = ApprovalService::new(s, a);
            svc.approve_record("rec-concurrent", &format!("approver-{}", i))
        }));
    }

    let successes: usize = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|r| r.is_ok())
        .count();

    // Only one concurrent approval should succeed for the same record
    assert!(
        successes <= 1,
        "expected at most 1 successful approval, got {}",
        successes
    );
}

#[test]
fn test_version_conflict() {
    let mut store = ApprovalStore::new();
    let record = ApprovalRecord::new("r-version", "Version Test");
    store.insert(record).unwrap();

    // Simulate a stale read: someone else already updated the record to version 2
    {
        let mut current = store.get("r-version").unwrap();
        current.version = 2;
        current.status = concurrent_approval::models::ApprovalStatus::Pending;
        // Directly overwrite to simulate another writer having bumped the version
        store.insert(current).unwrap();
    }

    // Now attempt an update using a stale version (version 1)
    let mut stale = store.get("r-version").unwrap();
    stale.version = 1; // pretend we read this before the other writer
    stale.status = concurrent_approval::models::ApprovalStatus::Approved;
    stale.approver = Some("late-approver".to_string());

    let result = store.update(stale);
    assert!(
        result.is_err(),
        "update with stale version should fail due to optimistic locking"
    );
}

#[test]
fn test_basic_approval_flow() {
    let (service, _store, _audit) = create_service();

    service.create_record("rec-basic", "Basic Flow Test").unwrap();
    service.approve_record("rec-basic", "manager-1").unwrap();

    let record = service.get_record("rec-basic").unwrap();
    assert_eq!(
        record.status,
        concurrent_approval::models::ApprovalStatus::Approved
    );
    assert_eq!(record.approver, Some("manager-1".to_string()));
    assert_eq!(record.version, 2);
}

#[test]
fn test_rejection_flow() {
    let (service, _store, _audit) = create_service();

    service.create_record("rec-reject", "Rejection Flow Test").unwrap();
    service.reject_record("rec-reject", "manager-2").unwrap();

    let record = service.get_record("rec-reject").unwrap();
    assert_eq!(
        record.status,
        concurrent_approval::models::ApprovalStatus::Rejected
    );
    assert_eq!(record.approver, Some("manager-2".to_string()));
    assert_eq!(record.version, 2);
}

#[test]
fn test_audit_trail() {
    let (service, _store, audit) = create_service();

    service.create_record("rec-audit", "Audit Trail Test").unwrap();
    service.approve_record("rec-audit", "auditor-1").unwrap();

    let log = audit.lock().unwrap();
    let entries = log.entries_for_record("rec-audit");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].action, "approve");
    assert_eq!(entries[0].actor, "auditor-1");
    assert_eq!(entries[0].from_status, "pending");
    assert_eq!(entries[0].to_status, "approved");
}
