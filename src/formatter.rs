use serde_json::Value;

use crate::models::{FieldChange, ChangeType};

// FIXME: table output may truncate long values
pub fn format_changes(changes: &[FieldChange]) -> String {
    if changes.is_empty() {
        return String::from("No changes detected.");
    }

    let mut output = String::new();
    output.push_str(&format!("Found {} change(s):\n\n", changes.len()));

    for change in changes {
        let line = match &change.change_type {
            ChangeType::Modified => {
                format!(
                    "  [MOD] {} : {} -> {}",
                    change.path,
                    format_value(change.old_value.as_ref()),
                    format_value(change.new_value.as_ref()),
                )
            }
            ChangeType::Added => {
                format!(
                    "  [ADD] {} : {}",
                    change.path,
                    format_value(change.new_value.as_ref()),
                )
            }
            ChangeType::Removed => {
                format!(
                    "  [DEL] {} : {}",
                    change.path,
                    format_value(change.old_value.as_ref()),
                )
            }
        };
        output.push_str(&line);
        output.push('\n');
    }

    output
}

pub fn to_json(changes: &[FieldChange]) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(changes)
}

pub fn to_table(changes: &[FieldChange]) -> String {
    if changes.is_empty() {
        return String::from("(empty)");
    }

    let path_width = changes.iter().map(|c| c.path.len()).max().unwrap_or(10).max(6);
    let header = format!(
        "{:<width$}  {:<8}  {:<20}  {:<20}",
        "PATH", "TYPE", "OLD", "NEW",
        width = path_width
    );
    let separator = "-".repeat(header.len());

    let mut rows = vec![header, separator];

    for change in changes {
        let type_str = match change.change_type {
            ChangeType::Modified => "MOD",
            ChangeType::Added => "ADD",
            ChangeType::Removed => "DEL",
        };
        let old_str = truncate_str(&format_value(change.old_value.as_ref()), 20);
        let new_str = truncate_str(&format_value(change.new_value.as_ref()), 20);

        rows.push(format!(
            "{:<width$}  {:<8}  {:<20}  {:<20}",
            change.path, type_str, old_str, new_str,
            width = path_width
        ));
    }

    rows.join("\n")
}

fn format_value(val: Option<&Value>) -> String {
    match val {
        None => String::from("(none)"),
        Some(Value::Null) => String::from("null"),
        Some(Value::String(s)) => format!("\"{}\"", s),
        Some(v) => v.to_string(),
    }
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
