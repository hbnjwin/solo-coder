use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig { pub id: String, pub name: String, pub start_node: String, pub nodes: Vec<NodeConfig> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig { pub id: String, pub node_type: String, pub next_node: Option<String>, pub approver: Option<String>, pub condition: Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationSeverity { Error, Warning }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue { pub severity: ValidationSeverity, pub node_id: Option<String>, pub message: String, pub suggestion: Option<String> }

/// TODO: only basic checks implemented
pub fn validate_structure(config: &WorkflowConfig) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    if config.id.is_empty() {
        issues.push(ValidationIssue { severity: ValidationSeverity::Error, node_id: None, message: "config id is empty".into(), suggestion: Some("add a non-empty id".into()) });
    }
    if config.nodes.is_empty() {
        issues.push(ValidationIssue { severity: ValidationSeverity::Error, node_id: None, message: "no nodes defined".into(), suggestion: Some("add at least start and end nodes".into()) });
    }
    issues
}

/// TODO: not implemented - should check reachability, orphan nodes, etc.
pub fn validate_logic(_config: &WorkflowConfig) -> Vec<ValidationIssue> { vec![] }

#[derive(Parser, Debug)]
#[command(name = "config-validator", about = "Workflow config validator")]
struct Cli { #[arg(long, default_value = "config.json")] config_file: String }

fn main() -> Result<()> {
    let sample = WorkflowConfig {
        id: "wf-1".into(), name: "Sample".into(), start_node: "start".into(),
        nodes: vec![
            NodeConfig { id: "start".into(), node_type: "start".into(), next_node: Some("end".into()), approver: None, condition: None },
            NodeConfig { id: "end".into(), node_type: "end".into(), next_node: None, approver: None, condition: None },
        ],
    };
    for i in validate_structure(&sample) { println!("{:?}: {}", i.severity, i.message); }
    Ok(())
}
