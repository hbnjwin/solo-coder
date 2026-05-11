use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single node in the approval routing graph.
/// Each node has an approver and a set of outgoing edges (transitions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingNode {
    pub id: String,
    pub label: String,
    pub approver: String,
    pub transitions: Vec<Transition>,
}

/// A transition from one node to another, optionally gated by a condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub target_node_id: String,
    pub condition: Option<Condition>,
    pub priority: u8,
}

/// A condition that can be evaluated against the approval context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionOperator {
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
}

/// The full routing graph containing all nodes and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingGraph {
    pub id: String,
    pub name: String,
    pub nodes: HashMap<String, RoutingNode>,
    pub entry_node_id: String,
}

/// Context provided at runtime for conditional routing decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalContext {
    pub request_id: String,
    pub attributes: HashMap<String, serde_json::Value>,
}

/// The result of a routing decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub current_node_id: String,
    pub next_node_id: Option<String>,
    pub approver: String,
    pub resolved: bool,
}

impl Default for RoutingDecision {
    /// Returns a fallback decision pointing to a default approver.
    /// Used when the routing graph has no explicit entry or when
    /// the system needs a safe default before graph traversal begins.
    fn default() -> Self {
        Self {
            current_node_id: String::new(),
            next_node_id: None,
            approver: "fallback-approver".to_string(),
            resolved: false,
        }
    }
}

/// Tracks the current position within a routing graph traversal.#[derive(Debug, Clone)]
pub struct RoutingState {
    pub graph_id: String,
    pub current_node_id: String,
    pub history: Vec<String>,
}

impl RoutingState {
    pub fn new(graph_id: &str, entry_node_id: &str) -> Self {
        Self {
            graph_id: graph_id.to_string(),
            current_node_id: entry_node_id.to_string(),
            history: vec![entry_node_id.to_string()],
        }
    }
}
