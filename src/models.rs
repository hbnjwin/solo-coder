use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingNode {
    pub id: String,
    pub label: String,
    pub node_type: NodeType,
    pub approver: String,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Approval,
    Condition,
    Countersign { pass_ratio: f64 },
    CC,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Approval
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub target_node_id: String,
    pub condition: Option<ExprCondition>,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExprCondition {
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingGraph {
    pub id: String,
    pub name: String,
    pub nodes: HashMap<String, RoutingNode>,
    pub entry_node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalContext {
    pub request_id: String,
    pub attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub current_node_id: String,
    pub next_node_id: Option<String>,
    pub approver: String,
    pub resolved: bool,
}

impl Default for RoutingDecision {
    fn default() -> Self {
        Self {
            current_node_id: String::new(),
            next_node_id: None,
            approver: "fallback-approver".to_string(),
            resolved: false,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountersignVote {
    pub voter: String,
    pub approved: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountersignResult {
    pub node_id: String,
    pub total_voters: usize,
    pub approved_count: usize,
    pub rejected_count: usize,
    pub pass_ratio: f64,
    pub passed: bool,
}
