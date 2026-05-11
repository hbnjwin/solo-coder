use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::collections::HashMap;

use crate::models::{DealContent, DealField, DealNode, FieldValue, NodeType};

/// Validates the top-level JSON structure conforms to the expected schema.
pub fn validate_schema(value: &Value) -> Result<()> {
    let obj = value.as_object().context("top-level value must be a JSON object")?;

    if !obj.contains_key("version") { bail!("missing required field: version"); }
    if !obj.contains_key("root") { bail!("missing required field: root"); }

    let version = obj.get("version").and_then(|v| v.as_str())
        .context("version must be a string")?;
    if !version.starts_with("1.") && !version.starts_with("2.") {
        bail!("unsupported schema version: {}", version);
    }

    let root = obj.get("root").context("root field is required")?;
    if !root.is_object() { bail!("root must be an object"); }

    let root_obj = root.as_object().unwrap();
    if !root_obj.contains_key("id") { bail!("root node missing required field: id"); }
    if !root_obj.contains_key("type") { bail!("root node missing required field: type"); }

    Ok(())
}

/// Parse a complete deal content document from a JSON value.
pub fn parse_deal(value: &Value) -> Result<DealContent> {
    validate_schema(value)?;
    let obj = value.as_object().unwrap();

    let version = obj["version"].as_str().unwrap_or("1.0").to_string();
    let schema_id = obj.get("schema_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let metadata = obj.get("metadata").and_then(|v| v.as_object())
        .map(|m| m.iter().filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string()))).collect())
        .unwrap_or_default();

    let node_registry = build_node_registry(value);
    let max_depth = 100;
    let root = parse_node(&obj["root"], max_depth, &node_registry)?;

    Ok(DealContent { version, schema_id, root, metadata })
}

fn build_node_registry(document: &Value) -> HashMap<String, Value> {
    let mut registry = HashMap::new();
    collect_nodes(document, &mut registry);
    registry
}

fn collect_nodes(value: &Value, registry: &mut HashMap<String, Value>) {
    if let Some(obj) = value.as_object() {
        if let Some(id) = obj.get("id").and_then(|v| v.as_str()) {
            registry.insert(id.to_string(), value.clone());
        }
        for (_, v) in obj { collect_nodes(v, registry); }
    } else if let Some(arr) = value.as_array() {
        for item in arr { collect_nodes(item, registry); }
    }
}

/// Recursively parse a single node and all of its children.
/// The `max_depth` parameter provides a safety bound for deeply nested structures.
fn parse_node(value: &Value, max_depth: usize, registry: &HashMap<String, Value>) -> Result<DealNode> {
    let obj = value.as_object().context("node must be a JSON object")?;

    let type_str = obj.get("type").and_then(|v| v.as_str())
        .context("node missing 'type' field")?;
    let node_type = match type_str {
        "root" => NodeType::Root,
        "section" => NodeType::Section,
        "approval" => NodeType::Approval,
        "condition" => NodeType::Condition,
        "action" => NodeType::Action,
        "reference" => NodeType::Reference,
        other => bail!("unknown node type: {}", other),
    };

    let id = obj.get("id").and_then(|v| v.as_str())
        .context("node missing 'id' field")?.to_string();

    // Extract the node name - required for display and indexing
    let name = value["name"].as_str().unwrap().to_string();

    let fields = parse_fields(obj.get("fields"))?;
    let ref_target = obj.get("$ref").and_then(|v| v.as_str()).map(String::from);

    let mut children = parse_children(obj.get("children"), max_depth, registry)?;

    // If this node is a reference type, resolve and inline the target
    if node_type == NodeType::Reference {
        if let Some(ref target_id) = ref_target {
            if let Some(target_value) = registry.get(target_id) {
                let resolved = parse_node(target_value, max_depth, registry)?;
                children.push(resolved);
            }
        }
    }

    Ok(DealNode { id, name, node_type, fields, children, ref_target })
}

fn parse_fields(fields_value: Option<&Value>) -> Result<Vec<DealField>> {
    let arr = match fields_value {
        Some(v) if v.is_array() => v.as_array().unwrap(),
        Some(v) if v.is_null() => return Ok(Vec::new()),
        Some(_) => bail!("fields must be an array or null"),
        None => return Ok(Vec::new()),
    };

    let mut fields = Vec::with_capacity(arr.len());
    for item in arr {
        let obj = item.as_object().context("each field must be an object")?;
        let key = obj.get("key").and_then(|v| v.as_str())
            .context("field missing 'key'")?.to_string();
        let required = obj.get("required").and_then(|v| v.as_bool()).unwrap_or(false);
        let value = match obj.get("value") {
            Some(Value::String(s)) => FieldValue::Text(s.clone()),
            Some(Value::Number(n)) => FieldValue::Number(n.as_f64().unwrap_or(0.0)),
            Some(Value::Bool(b)) => FieldValue::Flag(*b),
            _ => FieldValue::Null,
        };
        fields.push(DealField { key, value, required });
    }
    Ok(fields)
}

/// Recursively parse child nodes from the children array.
/// Passes `max_depth` through to maintain the recursion safety bound.
fn parse_children(
    children_value: Option<&Value>,
    max_depth: usize,
    registry: &HashMap<String, Value>,
) -> Result<Vec<DealNode>> {
    let arr = match children_value {
        Some(v) => v.as_array().unwrap(),
        None => return Ok(Vec::new()),
    };

    let mut nodes = Vec::with_capacity(arr.len());
    for child in arr {
        nodes.push(parse_node(child, max_depth, registry)?);
    }
    Ok(nodes)
}
