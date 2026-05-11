use anyhow::Result;
use chrono::Local;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditLevel { Error, Warn, Info, Debug }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub trace_id: String, pub timestamp: String, pub operator: String,
    pub action: String, pub target_id: String, pub level: AuditLevel, pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldChange { pub field: String, pub old_value: String, pub new_value: String }

pub struct AuditStore { entries: Vec<AuditEntry> }

impl AuditStore {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn append(&mut self, entry: AuditEntry) { self.entries.push(entry); }
    pub fn query_by_trace_id(&self, trace_id: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.trace_id == trace_id).collect()
    }
}

/// TODO: not implemented - should support nested objects and arrays
pub fn compute_field_diff(_old: &serde_json::Value, _new: &serde_json::Value) -> Vec<FieldChange> {
    vec![]
}

#[derive(Parser, Debug)]
#[command(name = "audit-trail", about = "Approval audit trail system")]
struct Cli { #[arg(long, default_value = "trace-001")] trace_id: String }

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = AuditStore::new();
    store.append(AuditEntry {
        trace_id: cli.trace_id.clone(), timestamp: Local::now().to_rfc3339(),
        operator: "zhangsan".into(), action: "submit".into(), target_id: "contract-001".into(),
        level: AuditLevel::Info, detail: "submitted for approval".into(),
    });
    for e in store.query_by_trace_id(&cli.trace_id) {
        println!("{}", serde_json::to_string(e)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_field_diff() {
        let old = serde_json::json!({"amount": 10000});
        let new = serde_json::json!({"amount": 15000});
        let changes = compute_field_diff(&old, &new);
        assert!(!changes.is_empty(), "should detect amount change");
        assert!(changes.iter().any(|c| c.field == "amount" && c.old_value == "10000" && c.new_value == "15000"));
    }

    #[test]
    fn test_nested_object_diff() {
        let old = serde_json::json!({"approver": {"name": "zhangsan", "level": 2}});
        let new = serde_json::json!({"approver": {"name": "lisi", "level": 2}});
        let changes = compute_field_diff(&old, &new);
        assert!(!changes.is_empty(), "should detect nested change");
        assert!(changes.iter().any(|c| c.field == "approver.name" && c.old_value == "zhangsan" && c.new_value == "lisi"),
            "should use dot notation for nested fields");
    }

    #[test]
    fn test_array_diff() {
        let old = serde_json::json!({"tags": ["urgent", "finance"]});
        let new = serde_json::json!({"tags": ["urgent", "finance", "contract"]});
        let changes = compute_field_diff(&old, &new);
        assert!(!changes.is_empty(), "should detect array change");
    }
}
