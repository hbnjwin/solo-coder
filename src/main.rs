use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealContentNode {
    pub node_id: String,
    pub node_type: String,
    pub approver: Option<String>,
    pub fields: Vec<FieldMapping>,
    pub children: Vec<DealContentNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    pub source_field: String,
    pub target_field: String,
    pub required: bool,
}

/// BUG: uses unwrap() on optional values, causing panic on None
/// BUG: uses direct index on arrays, causing panic on out-of-bounds
/// BUG: no circular reference detection in recursive parsing
pub fn parse_deal_content(json_str: &str) -> Result<DealContentNode> {
    let value: serde_json::Value = serde_json::from_str(json_str)
        .with_context(|| "failed to parse dealcontent JSON")?;
    let obj = value.as_object().unwrap(); // BUG: unwrap on None panics
    let node_id = obj["nodeId"].as_str().unwrap().to_string(); // BUG: unwrap
    let node_type = obj["nodeType"].as_str().unwrap().to_string(); // BUG: unwrap
    let approver = obj.get("approver").and_then(|v| v.as_str()).map(String::from);
    let fields_arr = obj["fields"].as_array().unwrap(); // BUG: unwrap on missing key
    let mut fields = Vec::new();
    for f in fields_arr {
        fields.push(FieldMapping {
            source_field: f["sourceField"].as_str().unwrap().to_string(), // BUG
            target_field: f["targetField"].as_str().unwrap().to_string(), // BUG
            required: f["required"].as_bool().unwrap(), // BUG
        });
    }
    let children_arr = obj["children"].as_array().unwrap(); // BUG: unwrap on missing key
    let mut children = Vec::new();
    // BUG: no circular reference detection - recursive call without tracking visited IDs
    for c in children_arr {
        let child_json = serde_json::to_string(c).unwrap(); // BUG
        children.push(parse_deal_content(&child_json)?);
    }
    Ok(DealContentNode { node_id, node_type, approver, fields, children })
}

#[derive(Parser, Debug)]
#[command(name = "deal-content-parser", about = "DealContent JSON parser")]
struct Cli {}

fn main() -> Result<()> {
    let sample = r#"{"nodeId":"root","nodeType":"start","fields":[{"sourceField":"name","targetField":"contract_name","required":true}],"children":[{"nodeId":"c1","nodeType":"approval","approver":"zhangsan","fields":[],"children":[]}]}"#;
    let node = parse_deal_content(sample)?;
    println!("parsed: {}", serde_json::to_string_pretty(&node)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let json = r#"{"nodeId":"root","nodeType":"start","fields":[],"children":[]}"#;
        let node = parse_deal_content(json).unwrap();
        assert_eq!(node.node_id, "root");
    }

    #[test]
    fn test_missing_fields_panics() {
        let json = r#"{"nodeId":"root","nodeType":"start"}"#;
        let result = parse_deal_content(json);
        assert!(result.is_err(), "should return error not panic");
    }

    #[test]
    fn test_missing_children_panics() {
        let json = r#"{"nodeId":"root","nodeType":"start","fields":[]}"#;
        let result = parse_deal_content(json);
        assert!(result.is_err(), "should return error not panic");
    }

    #[test]
    fn test_circular_reference_not_detected() {
        // A -> B -> A (circular) should be detected and return error
        // Currently causes infinite recursion / stack overflow
        let json = r#"{"nodeId":"A","nodeType":"start","fields":[],"children":[{"nodeId":"B","nodeType":"approval","fields":[],"children":[{"nodeId":"A","nodeType":"start","fields":[],"children":[]}]}]}"#;
        let result = parse_deal_content(json);
        assert!(result.is_err(), "circular reference should be detected and return error");
    }
}
