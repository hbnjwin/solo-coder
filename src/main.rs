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

/// BUG: only checks id and nodes empty, doesn't check duplicate node IDs
pub fn validate_structure(config: &WorkflowConfig) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    if config.id.is_empty() {
        issues.push(ValidationIssue { severity: ValidationSeverity::Error, node_id: None, message: "config id is empty".into(), suggestion: Some("add a non-empty id".into()) });
    }
    if config.nodes.is_empty() {
        issues.push(ValidationIssue { severity: ValidationSeverity::Error, node_id: None, message: "no nodes defined".into(), suggestion: Some("add at least start and end nodes".into()) });
    }
    // BUG: no duplicate node ID check
    issues
}

/// BUG: not implemented - should check reachability, orphan nodes, etc.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_node_id_not_detected() {
        let config = WorkflowConfig {
            id: "wf-1".into(), name: "test".into(), start_node: "n1".into(),
            nodes: vec![
                NodeConfig { id: "n1".into(), node_type: "start".into(), next_node: Some("n2".into()), approver: None, condition: None },
                NodeConfig { id: "n1".into(), node_type: "approval".into(), next_node: None, approver: Some("zhangsan".into()), condition: None },
            ],
        };
        let issues = validate_structure(&config);
        assert!(issues.iter().any(|i| i.message.contains("duplicate")), "should detect duplicate node IDs");
    }

    #[test]
    fn test_orphan_node_not_detected() {
        let config = WorkflowConfig {
            id: "wf-1".into(), name: "test".into(), start_node: "n1".into(),
            nodes: vec![
                NodeConfig { id: "n1".into(), node_type: "start".into(), next_node: Some("n2".into()), approver: None, condition: None },
                NodeConfig { id: "n2".into(), node_type: "end".into(), next_node: None, approver: None, condition: None },
                NodeConfig { id: "n3".into(), node_type: "approval".into(), next_node: None, approver: Some("zhangsan".into()), condition: None },
            ],
        };
        let issues = validate_logic(&config);
        assert!(issues.iter().any(|i| i.message.contains("orphan") || i.message.contains("unreachable")), "should detect orphan node n3");
    }

    #[test]
    fn test_end_node_not_reachable() {
        let config = WorkflowConfig {
            id: "wf-1".into(), name: "test".into(), start_node: "n1".into(),
            nodes: vec![
                NodeConfig { id: "n1".into(), node_type: "start".into(), next_node: None, approver: None, condition: None },
                NodeConfig { id: "n2".into(), node_type: "end".into(), next_node: None, approver: None, condition: None },
            ],
        };
        let issues = validate_logic(&config);
        assert!(issues.iter().any(|i| i.message.contains("reachable") || i.message.contains("reach")), "should detect unreachable end node");
    }
}
