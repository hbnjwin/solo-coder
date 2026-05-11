#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Generate 20 trae-solo projects with starter code"""

import os
import subprocess
import shutil

BASE_DIR = r"d:\work\github\solo-coder\projects\trae-solo"

def write_file(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

def run_cmd(cmd, cwd):
    subprocess.run(cmd, cwd=cwd, shell=True, capture_output=True)

# ==================== RUST PROJECTS ====================

# q6: workflow-state-machine
RUST_Q6_CARGO = '''[package]
name = "workflow-state-machine"
version = "0.1.0"
edition = "2021"

[dependencies]
'''
RUST_Q6_MAIN = '''use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApprovalState {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Completed,
}

#[derive(Debug)]
pub struct WorkflowInstance {
    pub id: String,
    pub state: ApprovalState,
    pub updated_at: Instant,
    pub version: u64,
}

pub struct StateMachine {
    transitions: HashMap<(ApprovalState, ApprovalState), bool>,
}

impl StateMachine {
    pub fn new() -> Self {
        let mut t = HashMap::new();
        t.insert((ApprovalState::Draft, ApprovalState::Submitted), true);
        t.insert((ApprovalState::Submitted, ApprovalState::UnderReview), true);
        t.insert((ApprovalState::UnderReview, ApprovalState::Approved), true);
        t.insert((ApprovalState::UnderReview, ApprovalState::Rejected), true);
        t.insert((ApprovalState::Rejected, ApprovalState::Submitted), true); // loop back
        t.insert((ApprovalState::Approved, ApprovalState::Completed), true);
        Self { transitions: t }
    }

    pub fn can_transition(&self, from: &ApprovalState, to: &ApprovalState) -> bool {
        self.transitions.get(&(from.clone(), to.clone())).copied().unwrap_or(false)
    }
}

pub fn transition(
    machine: &StateMachine,
    instance: &mut WorkflowInstance,
    target: ApprovalState,
) -> Result<(), String> {
    // BUG: no cycle detection - can loop forever if Rejected -> Submitted -> UnderReview -> Rejected
    let mut current = instance.state.clone();
    let mut steps = 0;
    while current != target {
        if steps > 100 {
            return Err("transition loop detected".to_string());
        }
        let mut found = false;
        for ((from, to), _) in &machine.transitions {
            if from == &current && machine.can_transition(&current, to) {
                current = to.clone();
                found = true;
                break;
            }
        }
        if !found {
            return Err(format!("no path from {:?} to {:?}", instance.state, target));
        }
        steps += 1;
    }
    instance.state = current;
    instance.updated_at = Instant::now();
    instance.version += 1;
    Ok(())
}

pub fn cleanup_stale_workflows(
    workflows: &mut HashMap<String, WorkflowInstance>,
) -> Vec<String> {
    let ttl = Duration::from_secs(3600);
    let now = Instant::now();
    let mut expired = Vec::new();
    // BUG: TTL check inverted (< instead of >)
    for (id, wf) in workflows.iter() {
        if now.duration_since(wf.updated_at) < ttl {
            expired.push(id.clone());
        }
    }
    for id in &expired {
        workflows.remove(id);
    }
    expired
}

fn main() {
    println!("workflow-state-machine starter");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_detection_rejected_loop() {
        let machine = StateMachine::new();
        let mut instance = WorkflowInstance {
            id: "wf1".to_string(),
            state: ApprovalState::Submitted,
            updated_at: Instant::now(),
            version: 1,
        };
        // Rejected -> Submitted -> UnderReview -> Rejected should be caught
        transition(&machine, &mut instance, ApprovalState::UnderReview).unwrap();
        transition(&machine, &mut instance, ApprovalState::Rejected).unwrap();
        let result = transition(&machine, &mut instance, ApprovalState::UnderReview);
        // This will currently succeed due to missing cycle detection
        // After fix, it should fail with "transition loop detected"
        assert!(result.is_err(), "cycle should be detected");
    }

    #[test]
    fn test_cleanup_removes_stale_only() {
        let mut workflows = HashMap::new();
        workflows.insert("old".to_string(), WorkflowInstance {
            id: "old".to_string(),
            state: ApprovalState::Draft,
            updated_at: Instant::now() - Duration::from_secs(7200),
            version: 1,
        });
        workflows.insert("fresh".to_string(), WorkflowInstance {
            id: "fresh".to_string(),
            state: ApprovalState::Draft,
            updated_at: Instant::now(),
            version: 1,
        });
        let expired = cleanup_stale_workflows(&mut workflows);
        assert!(expired.iter().any(|id| id == "old"), "stale should be removed");
        assert!(!expired.iter().any(|id| id == "fresh"), "fresh should not be removed");
        assert!(!workflows.contains_key("old"), "old should be gone");
        assert!(workflows.contains_key("fresh"), "fresh should remain");
    }
}
'''

# ... (writing just the first one as an example, then using a script)

projects = []

# q6
projects.append(("workflow-state-machine", RUST_Q6_CARGO, RUST_Q6_MAIN, None))

# Let me write a comprehensive script instead
print("Script template created")
