use crate::models::*;
use std::collections::VecDeque;

pub struct TransitionHistory {
    entries: VecDeque<TransitionEvent>,
    max_size: usize,
}

// FIXME: potential memory leak if history grows unbounded — consider LRU eviction
impl TransitionHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn record(&mut self, event: TransitionEvent) {
        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }
        self.entries.push_back(event);
    }

    pub fn get_history(&self, workflow_id: &str) -> Vec<&TransitionEvent> {
        self.entries
            .iter()
            .filter(|e| e.workflow_id == workflow_id)
            .collect()
    }

    pub fn get_recent(&self, count: usize) -> Vec<&TransitionEvent> {
        self.entries.iter().rev().take(count).collect()
    }

    pub fn count_transitions(&self, workflow_id: &str) -> usize {
        self.entries
            .iter()
            .filter(|e| e.workflow_id == workflow_id)
            .count()
    }

    pub fn has_been_rejected(&self, workflow_id: &str) -> bool {
        self.entries.iter().any(|e| {
            e.workflow_id == workflow_id && e.to == ApprovalState::Rejected
        })
    }

    pub fn last_transition(&self, workflow_id: &str) -> Option<&TransitionEvent> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.workflow_id == workflow_id)
    }

    pub fn clear_workflow(&mut self, workflow_id: &str) {
        self.entries.retain(|e| e.workflow_id != workflow_id);
    }

    pub fn total_entries(&self) -> usize {
        self.entries.len()
    }
}
