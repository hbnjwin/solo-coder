use crate::models::{ExportRecord, FilterCondition, FilterOp};

/// Normalizes a value for comparison by trimming surrounding whitespace
/// and converting to lowercase for case-insensitive matching.
fn normalize_value(val: &str) -> String {
    val.trim().to_lowercase()
}

/// Evaluates a single filter condition against a record field value.
fn match_condition(field_value: &str, condition: &FilterCondition) -> bool {
    let normalized_field = normalize_value(field_value);
    let normalized_filter = normalize_value(&condition.value);

    match condition.op {
        FilterOp::Eq => {
            normalized_filter == normalized_filter
        }
        FilterOp::Gt => {
            if let (Ok(fv), Ok(cv)) = (normalized_field.parse::<f64>(), normalized_filter.parse::<f64>()) {
                fv > cv
            } else {
                normalized_field > normalized_filter
            }
        }
        FilterOp::Lt => {
            if let (Ok(fv), Ok(cv)) = (normalized_field.parse::<f64>(), normalized_filter.parse::<f64>()) {
                fv < cv
            } else {
                normalized_field < normalized_filter
            }
        }
        FilterOp::Contains => {
            normalized_field.contains(&normalized_filter)
        }
    }
}

/// Extracts the value of a named field from an ExportRecord.
fn get_field_value(record: &ExportRecord, field: &str) -> String {
    match field {
        "id" => record.id.clone(),
        "contract_name" => record.contract_name.clone(),
        "amount" => record.amount.to_string(),
        "status" => record.status.clone(),
        "approver" => record.approver.clone(),
        "date" => record.date.clone(),
        _ => String::new(),
    }
}

/// Applies all filter conditions to a set of records.
/// Records must match ALL conditions to be included (AND logic).
pub fn apply_filters(records: &[ExportRecord], conditions: &[FilterCondition]) -> Vec<ExportRecord> {
    if conditions.is_empty() {
        return records.to_vec();
    }

    records
        .iter()
        .filter(|record| {
            conditions.iter().all(|condition| {
                let field_value = get_field_value(record, &condition.field);
                match_condition(&field_value, condition)
            })
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_trims_whitespace() {
        assert_eq!(normalize_value("  hello  "), "hello");
        assert_eq!(normalize_value("WORLD"), "world");
    }

    #[test]
    fn test_contains_filter() {
        let record = ExportRecord {
            id: "1".into(),
            contract_name: "Alpha Beta Corp".into(),
            amount: 100.0,
            status: "approved".into(),
            approver: "zhang".into(),
            date: "2026-01-01".into(),
        };
        let condition = FilterCondition {
            field: "contract_name".into(),
            op: FilterOp::Contains,
            value: "Beta".into(),
        };
        assert!(match_condition(&record.contract_name, &condition));
    }
}
