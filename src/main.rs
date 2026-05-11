use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord { pub id: String, pub status: String, pub approver: String, pub version: u64 }

pub struct ApprovalStore { records: HashMap<String, ApprovalRecord> }

impl ApprovalStore {
    pub fn new() -> Self { Self { records: HashMap::new() } }
    pub fn get(&self, id: &str) -> Option<ApprovalRecord> { self.records.get(id).cloned() }
    /// BUG: does not check version for optimistic locking
    pub fn update(&mut self, record: ApprovalRecord) -> Result<()> {
        self.records.insert(record.id.clone(), record);
        Ok(())
    }
    pub fn insert(&mut self, record: ApprovalRecord) { self.records.insert(record.id.clone(), record); }
}

/// BUG: read-then-write not atomic, no version check
pub fn approve_record(store: Arc<Mutex<ApprovalStore>>, record_id: &str, approver: &str) -> Result<()> {
    let mut s = store.lock().unwrap();
    let record = s.get(record_id).ok_or_else(|| anyhow::anyhow!("record not found"))?;
    let mut updated = record.clone();
    updated.status = "approved".into();
    updated.approver = approver.into();
    updated.version += 1;
    s.update(updated)
}

#[derive(Parser, Debug)]
#[command(name = "concurrent-approval", about = "Concurrent approval controller")]
struct Cli { #[arg(long, default_value = "rec-001")] record_id: String }

fn main() -> Result<()> {
    let store = Arc::new(Mutex::new(ApprovalStore::new()));
    store.lock().unwrap().insert(ApprovalRecord { id: "rec-001".into(), status: "pending".into(), approver: "".into(), version: 1 });
    approve_record(store.clone(), "rec-001", "zhangsan")?;
    let rec = store.lock().unwrap().get("rec-001").unwrap();
    println!("record: {}", serde_json::to_string(&rec)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    #[test]
    fn test_concurrent_overwrites() {
        let store = Arc::new(Mutex::new(ApprovalStore::new()));
        store.lock().unwrap().insert(ApprovalRecord { id: "rec-1".into(), status: "pending".into(), approver: "".into(), version: 1 });
        let mut handles = vec![];
        for i in 0..5 {
            let s = store.clone();
            handles.push(thread::spawn(move || approve_record(s, "rec-1", &format!("user-{}", i))));
        }
        let successes: Vec<_> = handles.into_iter().filter(|h| h.join().unwrap().is_ok()).count();
        assert!(successes <= 1, "only one should succeed, got {}", successes);
    }
}
