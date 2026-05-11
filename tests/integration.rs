use std::path::PathBuf;
use config_validator::parser::parse_config;
use config_validator::validator::validate_config;
use config_validator::models::ValidationError;

fn fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);
    path
}

#[test]
fn test_config_passes() {
    let config = parse_config(&fixture_path("valid.json")).unwrap();
    let errors = validate_config(&config);
    assert!(errors.is_empty(), "expected no errors, got: {:?}", errors);
}

#[test]
fn test_transition_target_ref() {
    let config = parse_config(&fixture_path("invalid_orphan.json")).unwrap();
    let errors = validate_config(&config);
    let has_ref_error = errors.iter().any(|e| matches!(
        e,
        ValidationError::InvalidTransitionRef { field, .. } if field == "to"
    ));
    assert!(has_ref_error, "expected invalid transition target error, got: {:?}", errors);
}

#[test]
fn test_node_reachability() {
    let config = parse_config(&fixture_path("invalid_cycle.json")).unwrap();
    let errors = validate_config(&config);
    let unreachable: Vec<_> = errors.iter().filter(|e| matches!(e, ValidationError::UnreachableNode(_))).collect();
    assert!(unreachable.is_empty(), "all nodes should be reachable via transitions, got: {:?}", unreachable);
}

#[test]
fn test_duplicate_ids() {
    let json = r#"{
        "name": "dup-test",
        "start_node": "alpha",
        "nodes": [
            { "id": "alpha", "node_type": "start" },
            { "id": "alpha", "node_type": "action" },
            { "id": "beta", "node_type": "end" }
        ],
        "transitions": [
            { "from": "alpha", "to": "beta" }
        ]
    }"#;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dup.json");
    std::fs::write(&path, json).unwrap();

    let config = parse_config(&path).unwrap();
    let errors = validate_config(&config);
    let has_dup = errors.iter().any(|e| matches!(e, ValidationError::DuplicateNodeId(id) if id == "alpha"));
    assert!(has_dup, "expected duplicate id error, got: {:?}", errors);
}

#[test]
fn test_empty_nodes() {
    let json = r#"{
        "name": "empty-test",
        "start_node": "x",
        "nodes": [],
        "transitions": []
    }"#;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("empty.json");
    std::fs::write(&path, json).unwrap();

    let config = parse_config(&path).unwrap();
    let errors = validate_config(&config);
    assert!(errors.contains(&ValidationError::EmptyNodes), "expected empty nodes error, got: {:?}", errors);
}
