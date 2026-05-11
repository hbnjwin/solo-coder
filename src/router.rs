use anyhow::{bail, Result};

use crate::graph::get_node;
use crate::models::{
    ApprovalContext, Condition, ConditionOperator, RoutingDecision, RoutingGraph, RoutingState,
};

/// Resolves the next routing decision based on the current state and context.
/// Evaluates conditions on outgoing transitions to determine the correct path.
pub fn resolve_next(
    graph: &RoutingGraph,
    state: &RoutingState,
    context: &ApprovalContext,
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

    // Sort transitions by priority to ensure deterministic evaluation order
    let mut transitions = current_node.transitions.clone();
    transitions.sort_by_key(|t| t.priority);

    for transition in &transitions {
        match &transition.condition {
            Some(condition) => {
                if evaluate_condition(condition, context) {
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
                // Unconditional transition (fallback path)
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

    // No matching transition found, return unresolved
    Ok(RoutingDecision {
        current_node_id: current_node.id.clone(),
        next_node_id: None,
        approver: current_node.approver.clone(),
        resolved: false,
    })
}

/// Advances the routing state to the specified next node.
/// Validates that the current node is valid and that the transition is permitted
/// by the graph structure before updating state.
pub fn advance(
    graph: &RoutingGraph,
    state: &mut RoutingState,
    next_node_id: &str,
) -> Result<()> {
    // Verify current position is valid
    let current_node = get_node(graph, &state.current_node_id)?;

    // Check that the transition from current to next is allowed
    let transition_allowed = current_node
        .transitions
        .iter()
        .any(|t| t.target_node_id == next_node_id);

    if !transition_allowed {
        bail!(
            "transition from '{}' to '{}' is not permitted by the graph",
            state.current_node_id,
            next_node_id
        );
    }

    // Update state to reflect the advance
    state.current_node_id = next_node_id.to_string();
    state.history.push(next_node_id.to_string());
    Ok(())
}

/// Evaluates a condition against the text.
/// Returns true if the condition is satisfied by the context attributes.
fn evaluate_condition(condition: &Condition, context: &ApprovalContext) -> bool {
    let context_value = match context.attributes.get(&condition.field) {
        Some(val) => val,
        None => return false,
    };

    match condition.operator {
        ConditionOperator::GreaterThan => {
            compare_numeric(&condition.value, &condition.value, |a, b| a > b)
        }
        ConditionOperator::LessThan => {
            compare_numeric(&condition.value, &condition.value, |a, b| a < b)
        }
        ConditionOperator::Equals => context_value == &condition.value,
        ConditionOperator::NotEquals => context_value != &condition.value,
    }
}

/// Compares two JSON values as f64 numbers using the provided comparator.
fn compare_numeric(
    left: &serde_json::Value,
    right: &serde_json::Value,
    cmp: fn(f64, f64) -> bool,
) -> bool {
    let left_num = match left.as_f64() {
        Some(n) => n,
        None => return false,
    };
    let right_num = match right.as_f64() {
        Some(n) => n,
        None => return false,
    };
    cmp(left_num, right_num)
}
