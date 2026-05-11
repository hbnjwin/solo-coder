use serde_json::json;

use audit_trail::differ::compute_diff;
use audit_trail::models::{ChangeType, DiffOptions};

#[test]
fn test_flat_diff() {
    let old = json!({"status": "pending", "amount": 1000});
    let new = json!({"status": "approved", "amount": 1000});

    let options = DiffOptions::default();
    let changes = compute_diff(&old, &new, &options);

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].path, "status");
    assert_eq!(changes[0].change_type, ChangeType::Modified);
}

#[test]
fn test_nested_object_diff() {
    let old = json!({
        "address": {
            "city": "Shanghai",
            "zip": "200000"
        }
    });
    let new = json!({
        "address": {
            "city": "Beijing",
            "zip": "200000"
        }
    });

    let options = DiffOptions::default();
    let changes = compute_diff(&old, &new, &options);

    assert_eq!(changes.len(), 1, "expected exactly one change for nested field");
    assert_eq!(
        changes[0].path, "address.city",
        "nested field path should use dot notation"
    );
}

#[test]
fn test_array_removal() {
    let old = json!({"items": ["alpha", "beta", "gamma"]});
    let new = json!({"items": ["alpha", "beta"]});

    let options = DiffOptions::default();
    let changes = compute_diff(&old, &new, &options);

    assert!(
        changes.iter().any(|c| c.change_type == ChangeType::Removed),
        "should detect removed array element"
    );
}

#[test]
fn test_no_changes() {
    let doc = json!({"name": "test", "value": 42});

    let options = DiffOptions::default();
    let changes = compute_diff(&doc, &doc, &options);

    assert!(changes.is_empty(), "identical documents should produce no changes");
}

#[test]
fn test_added_field() {
    let old = json!({"name": "report"});
    let new = json!({"name": "report", "version": 2});

    let options = DiffOptions::default();
    let changes = compute_diff(&old, &new, &options);

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].path, "version");
    assert_eq!(changes[0].change_type, ChangeType::Added);
}

#[test]
fn test_type_change() {
    let old = json!({"count": "five"});
    let new = json!({"count": 5});

    let options = DiffOptions::default();
    let changes = compute_diff(&old, &new, &options);

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].path, "count");
    assert_eq!(changes[0].change_type, ChangeType::Modified);
}
