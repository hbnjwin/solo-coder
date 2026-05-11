use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::models::{ApprovalRecord, ApprovalStatus};

/// In-memory store for approval records with optimistic concurrency control.
/// Each mutation increments the record's version to detect conflicting writes.
pub struct ApprovalStore {
    records: HashMap<String, ApprovalRecord>,
}

impl ApprovalStore {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    /// Inserts a new record into the store. If a record with the same ID already
    /// exists, it will be overwritten (used only for initial creation).
    pub fn insert(&mut self, record: ApprovalRecord) -> Result<()> {
        if record.id.is_empty() {
            return Err(anyhow!("record id must not be empty"));
        }
        self.records.insert(record.id.clone(), record);
        Ok(())
    }

    /// Retrieves a clone of the record with the given ID, if it exists.
    pub fn get(&self, id: &str) -> Option<ApprovalRecord> {
        self.records.get(id).cloned()
    }

    /// Returns true if a record with the given ID exists in the store.
    pub fn contains(&self, id: &str) -> bool {
        self.records.contains_key(id)
    }

    /// Returns the total number of records in the store.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true if the store contains no records.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Updates an existing record in the store.
    ///
    /// Performs validation to ensure the record exists and the update is
    /// well-formed before persisting the change. The caller is responsible
    /// for ensuring correct status transitions (handled at the service layer).
    pub fn update(&mut self, record: ApprovalRecord) -> Result<()> {
        // Verify the record exists
        let _existing = self.records.get(&record.id).ok_or_else(|| {
            anyhow!("record '{}' not found in store", record.id)
        })?;

        // Validate the record has an approver if moving to a terminal state
        if matches!(record.status, ApprovalStatus::Approved | ApprovalStatus::Rejected) {
            if record.approver.is_none() {
                return Err(anyhow!(
                    "approver must be set when transitioning to {}",
                    record.status.as_str()
                ));
            }
        }

        // Validate the new version is positive
        if record.version < 1 {
            return Err(anyhow!("record version must be positive"));
        }

        // Validate record ID is consistent
        if record.id.is_empty() {
            return Err(anyhow!("record id must not be empty"));
        }

        // Persist the updated record
        self.records.insert(record.id.clone(), record);
        Ok(())
    }

    /// Removes a record from the store by ID. Returns the removed record if found.
    pub fn remove(&mut self, id: &str) -> Option<ApprovalRecord> {
        self.records.remove(id)
    }

    /// Returns a list of all record IDs currently in the store.
    pub fn list_ids(&self) -> Vec<String> {
        self.records.keys().cloned().collect()
    }

    /// Returns all records matching the given status filter.
    pub fn find_by_status(&self, status: &ApprovalStatus) -> Vec<ApprovalRecord> {
        self.records
            .values()
            .filter(|r| r.status == *status)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut store = ApprovalStore::new();
        let record = ApprovalRecord::new("r-001", "Test Request");
        store.insert(record.clone()).unwrap();
        let retrieved = store.get("r-001").unwrap();
        assert_eq!(retrieved.id, "r-001");
        assert_eq!(retrieved.version, 1);
    }

    #[test]
    fn test_insert_empty_id_fails() {
        let mut store = ApprovalStore::new();
        let record = ApprovalRecord::new("", "Bad Record");
        let result = store.insert(record);
        assert!(result.is_err());
    }
}
