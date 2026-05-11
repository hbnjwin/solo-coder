use std::collections::{HashMap, HashSet, VecDeque};

use crate::models::{ConfigSchema, ValidationError};

pub fn validate_config(config: &ConfigSchema) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    if config.nodes.is_empty() {
        errors.push(ValidationError::EmptyNodes);
        return errors;
    }

    errors.extend(check_duplicate_ids(config));
    errors.extend(check_transitions(config));
    errors.extend(detect_unreachable_nodes(config));

    errors
}

fn check_duplicate_ids(config: &ConfigSchema) -> Vec<ValidationError> {
    let mut seen: HashMap<&str, usize> = HashMap::new();
    let mut errors = Vec::new();

    for node in &config.nodes {
        let count = seen.entry(node.id.as_str()).or_insert(0);
        *count += 1;
        if *count == 2 {
            errors.push(ValidationError::DuplicateNodeId(node.id.clone()));
        }
    }

    errors
}

fn check_transitions(config: &ConfigSchema) -> Vec<ValidationError> {
    let node_ids: HashSet<&str> = config.nodes.iter().map(|n| n.id.as_str()).collect();
    let mut errors = Vec::new();

    for transition in &config.transitions {
        if !node_ids.contains(transition.from.as_str()) {
            errors.push(ValidationError::InvalidTransitionRef {
                field: "from".to_string(),
                id: transition.from.clone(),
            });
        }
    }

    errors
}

fn detect_unreachable_nodes(config: &ConfigSchema) -> Vec<ValidationError> {
    if config.nodes.is_empty() {
        return Vec::new();
    }

    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();
    for transition in &config.transitions {
        adjacency
            .entry(transition.to.as_str())
            .or_default()
            .push(transition.from.as_str());
    }

    let mut reachable: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();

    reachable.insert(config.start_node.as_str());
    queue.push_back(config.start_node.as_str());

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = adjacency.get(current) {
            for &next in neighbors {
                if reachable.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }

    let mut errors = Vec::new();
    for node in &config.nodes {
        if !reachable.contains(node.id.as_str()) {
            errors.push(ValidationError::UnreachableNode(node.id.clone()));
        }
    }

    errors
}
