use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete deal content document with metadata and a root node tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealContent {
    pub version: String,
    pub schema_id: String,
    pub root: DealNode,
    pub metadata: HashMap<String, String>,
}

/// A single node in the deal content tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub fields: Vec<DealField>,
    pub children: Vec<DealNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Root,
    Section,
    Approval,
    Condition,
    Action,
    Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealField {
    pub key: String,
    pub value: FieldValue,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    Text(String),
    Number(f64),
    Flag(bool),
    Null,
}

impl Default for DealNode {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            node_type: NodeType::Section,
            fields: Vec::new(),
            children: Vec::new(),
            ref_target: None,
        }
    }
}

impl Default for DealContent {
    fn default() -> Self {
        Self {
            version: String::from("1.0"),
            schema_id: String::new(),
            root: DealNode::default(),
            metadata: HashMap::new(),
        }
    }
}

impl DealNode {
    pub fn descendant_count(&self) -> usize {
        self.children.iter().map(|c| 1 + c.descendant_count()).sum()
    }
}
