use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;

use routing_condition_engine::graph::build_graph;
use routing_condition_engine::models::{ApprovalContext, NodeType, RoutingNode, RoutingState, Transition, ExprCondition};
use routing_condition_engine::router::resolve_next;

#[derive(Parser, Debug)]
#[command(name = "routing-condition-engine", about = "Approval routing with condition expressions and countersign")]
struct Cli {
    #[arg(long, default_value = "{}")]
    context: String,

    #[arg(long, default_value = "req-001")]
    request_id: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let entry = RoutingNode {
        id: "node-0".to_string(),
        label: "条件路由".to_string(),
        node_type: NodeType::Condition,
        approver: "router".to_string(),
        transitions: vec![
            Transition {
                target_node_id: "node-ceo".to_string(),
                condition: Some(ExprCondition {
                    expression: "amount > 10000 AND department == \"finance\"".to_string(),
                }),
                priority: 0,
            },
            Transition {
                target_node_id: "node-mgr".to_string(),
                condition: None,
                priority: 1,
            },
        ],
    };

    let ceo_node = RoutingNode {
        id: "node-ceo".to_string(),
        label: "CEO审批".to_string(),
        node_type: NodeType::Countersign { pass_ratio: 0.6 },
        approver: "ceo-group".to_string(),
        transitions: vec![],
    };

    let mgr_node = RoutingNode {
        id: "node-mgr".to_string(),
        label: "经理审批".to_string(),
        node_type: NodeType::Approval,
        approver: "manager".to_string(),
        transitions: vec![],
    };

    let graph = build_graph("g1", "conditional-with-countersign", "node-0", vec![entry, ceo_node, mgr_node])?;
    let state = RoutingState::new("g1", "node-0");

    let attrs: HashMap<String, serde_json::Value> = serde_json::from_str(&cli.context)?;
    let context = ApprovalContext {
        request_id: cli.request_id,
        attributes: attrs,
    };

    let decision = resolve_next(&graph, &state, &context, None)?;
    println!(
        "routing decision: approver={}, next={:?}, resolved={}",
        decision.approver, decision.next_node_id, decision.resolved
    );

    Ok(())
}
