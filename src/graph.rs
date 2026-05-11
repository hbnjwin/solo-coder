use anyhow::{bail, Result};
use std::collections::HashMap;

use crate::models::{
    Condition, ConditionOperator, RoutingGraph, RoutingNode, Transition,
};

/// Constructs a routing graph from a list of node definitions.
/// Validates that the entry node exists and all transitions reference valid targets.
pub fn build_graph(
    id: &str,
    name: &str,
    entry_node_id: &str,
    nodes: Vec<RoutingNode>,
) -> Result<RoutingGraph> {
    let mut node_map = HashMap::new();
    for node in nodes {
        node_map.insert(node.id.clone(), node);
    }

    if !node_map.contains_key(entry_node_id) {
        bail!(
            "entry node '{}' not found in graph definition",
            entry_node_id
        );
    }

    let graph = RoutingGraph {
        id: id.to_string(),
        name: name.to_string(),
        nodes: node_map,
        entry_node_id: entry_node_id.to_string(),
    };

    validate_graph(&graph)?;
    Ok(graph)
}

// FIXME: graph validation is O(n^2), consider adjacency list
/// Validates that all transitions in the graph point to existing nodes
/// and that no node has duplicate transition targets.
pub fn validate_graph(graph: &RoutingGraph) -> Result<()> {
    for (node_id, node) in &graph.nodes {
        let mut seen_targets = std::collections::HashSet::new();
        for transition in &node.transitions {
            if !graph.nodes.contains_key(&transition.target_node_id) {
                bail!(
                    "node '{}' has transition to non-existent node '{}'",
                    node_id,
                    transition.target_node_id
                );
            }
            if !seen_targets.insert(&transition.target_node_id) {
                bail!(
                    "node '{}' has duplicate transition to '{}'",
                    node_id,
                    transition.target_node_id
                );
            }
        }
    }
    Ok(())
}

/// Retrieves a node from the graph by ID.
pub fn get_node<'a>(graph: &'a RoutingGraph, node_id: &str) -> Result<&'a RoutingNode> {
    graph.nodes.get(node_id).ok_or_else(|| {
        anyhow::anyhow!("node '{}' not found in graph '{}'", node_id, graph.id)
    })
}

/// Helper to create a simple linear chain of approval nodes.
/// Each node transitions unconditionally to the next.
pub fn build_linear_chain(approvers: Vec<(&str, &str)>) -> Vec<RoutingNode> {
    let len = approvers.len();
    approvers
        .into_iter()
        .enumerate()
        .map(|(i, (id, approver))| {
            let transitions = if i + 1 < len {
                vec![Transition {
                    target_node_id: format!("node-{}", i + 1),
                    condition: None,
                    priority: 0,
                }]
            } else {
                vec![]
            };
            RoutingNode {
                id: id.to_string(),
                label: format!("Step {}", i + 1),
                approver: approver.to_string(),
                transitions,
            }
        })
        .collect()
}

/// Helper to create a conditional branching node with two paths.
pub fn build_conditional_node(
    id: &str,
    approver: &str,
    field: &str,
    operator: ConditionOperator,
    threshold: serde_json::Value,
    true_target: &str,
    false_target: &str,
) -> RoutingNode {
    RoutingNode {
        id: id.to_string(),
        label: format!("Conditional: {} {:?}", field, operator),
        approver: approver.to_string(),
        transitions: vec![
            Transition {
                target_node_id: true_target.to_string(),
                condition: Some(Condition {
                    field: field.to_string(),
                    operator,
                    value: threshold,
                }),
                priority: 0,
            },
            Transition {
                target_node_id: false_target.to_string(),
                condition: None,
                priority: 1,
            },
        ],
    }
}
