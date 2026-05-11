use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigSchema {
    pub name: String,
    pub start_node: String,
    pub nodes: Vec<NodeConfig>,
    pub transitions: Vec<TransitionConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NodeConfig {
    pub id: String,
    pub node_type: String,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransitionConfig {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub condition: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    EmptyNodes,
    DuplicateNodeId(String),
    InvalidTransitionRef { field: String, id: String },
    UnreachableNode(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyNodes => write!(f, "configuration contains no nodes"),
            ValidationError::DuplicateNodeId(id) => {
                write!(f, "duplicate node identifier: {}", id)
            }
            ValidationError::InvalidTransitionRef { field, id } => {
                write!(f, "transition references unknown {} node: {}", field, id)
            }
            ValidationError::UnreachableNode(id) => {
                write!(f, "node is unreachable from start: {}", id)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
