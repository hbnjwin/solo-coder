use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Record = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    NotEmpty,
    Contains,
    StartsWith,
}

impl Operator {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "eq" => Some(Self::Eq),
            "neq" => Some(Self::Neq),
            "gt" => Some(Self::Gt),
            "lt" => Some(Self::Lt),
            "gte" => Some(Self::Gte),
            "lte" => Some(Self::Lte),
            "not_empty" => Some(Self::NotEmpty),
            "contains" => Some(Self::Contains),
            "starts_with" => Some(Self::StartsWith),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Eq => "equals",
            Self::Neq => "not equals",
            Self::Gt => "greater than",
            Self::Lt => "less than",
            Self::Gte => "greater than or equal",
            Self::Lte => "less than or equal",
            Self::NotEmpty => "is not empty",
            Self::Contains => "contains",
            Self::StartsWith => "starts with",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub id: String,
    pub field: String,
    pub operator: Operator,
    pub value: serde_json::Value,
    pub description: Option<String>,
}

impl ValidationRule {
    pub fn new(id: &str, field: &str, op: Operator, value: serde_json::Value) -> Self {
        Self {
            id: id.into(),
            field: field.into(),
            operator: op,
            value,
            description: None,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub rule_id: String,
    pub passed: bool,
    pub message: String,
    pub field: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChainMode {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleChain {
    pub id: String,
    pub mode: ChainMode,
    pub rules: Vec<ValidationRule>,
}

impl RuleChain {
    pub fn new(id: &str, mode: ChainMode) -> Self {
        Self {
            id: id.into(),
            mode,
            rules: Vec::new(),
        }
    }

    pub fn add_rule(mut self, rule: ValidationRule) -> Self {
        self.rules.push(rule);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainResult {
    pub chain_id: String,
    pub mode: ChainMode,
    pub passed: bool,
    pub details: Vec<ValidationResult>,
}
