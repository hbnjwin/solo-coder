use std::collections::HashMap;

use routing_condition_engine::countersign::evaluate_countersign;
use routing_condition_engine::expr::evaluate_expression;
use routing_condition_engine::graph::{build_graph, build_linear_chain};
use routing_condition_engine::models::*;
use routing_condition_engine::router::{advance, resolve_next};
use serde_json::json;

#[test]
fn test_expression_routing() {
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
        label: "CEO".to_string(),
        node_type: NodeType::Approval,
        approver: "ceo".to_string(),
        transitions: vec![],
    };

    let mgr_node = RoutingNode {
        id: "node-mgr".to_string(),
        label: "Manager".to_string(),
        node_type: NodeType::Approval,
        approver: "manager".to_string(),
        transitions: vec![],
    };

    let graph = build_graph("g1", "expr-test", "node-0", vec![entry, ceo_node, mgr_node]).unwrap();
    let state = RoutingState::new("g1", "node-0");

    let mut attrs = HashMap::new();
    attrs.insert("amount".to_string(), json!(20000));
    attrs.insert("department".to_string(), json!("finance"));
    let context = ApprovalContext { request_id: "r1".to_string(), attributes: attrs };

    let decision = resolve_next(&graph, &state, &context, None).unwrap();
    assert_eq!(decision.next_node_id, Some("node-ceo".to_string()));
}

#[test]
fn test_expression_routing_fallback() {
    let entry = RoutingNode {
        id: "node-0".to_string(),
        label: "条件路由".to_string(),
        node_type: NodeType::Condition,
        approver: "router".to_string(),
        transitions: vec![
            Transition {
                target_node_id: "node-ceo".to_string(),
                condition: Some(ExprCondition {
                    expression: "amount > 10000".to_string(),
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
        label: "CEO".to_string(),
        node_type: NodeType::Approval,
        approver: "ceo".to_string(),
        transitions: vec![],
    };

    let mgr_node = RoutingNode {
        id: "node-mgr".to_string(),
        label: "Manager".to_string(),
        node_type: NodeType::Approval,
        approver: "manager".to_string(),
        transitions: vec![],
    };

    let graph = build_graph("g2", "fallback-test", "node-0", vec![entry, ceo_node, mgr_node]).unwrap();
    let state = RoutingState::new("g2", "node-0");

    let mut attrs = HashMap::new();
    attrs.insert("amount".to_string(), json!(5000));
    let context = ApprovalContext { request_id: "r2".to_string(), attributes: attrs };

    let decision = resolve_next(&graph, &state, &context, None).unwrap();
    assert_eq!(decision.next_node_id, Some("node-mgr".to_string()));
}

#[test]
fn test_countersign_pass() {
    let votes = vec![
        CountersignVote { voter: "a".to_string(), approved: true, comment: None },
        CountersignVote { voter: "b".to_string(), approved: true, comment: None },
        CountersignVote { voter: "c".to_string(), approved: false, comment: None },
    ];

    let entry = RoutingNode {
        id: "node-0".to_string(),
        label: "会签".to_string(),
        node_type: NodeType::Countersign { pass_ratio: 0.6 },
        approver: "group".to_string(),
        transitions: vec![Transition {
            target_node_id: "node-1".to_string(),
            condition: None,
            priority: 0,
        }],
    };

    let next_node = RoutingNode {
        id: "node-1".to_string(),
        label: "End".to_string(),
        node_type: NodeType::Approval,
        approver: "end".to_string(),
        transitions: vec![],
    };

    let graph = build_graph("g3", "countersign-test", "node-0", vec![entry, next_node]).unwrap();
    let state = RoutingState::new("g3", "node-0");
    let context = ApprovalContext { request_id: "r3".to_string(), attributes: HashMap::new() };

    let mut vote_map = HashMap::new();
    vote_map.insert("node-0".to_string(), votes);

    let decision = resolve_next(&graph, &state, &context, Some(&vote_map)).unwrap();
    assert_eq!(decision.next_node_id, Some("node-1".to_string()));
}

#[test]
fn test_countersign_blocked() {
    let votes = vec![
        CountersignVote { voter: "a".to_string(), approved: true, comment: None },
        CountersignVote { voter: "b".to_string(), approved: false, comment: None },
        CountersignVote { voter: "c".to_string(), approved: false, comment: None },
    ];

    let entry = RoutingNode {
        id: "node-0".to_string(),
        label: "会签".to_string(),
        node_type: NodeType::Countersign { pass_ratio: 0.6 },
        approver: "group".to_string(),
        transitions: vec![Transition {
            target_node_id: "node-1".to_string(),
            condition: None,
            priority: 0,
        }],
    };

    let next_node = RoutingNode {
        id: "node-1".to_string(),
        label: "End".to_string(),
        node_type: NodeType::Approval,
        approver: "end".to_string(),
        transitions: vec![],
    };

    let graph = build_graph("g4", "countersign-block", "node-0", vec![entry, next_node]).unwrap();
    let state = RoutingState::new("g4", "node-0");
    let context = ApprovalContext { request_id: "r4".to_string(), attributes: HashMap::new() };

    let mut vote_map = HashMap::new();
    vote_map.insert("node-0".to_string(), votes);

    let decision = resolve_next(&graph, &state, &context, Some(&vote_map)).unwrap();
    assert!(decision.next_node_id.is_none());
    assert!(!decision.resolved);
}

#[test]
fn test_complex_expression_with_or() {
    let mut attrs = HashMap::new();
    attrs.insert("amount".to_string(), json!(5000));
    attrs.insert("level".to_string(), json!(5));

    let result = evaluate_expression("amount > 10000 OR level > 3", &attrs).unwrap();
    assert!(result);
}

#[test]
fn test_linear_chain_compat() {
    let nodes = build_linear_chain(vec![("n0", "alice"), ("n1", "bob"), ("n2", "carol")]);
    let graph = build_graph("g5", "linear", "n0", nodes).unwrap();
    let state = RoutingState::new("g5", "n0");
    let context = ApprovalContext { request_id: "r5".to_string(), attributes: HashMap::new() };

    let decision = resolve_next(&graph, &state, &context, None).unwrap();
    assert_eq!(decision.approver, "bob");
}

#[test]
fn test_advance_and_multi_step() {
    let nodes = build_linear_chain(vec![("n0", "alice"), ("n1", "bob"), ("n2", "carol")]);
    let graph = build_graph("g6", "multi-step", "n0", nodes).unwrap();
    let mut state = RoutingState::new("g6", "n0");
    let context = ApprovalContext { request_id: "r6".to_string(), attributes: HashMap::new() };

    advance(&graph, &mut state, "n1").unwrap();
    assert_eq!(state.current_node_id, "n1");

    let decision = resolve_next(&graph, &state, &context, None).unwrap();
    assert_eq!(decision.approver, "carol");

    advance(&graph, &mut state, "n2").unwrap();
    let decision = resolve_next(&graph, &state, &context, None).unwrap();
    assert!(decision.next_node_id.is_none());
}
