use deal_content_parser::parser::parse_deal;
use deal_content_parser::formatter::{format_deal, summarize, FormatOptions};
use serde_json::json;

#[test]
fn test_parse_valid_deal() {
    let input = json!({
        "version": "1.0",
        "schema_id": "deal-v1",
        "root": {
            "id": "root-001",
            "type": "root",
            "name": "Master Agreement",
            "fields": [
                {"key": "contract_id", "value": "C-2024-001", "required": true},
                {"key": "status", "value": "draft", "required": false}
            ],
            "children": [
                {
                    "id": "section-001",
                    "type": "section",
                    "name": "Terms",
                    "fields": [],
                    "children": []
                }
            ]
        }
    });

    let result = parse_deal(&input);
    assert!(result.is_ok(), "valid deal should parse successfully");
    let content = result.unwrap();
    assert_eq!(content.version, "1.0");
    assert_eq!(content.root.id, "root-001");
    assert_eq!(content.root.children.len(), 1);
}

#[test]
fn test_missing_required_fields() {
    // Node is missing the "name" field entirely
    let input = json!({
        "version": "1.0",
        "root": {
            "id": "root-001",
            "type": "root",
            "fields": [],
            "children": []
        }
    });

    let result = parse_deal(&input);
    assert!(result.is_err(), "missing 'name' field should return an error");
}

#[test]
fn test_missing_children_array() {
    // Node has children set to null instead of an array
    let input = json!({
        "version": "1.0",
        "root": {
            "id": "root-001",
            "type": "root",
            "name": "Test Deal",
            "fields": [],
            "children": [
                {
                    "id": "child-001",
                    "type": "section",
                    "name": "Section A",
                    "fields": [],
                    "children": null
                }
            ]
        }
    });

    let result = parse_deal(&input);
    assert!(result.is_err(), "null children should return an error, not panic");
}

#[test]
fn test_circular_reference() {
    // A reference node points back to an ancestor node ID.
    // The parser should detect this cycle and return an error
    // rather than infinitely resolving the reference.
    let input = json!({
        "version": "1.0",
        "root": {
            "id": "node-A",
            "type": "root",
            "name": "Root Node",
            "fields": [],
            "children": [
                {
                    "id": "node-B",
                    "type": "section",
                    "name": "Section B",
                    "fields": [],
                    "children": [
                        {
                            "id": "ref-back",
                            "type": "reference",
                            "name": "Back Reference",
                            "$ref": "node-A",
                            "fields": [],
                            "children": []
                        }
                    ]
                }
            ]
        }
    });

    let result = parse_deal(&input);
    assert!(result.is_err(), "circular reference should be detected and return an error");
}

#[test]
fn test_deep_nesting() {
    // Build a 50-level deep valid structure
    fn build_nested(depth: usize) -> serde_json::Value {
        if depth == 0 {
            return json!({
                "id": format!("leaf-{}", depth),
                "type": "action",
                "name": format!("Leaf Node {}", depth),
                "fields": [],
                "children": []
            });
        }
        json!({
            "id": format!("node-{}", depth),
            "type": "section",
            "name": format!("Level {}", depth),
            "fields": [],
            "children": [build_nested(depth - 1)]
        })
    }

    let input = json!({
        "version": "1.0",
        "root": {
            "id": "deep-root",
            "type": "root",
            "name": "Deep Structure",
            "fields": [],
            "children": [build_nested(50)]
        }
    });

    let result = parse_deal(&input);
    assert!(result.is_ok(), "deep but valid nesting should parse successfully");
}

#[test]
fn test_format_output() {
    let input = json!({
        "version": "1.0",
        "schema_id": "test-schema",
        "root": {
            "id": "fmt-root",
            "type": "root",
            "name": "Format Test",
            "fields": [
                {"key": "title", "value": "Hello", "required": true}
            ],
            "children": [
                {
                    "id": "fmt-child",
                    "type": "approval",
                    "name": "Review Step",
                    "fields": [],
                    "children": []
                }
            ]
        }
    });

    let content = parse_deal(&input).expect("should parse valid input");
    let opts = FormatOptions::default();
    let output = format_deal(&content, &opts);

    assert!(output.contains("Format Test"), "output should contain root node name");
    assert!(output.contains("Review Step"), "output should contain child node name");
    assert!(output.contains("title"), "output should contain field key");

    let summary = summarize(&content);
    assert!(summary.contains("2 nodes"), "summary should report correct node count");
}
