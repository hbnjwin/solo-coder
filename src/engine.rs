use crate::models::*;
use anyhow::Result;
use std::collections::HashMap;

pub struct StateMachine {
    rules: Vec<TransitionRule>,
}

impl StateMachine {
    pub fn new() -> Self {
        let rules = vec![
            TransitionRule::new(ApprovalState::Draft, ApprovalState::Submitted),
            TransitionRule::new(ApprovalState::Submitted, ApprovalState::UnderReview),
            TransitionRule::new(ApprovalState::UnderReview, ApprovalState::Approved),
            TransitionRule::new(ApprovalState::UnderReview, ApprovalState::Rejected)
                .with_comment_required(),
            TransitionRule::new(ApprovalState::UnderReview, ApprovalState::Escalated)
                .with_min_priority(Priority::High),
            TransitionRule::new(ApprovalState::Rejected, ApprovalState::Submitted),
            TransitionRule::new(ApprovalState::Escalated, ApprovalState::Approved),
            TransitionRule::new(ApprovalState::Escalated, ApprovalState::Rejected)
                .with_comment_required(),
            TransitionRule::new(ApprovalState::Approved, ApprovalState::Completed),
        ];
        Self { rules }
    }

    pub fn get_available_transitions(&self, from: &ApprovalState) -> Vec<&TransitionRule> {
        self.rules.iter().filter(|r| &r.from == from).collect()
    }

    pub fn can_transition(&self, from: &ApprovalState, to: &ApprovalState) -> bool {
        self.rules.iter().any(|r| &r.from == from && &r.to == to)
    }

    pub fn validate_transition(
        &self,
        instance: &WorkflowInstance,
        to: &ApprovalState,
        comment: Option<&str>,
    ) -> Result<()> {
        let rule = self
            .rules
            .iter()
            .find(|r| r.from == instance.state && r.to == *to)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "no transition from {} to {} allowed",
                    instance.state,
                    to
                )
            })?;

        if rule.requires_comment && comment.is_none() {
            anyhow::bail!("transition to {} requires a comment", to);
        }

        if let Some(ref min_p) = rule.min_priority {
            let priority_rank = |p: &Priority| match p {
                Priority::Low => 0,
                Priority::Normal => 1,
                Priority::High => 2,
                Priority::Urgent => 3,
            };
            if priority_rank(&instance.priority) < priority_rank(min_p) {
                anyhow::bail!(
                    "transition to {} requires at least {:?} priority",
                    to,
                    min_p
                );
            }
        }

        Ok(())
    }
}

fn can_reach(
    machine: &StateMachine,
    from: &ApprovalState,
    target: &ApprovalState,
) -> bool {
    let mut visited = std::collections::HashSet::new();
    let mut queue = vec![from.clone()];
    visited.insert(from.clone());

    while let Some(current) = queue.pop() {
        for rule in machine.get_available_transitions(&current) {
            if &rule.to == target {
                return true;
            }
            if !visited.contains(&rule.to) {
                visited.insert(rule.to.clone());
                queue.push(rule.to.clone());
            }
        }
    }
    false
}

fn path_would_create_cycle(machine: &StateMachine, path: &[ApprovalState]) -> bool {
    if path.len() <= 2 {
        return false;
    }

    for (i, state) in path.iter().enumerate() {
        for rule in machine.get_available_transitions(state) {
            if path[..i].contains(&rule.to) {
                return true;
            }
        }
    }
    false
}

pub fn find_path(
    machine: &StateMachine,
    from: &ApprovalState,
    to: &ApprovalState,
) -> Option<Vec<ApprovalState>> {
    if from == to {
        return Some(vec![from.clone()]);
    }

    let mut queue: Vec<Vec<ApprovalState>> = vec![vec![from.clone()]];
    let mut visited = std::collections::HashSet::new();
    visited.insert(from.clone());

    while !queue.is_empty() {
        let path = queue.remove(0);
        let current = path.last().unwrap();

        for rule in machine.get_available_transitions(current) {
            if visited.contains(&rule.to) {
                continue;
            }
            visited.insert(rule.to.clone());

            let mut new_path = path.clone();
            new_path.push(rule.to.clone());

            if path_would_create_cycle(machine, &new_path) {
                continue;
            }

            if rule.to == *to {
                return Some(new_path);
            }

            queue.push(new_path);
        }
    }

    None
}

pub fn auto_advance(
    machine: &StateMachine,
    instance: &mut WorkflowInstance,
    target: ApprovalState,
) -> Result<Vec<TransitionEvent>> {
    let path = find_path(machine, &instance.state, &target)
        .ok_or_else(|| anyhow::anyhow!("no path from {} to {}", instance.state, target))?;

    let mut events = Vec::new();

    for next_state in path.iter().skip(1) {
        let event = TransitionEvent {
            workflow_id: instance.id.clone(),
            from: instance.state.clone(),
            to: next_state.clone(),
            timestamp: instance.updated_at.clone(),
            actor: instance.assignee.clone(),
            comment: None,
        };
        instance.state = next_state.clone();
        instance.version += 1;
        events.push(event);
    }

    Ok(events)
}

pub fn batch_advance(
    machine: &StateMachine,
    instances: &mut HashMap<String, WorkflowInstance>,
    target: ApprovalState,
) -> HashMap<String, Result<Vec<TransitionEvent>>> {
    let ids: Vec<String> = instances.keys().cloned().collect();
    let mut results = HashMap::new();

    for id in ids {
        let instance = instances.get_mut(&id).unwrap();
        let result = auto_advance(machine, instance, target.clone());
        results.insert(id, result);
    }

    results
}
