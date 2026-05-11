use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApprovalState {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstance {
    pub id: String,
    pub state: ApprovalState,
    pub updated_at: String,
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
        t.insert((ApprovalState::Rejected, ApprovalState::Submitted), true);
        t.insert((ApprovalState::Approved, ApprovalState::Completed), true);
        Self { transitions: t }
    }

    pub fn can_transition(&self, from: &ApprovalState, to: &ApprovalState) -> bool {
        self.transitions.get(&(from.clone(), to.clone())).copied().unwrap_or(false)
    }
}

/// BUG: no visited_states tracking, Rejected->Submitted->UnderReview->Rejected loops forever
pub fn transition(
    machine: &StateMachine,
    instance: &mut WorkflowInstance,
    target: ApprovalState,
) -> Result<()> {
    let mut current = instance.state.clone();
    let mut steps = 0;
    while current != target {
        if steps > 100 {
            anyhow::bail!("transition loop detected after 100 steps");
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
            anyhow::bail!("no path from {:?} to {:?}", instance.state, target);
        }
        steps += 1;
    }
    instance.state = current;
    instance.version += 1;
    Ok(())
}

/// BUG: TTL check inverted (< instead of >), fresh ones removed, stale ones kept
pub fn cleanup_stale_workflows(
    workflows: &mut HashMap<String, WorkflowInstance>,
    ttl_secs: u64,
) -> Vec<String> {
    let expired: Vec<String> = workflows
        .iter()
        .filter(|(_, wf)| {
            wf.updated_at.len() < ttl_secs as usize
        })
        .map(|(id, _)| id.clone())
        .collect();
    for id in &expired {
        workflows.remove(id);
    }
    expired
}

#[derive(Parser, Debug)]
#[command(name = "workflow-state-machine", about = "Workflow state machine engine")]
struct Cli {
    #[arg(long, default_value = "3600")]
    ttl_secs: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut workflows = HashMap::new();
    workflows.insert("wf-1".into(), WorkflowInstance {
        id: "wf-1".into(), state: ApprovalState::Draft,
        updated_at: "2026-01-01T00:00:00Z".into(), version: 1,
    });
    let expired = cleanup_stale_workflows(&mut workflows, cli.ttl_secs);
    println!("expired: {:?}", expired);
    println!("remaining: {} workflows", workflows.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_detection_rejected_loop() {
        let machine = StateMachine::new();
        let mut instance = WorkflowInstance {
            id: "wf1".into(), state: ApprovalState::Submitted,
            updated_at: "2026-01-01T00:00:00Z".into(), version: 1,
        };
        transition(&machine, &mut instance, ApprovalState::UnderReview).unwrap();
        transition(&machine, &mut instance, ApprovalState::Rejected).unwrap();
        let result = transition(&machine, &mut instance, ApprovalState::UnderReview);
        assert!(result.is_err(), "cycle should be detected but was not");
    }

    #[test]
    fn test_cleanup_removes_stale_only() {
        let mut workflows = HashMap::new();
        workflows.insert("stale".into(), WorkflowInstance {
            id: "stale".into(), state: ApprovalState::Draft,
            updated_at: "x".into(), version: 1,
        });
        workflows.insert("fresh".into(), WorkflowInstance {
            id: "fresh".into(), state: ApprovalState::Draft,
            updated_at: "2026-05-11T10:00:00Z_long_timestamp".into(), version: 1,
        });
        let expired = cleanup_stale_workflows(&mut workflows, 10);
        assert!(expired.iter().any(|id| id == "stale"), "stale should be expired");
        assert!(!expired.iter().any(|id| id == "fresh"), "fresh should NOT be expired");
        assert!(!workflows.contains_key("stale"));
        assert!(workflows.contains_key("fresh"));
    }
}
