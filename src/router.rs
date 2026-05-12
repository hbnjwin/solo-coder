use anyhow::{bail, Result};
use std::collections::HashMap;

use crate::countersign::evaluate_countersign;
use crate::expr::evaluate_expression;
use crate::graph::get_node;
use crate::models::{ApprovalContext, NodeType, RoutingDecision, RoutingGraph, RoutingState};

pub fn resolve_next(
    graph: &RoutingGraph,
    state: &RoutingState,
    context: &ApprovalContext,
    votes: Option<&HashMap<String, Vec<crate::models::CountersignVote>>>,
) -> Result<RoutingDecision> {
    let current_node = get_node(graph, &state.current_node_id)?;

    if current_node.transitions.is_empty() {
        return Ok(RoutingDecision {
            current_node_id: current_node.id.clone(),
            next_node_id: None,
            approver: current_node.approver.clone(),
            resolved: true,
        });
    }

    match &current_node.node_type {
        NodeType::Countersign { pass_ratio } => {
            let vote_list = votes
                .and_then(|v| v.get(&current_node.id))
                .cloned()
                .unwrap_or_default();
            let result = evaluate_countersign(&current_node.id, &vote_list, *pass_ratio);
            if !result.passed {
                return Ok(RoutingDecision {
                    current_node_id: current_node.id.clone(),
                    next_node_id: None,
                    approver: current_node.approver.clone(),
                    resolved: false,
                });
            }
        }
        _ => {}
    }

    let mut transitions = current_node.transitions.clone();
    transitions.sort_by_key(|t| t.priority);

    for transition in &transitions {
        match &transition.condition {
            Some(expr_cond) => {
                let matched = evaluate_expression(&expr_cond.expression, &context.attributes)?;
                if matched {
                    let target = get_node(graph, &transition.target_node_id)?;
                    return Ok(RoutingDecision {
                        current_node_id: current_node.id.clone(),
                        next_node_id: Some(target.id.clone()),
                        approver: target.approver.clone(),
                        resolved: true,
                    });
                }
            }
            None => {
                let target = get_node(graph, &transition.target_node_id)?;
                return Ok(RoutingDecision {
                    current_node_id: current_node.id.clone(),
                    next_node_id: Some(target.id.clone()),
                    approver: target.approver.clone(),
                    resolved: true,
                });
            }
        }
    }

    Ok(RoutingDecision {
        current_node_id: current_node.id.clone(),
        next_node_id: None,
        approver: current_node.approver.clone(),
        resolved: false,
    })
}

pub fn advance(
    graph: &RoutingGraph,
    state: &mut RoutingState,
    next_node_id: &str,
) -> Result<()> {
    let current_node = get_node(graph, &state.current_node_id)?;

    let transition_allowed = current_node
        .transitions
        .iter()
        .any(|t| t.target_node_id == next_node_id);

    if !transition_allowed {
        bail!("transition from '{}' to '{}' is not permitted by the graph", state.current_node_id, next_node_id);
    }

    state.current_node_id = next_node_id.to_string();
    state.history.push(next_node_id.to_string());
    Ok(())
}
