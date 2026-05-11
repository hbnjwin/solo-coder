use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeType {
    Modified,
    Added,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldChange {
    pub path: String,
    pub change_type: ChangeType,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
}

impl FieldChange {
    pub fn modified(path: String, old: serde_json::Value, new: serde_json::Value) -> Self {
        Self {
            path,
            change_type: ChangeType::Modified,
            old_value: Some(old),
            new_value: Some(new),
        }
    }

    pub fn added(path: String, value: serde_json::Value) -> Self {
        Self {
            path,
            change_type: ChangeType::Added,
            old_value: None,
            new_value: Some(value),
        }
    }

    pub fn removed(path: String, value: serde_json::Value) -> Self {
        Self {
            path,
            change_type: ChangeType::Removed,
            old_value: Some(value),
            new_value: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub entity_id: String,
    pub revision: u64,
    pub changes: Vec<FieldChange>,
}

#[derive(Debug, Clone)]
pub struct DiffOptions {
    pub track_additions: bool,
    pub track_removals: bool,
    pub track_modifications: bool,
    pub max_depth: usize,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            track_additions: true,
            track_removals: true,
            track_modifications: true,
            max_depth: 16,
        }
    }
}
