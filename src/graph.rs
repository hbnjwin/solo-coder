use anyhow::{bail, Result};
use std::collections::HashMap;

use crate::models::{NodeType, RoutingGraph, RoutingNode, Transition};

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
        bail!("entry node '{}' not found in graph definition", entry_node_id);
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

pub fn validate_graph(graph: &RoutingGraph) -> Result<()> {
    for (node_id, node) in &graph.nodes {
        let mut seen_targets = std::collections::HashSet::new();
        for transition in &node.transitions {
            if !graph.nodes.contains_key(&transition.target_node_id) {
                bail!("node '{}' has transition to non-existent node '{}'", node_id, transition.target_node_id);
            }
            if !seen_targets.insert(&transition.target_node_id) {
                bail!("node '{}' has duplicate transition to '{}'", node_id, transition.target_node_id);
            }
        }
        if let NodeType::Countersign { pass_ratio } = node.node_type {
            if pass_ratio <= 0.0 || pass_ratio > 1.0 {
                bail!("node '{}' has invalid pass_ratio {}, must be in (0, 1]", node_id, pass_ratio);
            }
        }
    }
    Ok(())
}

pub fn get_node<'a>(graph: &'a RoutingGraph, node_id: &str) -> Result<&'a RoutingNode> {
    graph.nodes.get(node_id).ok_or_else(|| {
        anyhow::anyhow!("node '{}' not found in graph '{}'", node_id, graph.id)
    })
}

pub fn build_linear_chain(approvers: Vec<(&str, &str)>) -> Vec<RoutingNode> {
    let len = approvers.len();
    let ids: Vec<String> = approvers.iter().map(|(id, _)| id.to_string()).collect();
    approvers
        .into_iter()
        .enumerate()
        .map(|(i, (id, approver))| {
            let transitions = if i + 1 < len {
                vec![Transition {
                    target_node_id: ids[i + 1].clone(),
                    condition: None,
                    priority: 0,
                }]
            } else {
                vec![]
            };
            RoutingNode {
                id: id.to_string(),
                label: format!("Step {}", i + 1),
                node_type: NodeType::Approval,
                approver: approver.to_string(),
                transitions,
            }
        })
        .collect()
}
