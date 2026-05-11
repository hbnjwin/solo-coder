use std::collections::HashMap;

use approval_routing::graph::{build_conditional_node, build_graph};
use approval_routing::models::{
    ApprovalContext, ConditionOperator, RoutingGraph, RoutingNode, RoutingState, Transition,
};
use approval_routing::router::{advance, resolve_next};

#[test]
fn test_linear_routing() {
    let nodes = vec![
        RoutingNode {
            id: "node-0".to_string(),
            label: "Step 1".to_string(),
            approver: "manager".to_string(),
            transitions: vec![Transition {
                target_node_id: "node-1".to_string(),
                condition: None,
                priority: 0,
            }],
        },
        RoutingNode {
            id: "node-1".to_string(),
            label: "Step 2".to_string(),
            approver: "director".to_string(),
            transitions: vec![Transition {
                target_node_id: "node-2".to_string(),
                condition: None,
                priority: 0,
            }],
        },
        RoutingNode {
            id: "node-2".to_string(),
            label: "Step 3".to_string(),
            approver: "vp".to_string(),
            transitions: vec![],
        },
    ];

    let graph = build_graph("g1", "linear", "node-0", nodes).unwrap();
    let state = RoutingState::new("g1", "node-0");
    let context = ApprovalContext {
        request_id: "req-1".to_string(),
        attributes: HashMap::new(),
    };

    let decision = resolve_next(&graph, &state, &context).unwrap();
    assert_eq!(decision.approver, "director");
    assert_eq!(decision.next_node_id, Some("node-1".to_string()));
    assert!(decision.resolved);
}

#[test]
fn test_conditional_branch() {
    // Build a graph where node-0 branches:
    //   amount > 1000000 -> node-ceo (approver: "ceo")
    //   otherwise -> node-mgr (approver: "manager")
    let entry = build_conditional_node(
        "node-0",
        "router",
        "amount",
        ConditionOperator::GreaterThan,
        serde_json::json!(1000000),
        "node-ceo",
        "node-mgr",
    );

    let ceo_node = RoutingNode {
        id: "node-ceo".to_string(),
        label: "CEO Approval".to_string(),
        approver: "ceo".to_string(),
        transitions: vec![],
    };

    let mgr_node = RoutingNode {
        id: "node-mgr".to_string(),
        label: "Manager Approval".to_string(),
        approver: "manager".to_string(),
        transitions: vec![],
    };

    let graph = build_graph("g2", "conditional", "node-0", vec![entry, ceo_node, mgr_node]).unwrap();
    let state = RoutingState::new("g2", "node-0");

    // Context: amount is 2,000,000 which is > 1,000,000
    let mut attrs = HashMap::new();
    attrs.insert("amount".to_string(), serde_json::json!(2000000));
    let context = ApprovalContext {
        request_id: "req-2".to_string(),
        attributes: attrs,
    };

    let decision = resolve_next(&graph, &state, &context).unwrap();
    assert_eq!(
        decision.approver, "ceo",
        "amount 2M exceeds threshold 1M, should route to CEO"
    );
    assert_eq!(decision.next_node_id, Some("node-ceo".to_string()));
}

#[test]
fn test_advance_invalid_node() {
    // Construct a graph directly where node-0 has a transition to a node
    // that is listed in transitions but does not exist as an actual graph node.
    let mut nodes = HashMap::new();
    nodes.insert(
        "node-0".to_string(),
        RoutingNode {
            id: "node-0".to_string(),
            label: "Start".to_string(),
            approver: "alice".to_string(),
            transitions: vec![Transition {
                target_node_id: "node-phantom".to_string(),
                condition: None,
                priority: 0,
            }],
        },
    );

    let graph = RoutingGraph {
        id: "g3".to_string(),
        name: "dangling".to_string(),
        nodes,
        entry_node_id: "node-0".to_string(),
    };

    let mut state = RoutingState::new("g3", "node-0");

    // Advance to node-phantom which is in the transition list but not in the graph
    let result = advance(&graph, &mut state, "node-phantom");
    assert!(
        result.is_err(),
        "advancing to a non-existent node should return an error"
    );
}

#[test]
fn test_graph_validation() {
    let nodes = vec![
        RoutingNode {
            id: "start".to_string(),
            label: "Entry".to_string(),
            approver: "sys".to_string(),
            transitions: vec![
                Transition {
                    target_node_id: "review".to_string(),
                    condition: None,
                    priority: 0,
                },
            ],
        },
        RoutingNode {
            id: "review".to_string(),
            label: "Review".to_string(),
            approver: "reviewer".to_string(),
            transitions: vec![
                Transition {
                    target_node_id: "approve".to_string(),
                    condition: None,
                    priority: 0,
                },
            ],
        },
        RoutingNode {
            id: "approve".to_string(),
            label: "Final".to_string(),
            approver: "approver".to_string(),
            transitions: vec![],
        },
    ];

    let result = build_graph("g4", "validated", "start", nodes);
    assert!(result.is_ok(), "well-formed graph should pass validation");
}

#[test]
fn test_multi_level_routing() {
    let nodes = vec![
        RoutingNode {
            id: "l1".to_string(),
            label: "Level 1".to_string(),
            approver: "team-lead".to_string(),
            transitions: vec![Transition {
                target_node_id: "l2".to_string(),
                condition: None,
                priority: 0,
            }],
        },
        RoutingNode {
            id: "l2".to_string(),
            label: "Level 2".to_string(),
            approver: "department-head".to_string(),
            transitions: vec![Transition {
                target_node_id: "l3".to_string(),
                condition: None,
                priority: 0,
            }],
        },
        RoutingNode {
            id: "l3".to_string(),
            label: "Level 3".to_string(),
            approver: "cfo".to_string(),
            transitions: vec![],
        },
    ];

    let graph = build_graph("g5", "multi-level", "l1", nodes).unwrap();
    let mut state = RoutingState::new("g5", "l1");
    let context = ApprovalContext {
        request_id: "req-5".to_string(),
        attributes: HashMap::new(),
    };

    // Resolve from l1 -> should point to l2
    let decision = resolve_next(&graph, &state, &context).unwrap();
    assert_eq!(decision.approver, "department-head");

    // Advance to l2
    advance(&graph, &mut state, "l2").unwrap();
    assert_eq!(state.current_node_id, "l2");

    // Resolve from l2 -> should point to l3
    let decision = resolve_next(&graph, &state, &context).unwrap();
    assert_eq!(decision.approver, "cfo");

    // Advance to l3
    advance(&graph, &mut state, "l3").unwrap();
    assert_eq!(state.current_node_id, "l3");

    // l3 is terminal
    let decision = resolve_next(&graph, &state, &context).unwrap();
    assert!(decision.next_node_id.is_none());
    assert_eq!(decision.approver, "cfo");
}
