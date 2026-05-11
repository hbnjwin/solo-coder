use crate::models::{ApprovalStatus, AuditEntry};

/// Append-only audit log for tracking all approval workflow transitions.
/// Provides a complete history of actions taken on records.
pub struct AuditLog {
    entries: Vec<AuditEntry>,
}

// FIXME: audit entries are never pruned, potential memory issue
impl AuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Records a status transition in the audit log.
    pub fn log_transition(
        &mut self,
        record_id: &str,
        action: &str,
        actor: &str,
        from_status: &ApprovalStatus,
        to_status: &ApprovalStatus,
        version: u64,
    ) {
        let entry = AuditEntry::new(record_id, action, actor, from_status, to_status, version);
        self.entries.push(entry);
    }

    /// Returns all audit entries for a given record ID, in chronological order.
    pub fn entries_for_record(&self, record_id: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.record_id == record_id)
            .collect()
    }

    /// Returns the total number of audit entries across all records.
    pub fn total_entries(&self) -> usize {
        self.entries.len()
    }

    /// Returns the most recent audit entry, if any.
    pub fn last_entry(&self) -> Option<&AuditEntry> {
        self.entries.last()
    }

    /// Returns all entries performed by a specific actor.
    pub fn entries_by_actor(&self, actor: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.actor == actor)
            .collect()
    }
}
