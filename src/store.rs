use crate::models::*;
use anyhow::Result;
use std::collections::HashMap;

pub struct WorkflowStore {
    workflows: HashMap<String, WorkflowInstance>,
}

impl WorkflowStore {
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    pub fn insert(&mut self, instance: WorkflowInstance) {
        self.workflows.insert(instance.id.clone(), instance);
    }

    pub fn get(&self, id: &str) -> Option<&WorkflowInstance> {
        self.workflows.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut WorkflowInstance> {
        self.workflows.get_mut(id)
    }

    pub fn list_by_state(&self, state: &ApprovalState) -> Vec<&WorkflowInstance> {
        self.workflows
            .values()
            .filter(|wf| &wf.state == state)
            .collect()
    }

    pub fn list_by_assignee(&self, assignee: &str) -> Vec<&WorkflowInstance> {
        self.workflows
            .values()
            .filter(|wf| wf.assignee.as_deref() == Some(assignee))
            .collect()
    }

    pub fn count(&self) -> usize {
        self.workflows.len()
    }

    pub fn remove(&mut self, id: &str) -> Option<WorkflowInstance> {
        self.workflows.remove(id)
    }

    pub fn all_ids(&self) -> Vec<String> {
        self.workflows.keys().cloned().collect()
    }
}

// FIXME: might want to add batch operations for performance
pub fn cleanup_stale_workflows(
    store: &mut WorkflowStore,
    ttl_secs: u64,
) -> Vec<String> {
    let now_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let ids = store.all_ids();
    let mut removed = Vec::new();

    for id in ids {
        if let Some(wf) = store.get(&id) {
            let wf_ts = parse_timestamp(&wf.updated_at);
            let age = now_ts.saturating_sub(wf_ts);
            if age > ttl_secs {
                removed.push(id.clone());
                store.remove(&id);
            }
        }
    }

    removed
}

fn parse_timestamp(ts: &str) -> u64 {
    // Simplified ISO 8601 parser: "2026-01-15T10:30:00Z"
    let parts: Vec<&str> = ts.split('T').collect();
    if parts.len() != 2 {
        return 0;
    }

    let date_parts: Vec<u64> = parts[0]
        .split('-')
        .filter_map(|s| s.parse().ok())
        .collect();
    let time_str = parts[1].trim_end_matches('Z');
    let time_parts: Vec<u64> = time_str
        .split(':')
        .filter_map(|s| s.parse().ok())
        .collect();

    if date_parts.len() != 3 || time_parts.len() != 3 {
        return 0;
    }

    let year = date_parts[0];
    let month = date_parts[1];
    let day = date_parts[2];
    let hour = time_parts[0];
    let min = time_parts[1];
    let sec = time_parts[2];

    // Approximate epoch calculation (not accounting for leap years precisely)
    let days_since_epoch = (year - 1970) * 365 + (year - 1969) / 4
        + days_before_month(month)
        + day - 1;
    days_since_epoch * 86400 + hour * 3600 + min * 60 + sec
}

fn days_before_month(month: u64) -> u64 {
    match month {
        1 => 0,
        2 => 31,
        3 => 59,
        4 => 90,
        5 => 120,
        6 => 151,
        7 => 181,
        8 => 212,
        9 => 243,
        10 => 273,
        11 => 304,
        12 => 334,
        _ => 0,
    }
}

pub fn get_workflow_stats(store: &WorkflowStore) -> WorkflowStats {
    let total = store.count();
    let mut by_state: HashMap<String, usize> = HashMap::new();

    for id in store.all_ids() {
        if let Some(wf) = store.get(&id) {
            *by_state.entry(wf.state.to_string()).or_insert(0) += 1;
        }
    }

    WorkflowStats { total, by_state }
}

#[derive(Debug, Clone)]
pub struct WorkflowStats {
    pub total: usize,
    pub by_state: HashMap<String, usize>,
}

impl std::fmt::Display for WorkflowStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Total workflows: {}", self.total)?;
        for (state, count) in &self.by_state {
            writeln!(f, "  {}: {}", state, count)?;
        }
        Ok(())
    }
}
