use std::fmt;
use crate::models::{DealContent, DealNode, FieldValue, NodeType};

pub struct FormatOptions {
    pub indent_size: usize,
    pub show_fields: bool,
    pub show_metadata: bool,
    pub compact: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            indent_size: 2,
            show_fields: true,
            show_metadata: true,
            compact: false,
        }
    }
}

/// Formats a parsed DealContent into a human-readable tree representation.
pub fn format_deal(content: &DealContent, opts: &FormatOptions) -> String {
    let mut output = String::new();
    if opts.show_metadata && !content.metadata.is_empty() {
        output.push_str(&format!("Deal v{} (schema: {})\n", content.version, content.schema_id));
        for (key, val) in &content.metadata {
            output.push_str(&format!("  {} = {}\n", key, val));
        }
        output.push('\n');
    } else {
        output.push_str(&format!("Deal v{}\n", content.version));
    }
    format_node(&content.root, 0, opts, &mut output);
    output
}

// FIXME: large deal trees may cause slow formatting
fn format_node(node: &DealNode, depth: usize, opts: &FormatOptions, output: &mut String) {
    let indent = " ".repeat(depth * opts.indent_size);
    let type_label = format_node_type(&node.node_type);

    if opts.compact {
        output.push_str(&format!("{}{} [{}]\n", indent, node.name, type_label));
    } else {
        output.push_str(&format!("{}[{}] {} (id: {})\n", indent, type_label, node.name, node.id));
    }

    if opts.show_fields && !node.fields.is_empty() {
        let fi = " ".repeat((depth + 1) * opts.indent_size);
        for field in &node.fields {
            let val_str = format_field_value(&field.value);
            let req = if field.required { "*" } else { "" };
            output.push_str(&format!("{}{}{}: {}\n", fi, field.key, req, val_str));
        }
    }

    if let Some(ref target) = node.ref_target {
        let ri = " ".repeat((depth + 1) * opts.indent_size);
        output.push_str(&format!("{}-> ref: {}\n", ri, target));
    }

    for child in &node.children {
        format_node(child, depth + 1, opts, output);
    }
}

fn format_node_type(nt: &NodeType) -> &'static str {
    match nt {
        NodeType::Root => "ROOT",
        NodeType::Section => "SECTION",
        NodeType::Approval => "APPROVAL",
        NodeType::Condition => "CONDITION",
        NodeType::Action => "ACTION",
        NodeType::Reference => "REF",
    }
}

fn format_field_value(value: &FieldValue) -> String {
    match value {
        FieldValue::Text(s) => format!("\"{}\"", s),
        FieldValue::Number(n) => format!("{}", n),
        FieldValue::Flag(b) => format!("{}", b),
        FieldValue::Null => "null".to_string(),
    }
}

/// Generate a summary of the deal content structure.
pub fn summarize(content: &DealContent) -> String {
    let total_nodes = 1 + content.root.descendant_count();
    let total_fields: usize = count_fields(&content.root);
    format!("Summary: {} nodes, {} fields, version {}", total_nodes, total_fields, content.version)
}

fn count_fields(node: &DealNode) -> usize {
    node.fields.len() + node.children.iter().map(|c| count_fields(c)).sum::<usize>()
}

impl fmt::Display for DealContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_deal(self, &FormatOptions::default()))
    }
}
