use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalNode {
    pub id: String,
    pub name: String,
    pub approver: String,
    pub next_node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRoute {
    pub id: String,
    pub name: String,
    pub nodes: Vec<ApprovalNode>,
    pub current_node_idx: usize,
}

impl ApprovalRoute {
    pub fn new(id: &str, name: &str, nodes: Vec<ApprovalNode>) -> Self {
        Self { id: id.into(), name: name.into(), nodes, current_node_idx: 0 }
    }
    pub fn current_node(&self) -> Option<&ApprovalNode> {
        self.nodes.get(self.current_node_idx)
    }
    pub fn advance(&mut self) -> Result<&ApprovalNode> {
        if self.current_node_idx + 1 >= self.nodes.len() {
            anyhow::bail!("already at last node");
        }
        self.current_node_idx += 1;
        Ok(&self.nodes[self.current_node_idx])
    }
}

/// Build a simple linear approval route. No conditional branching yet.
pub fn build_linear_route(id: &str, approvers: Vec<&str>) -> ApprovalRoute {
    let nodes: Vec<ApprovalNode> = approvers.iter().enumerate().map(|(i, name)| ApprovalNode {
        id: format!("node-{}", i),
        name: format!("Approval step {}", i + 1),
        approver: name.to_string(),
        next_node_id: if i + 1 < approvers.len() { Some(format!("node-{}", i + 1)) } else { None },
    }).collect();
    ApprovalRoute::new(id, "default-route", nodes)
}

/// Resolve next approver. TODO: add conditional branching based on context.
pub fn resolve_next_approver(route: &ApprovalRoute, _context: &HashMap<String, serde_json::Value>) -> Result<String> {
    match route.current_node() {
        Some(node) => Ok(node.approver.clone()),
        None => anyhow::bail!("no current node in route"),
    }
}

#[derive(Parser, Debug)]
#[command(name = "approval-routing", about = "Approval routing engine")]
struct Cli {
    #[arg(long, default_value = "zhangsan,lisi,wangwu")]
    approvers: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let approvers: Vec<&str> = cli.approvers.split(',').collect();
    let route = build_linear_route("route-1", approvers);
    let ctx = HashMap::new();
    let approver = resolve_next_approver(&route, &ctx)?;
    println!("current approver: {}", approver);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_conditional_branching_missing() {
        let route = build_linear_route("r1", vec!["manager"]);
        let mut ctx = HashMap::new();
        ctx.insert("amount".into(), serde_json::json!(2000000));
        let approver = resolve_next_approver(&route, &ctx).unwrap();
        assert_eq!(approver, "manager"); // BUG: should be "ceo" for amount > 1M
    }
}
