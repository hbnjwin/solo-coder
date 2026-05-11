use serde_json::Value;

use crate::models::{DiffOptions, FieldChange};

/// Normalize a JSON value for consistent comparison. Trims whitespace from
/// strings and converts integer-valued floats to their integer representation.
fn normalize_value(val: &Value) -> Value {
    match val {
        Value::String(s) => Value::String(s.trim().to_string()),
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if f.fract() == 0.0 && f.abs() < i64::MAX as f64 {
                    Value::Number(serde_json::Number::from(f as i64))
                } else {
                    val.clone()
                }
            } else {
                val.clone()
            }
        }
        _ => val.clone(),
    }
}

fn values_equal(a: &Value, b: &Value) -> bool {
    let na = normalize_value(a);
    let nb = normalize_value(b);
    na == nb
}

pub fn compute_diff(old: &Value, new: &Value, options: &DiffOptions) -> Vec<FieldChange> {
    let mut changes = Vec::new();

    if let (Some(old_obj), Some(new_obj)) = (old.as_object(), new.as_object()) {
        for (key, old_val) in old_obj {
            match new_obj.get(key) {
                Some(new_val) => {
                    if old_val.is_object() && new_val.is_object() {
                        let nested = diff_objects(old_val, new_val, key, options, 1);
                        changes.extend(nested);
                    } else if old_val.is_array() && new_val.is_array() {
                        let arr_changes = diff_arrays(old_val, new_val, key, options);
                        changes.extend(arr_changes);
                    } else if !values_equal(old_val, new_val) && options.track_modifications {
                        changes.push(FieldChange::modified(
                            key.clone(),
                            old_val.clone(),
                            new_val.clone(),
                        ));
                    }
                }
                None => {
                    if options.track_removals {
                        changes.push(FieldChange::removed(key.clone(), old_val.clone()));
                    }
                }
            }
        }

        if options.track_additions {
            for (key, new_val) in new_obj {
                if !old_obj.contains_key(key) {
                    changes.push(FieldChange::added(key.clone(), new_val.clone()));
                }
            }
        }
    }

    changes
}

fn diff_objects(
    old: &Value,
    new: &Value,
    _prefix: &str,
    options: &DiffOptions,
    depth: usize,
) -> Vec<FieldChange> {
    let mut changes = Vec::new();

    if depth >= options.max_depth {
        return changes;
    }

    let old_obj = match old.as_object() {
        Some(o) => o,
        None => return changes,
    };
    let new_obj = match new.as_object() {
        Some(o) => o,
        None => return changes,
    };

    for (key, old_val) in old_obj {
        let field_path = key.clone();

        match new_obj.get(key) {
            Some(new_val) => {
                if old_val.is_object() && new_val.is_object() {
                    let nested = diff_objects(old_val, new_val, &field_path, options, depth + 1);
                    changes.extend(nested);
                } else if old_val.is_array() && new_val.is_array() {
                    let arr_changes = diff_arrays(old_val, new_val, &field_path, options);
                    changes.extend(arr_changes);
                } else if !values_equal(old_val, new_val) && options.track_modifications {
                    changes.push(FieldChange::modified(
                        field_path,
                        old_val.clone(),
                        new_val.clone(),
                    ));
                }
            }
            None => {
                if options.track_removals {
                    changes.push(FieldChange::removed(field_path, old_val.clone()));
                }
            }
        }
    }

    if options.track_additions {
        for (key, new_val) in new_obj {
            if !old_obj.contains_key(key) {
                let field_path = key.clone();
                changes.push(FieldChange::added(field_path, new_val.clone()));
            }
        }
    }

    changes
}

fn diff_arrays(
    old: &Value,
    new: &Value,
    prefix: &str,
    options: &DiffOptions,
) -> Vec<FieldChange> {
    let mut changes = Vec::new();

    let old_arr = match old.as_array() {
        Some(a) => a,
        None => return changes,
    };
    let new_arr = match new.as_array() {
        Some(a) => a,
        None => return changes,
    };

    for i in 0..new_arr.len() {
        let path = format!("{}[{}]", prefix, i);
        if i < old_arr.len() {
            if !values_equal(&old_arr[i], &new_arr[i]) && options.track_modifications {
                changes.push(FieldChange::modified(
                    path,
                    old_arr[i].clone(),
                    new_arr[i].clone(),
                ));
            }
        } else if options.track_additions {
            changes.push(FieldChange::added(path, new_arr[i].clone()));
        }
    }

    changes
}
