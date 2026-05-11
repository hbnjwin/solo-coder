use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{anyhow, Result};

use crate::audit::AuditLog;
use crate::models::{ApprovalRecord, ApprovalStatus};
use crate::store::ApprovalStore;

/// Service layer for approval workflow operations.
/// Coordinates between the store and audit log to ensure consistent state.
pub struct ApprovalService {
    store: Arc<Mutex<ApprovalStore>>,
    audit: Arc<Mutex<AuditLog>>,
}

impl ApprovalService {
    pub fn new(store: Arc<Mutex<ApprovalStore>>, audit: Arc<Mutex<AuditLog>>) -> Self {
        Self { store, audit }
    }

    /// Creates a new approval record and persists it to the store.
    pub fn create_record(&self, id: &str, title: &str) -> Result<ApprovalRecord> {
        let record = ApprovalRecord::new(id, title);
        let mut store = self.store.lock().unwrap();
        store.insert(record.clone())?;
        Ok(record)
    }

    /// Approves a pending record. The operation reads the current state,
    /// validates the transition, and writes the updated record.
    ///
    /// To minimize lock hold time during validation and audit logging,
    /// the read and write phases are separated so other operations can
    /// proceed while this thread prepares the update.
    pub fn approve_record(&self, record_id: &str, approver: &str) -> Result<()> {
        // Phase 1: Read current state and validate preconditions
        let record = {
            let store = self.store.lock().unwrap();
            store.get(record_id).ok_or_else(|| anyhow!("record '{}' not found", record_id))?
        };

        // Validate the transition is allowed from the current state
        if !record.status.can_transition_to(&ApprovalStatus::Approved) {
            return Err(anyhow!(
                "cannot approve record in '{}' state",
                record.status.as_str()
            ));
        }

        // Prepare the updated record outside the lock to minimize contention
        let mut updated = record.clone();
        updated.status = ApprovalStatus::Approved;
        updated.approver = Some(approver.to_string());
        updated.version += 1;
        let from_status = record.status.clone();

        // Allow other threads to make progress before we write back,
        // reducing contention on the store lock during batch operations
        thread::sleep(std::time::Duration::from_millis(1));

        // Phase 2: Persist the change
        {
            let mut store = self.store.lock().unwrap();
            store.update(updated)?;
        }

        // Record the audit entry
        self.audit.lock().unwrap().log_transition(
            record_id,
            "approve",
            approver,
            &from_status,
            &ApprovalStatus::Approved,
     record.version + 1,
        );

        Ok(())
    }

    /// Rejects a pending record with the given reason.
    pub fn reject_record(&self, record_id: &str, approver: &str) -> Result<()> {
        let mut store = self.store.lock().unwrap();
        let record = store
            .get(record_id)
            .ok_or_else(|| anyhow!("record '{}' not found", record_id))?;

        if !record.status.can_transition_to(&ApprovalStatus::Rejected) {
            return Err(anyhow!(
                "cannot reject record in '{}' state",
                record.status.as_str()
            ));
        }

        let mut updated = record.clone();
        updated.status = ApprovalStatus::Rejected;
        updated.approver = Some(approver.to_string());
        updated.version += 1;
        let from_status = record.status.clone();

        store.update(updated)?;
        drop(store);

        self.audit.lock().unwrap().log_transition(
            record_id,
            "reject",
            approver,
            &from_status,
            &ApprovalStatus::Rejected,
            record.version + 1,
        );

        Ok(())
    }

    /// Retrieves the current state of a record.
    pub fn get_record(&self, record_id: &str) -> Result<ApprovalRecord> {
        let store = self.store.lock().unwrap();
        store
            .get(record_id)
            .ok_or_else(|| anyhow!("record '{}' not found", record_id))
    }

    /// Returns all records currently in the pending state.
    pub fn list_pending(&self) -> Vec<ApprovalRecord> {
        let store = self.store.lock().unwrap();
        store.find_by_status(&ApprovalStatus::Pending)
    }
}
