use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;

use approval_routing::graph::build_graph;
use approval_routing::models::{ApprovalContext, RoutingNode, RoutingState, Transition};
use approval_routing::router::resolve_next;

#[derive(Parser, Debug)]
#[command(name = "approval-routing", about = "Approval routing engine")]
struct Cli {
    /// Comma-separated list of approvers for a linear chain
    #[arg(long, default_value = "manager,director,vp")]
    approvers: String,

    /// JSON string of context attributes for conditional routing
    #[arg(long, default_value = "{}")]
    context: String,

    /// Request ID for tracking
    #[arg(long, default_value = "req-001")]
    request_id: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let approvers: Vec<&str> = cli.approvers.split(',').collect();
    let nodes: Vec<RoutingNode> = approvers
        .iter()
        .enumerate()
        .map(|(i, approver)| {
            let transitions = if i + 1 < approvers.len() {
                vec![Transition {
                    target_node_id: format!("node-{}", i + 1),
                    condition: None,
                    priority: 0,
                }]
            } else {
                vec![]
            };
            RoutingNode {
                id: format!("node-{}", i),
                label: format!("Level {} approval", i + 1),
                approver: approver.trim().to_string(),
                transitions,
            }
        })
        .collect();

    let graph = build_graph("graph-1", "default", "node-0", nodes)?;
    let state = RoutingState::new("graph-1", "node-0");

    let attrs: HashMap<String, serde_json::Value> =
        serde_json::from_str(&cli.context)?;
    let context = ApprovalContext {
        request_id: cli.request_id,
        attributes: attrs,
    };

    let decision = resolve_next(&graph, &state, &context)?;
    println!(
        "routing decision: approver={}, next={:?}, resolved={}",
        decision.approver, decision.next_node_id, decision.resolved
    );

    Ok(())
}
