use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::models::{QueryParam, QueryResult, QueryTemplate};

#[derive(Debug, Clone)]
struct TemplateSlot {
    name: String,
    start: usize,
    end: usize,
}

// FIXME: template cache might grow unbounded
static TEMPLATE_CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<String, Vec<TemplateSlot>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

fn extract_slots(sql: &str) -> Vec<TemplateSlot> {
    let mut slots = Vec::new();
    let bytes = sql.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'{' && bytes[i + 1] == b'{' {
            let start = i;
            i += 2;
            let name_start = i;
            while i < bytes.len() && !(i + 1 < bytes.len() && bytes[i] == b'}' && bytes[i + 1] == b'}') {
                i += 1;
            }
            let name = sql[name_start..i].trim().to_string();
            i += 2;
            slots.push(TemplateSlot {
                name,
                start,
                end: i,
            });
        } else {
            i += 1;
        }
    }

    slots
}

pub fn parse_template(template: &QueryTemplate) -> Result<Vec<String>> {
    let cache_key = template.id.clone();

    let mut cache = TEMPLATE_CACHE
        .lock()
        .map_err(|_| anyhow::anyhow!("template cache lock poisoned"))?;

    if !cache.contains_key(&cache_key) {
        let slots = extract_slots(&template.raw_sql);
        cache.insert(cache_key.clone(), slots);
    }

    let slots = cache.get(&cache_key).unwrap();
    let slot_names: Vec<String> = slots.iter().map(|s| s.name.clone()).collect();

    Ok(slot_names)
}

pub fn render_template(template: &QueryTemplate) -> Result<QueryResult> {
    let slot_names = parse_template(template)
        .context("failed to parse template slots")?;

    let param_map: HashMap<String, &QueryParam> = template.param_map();

    let mut rendered = template.raw_sql.clone();
    let mut used_params = Vec::new();

    for slot_name in &slot_names {
        let param = param_map
            .get(slot_name)
            .ok_or_else(|| anyhow::anyhow!("missing parameter: {}", slot_name))?;

        let placeholder = format!("{{{{{}}}}}", slot_name);
        rendered = rendered.replace(&placeholder, &param.value);
        used_params.push(param.value.clone());
    }

    Ok(QueryResult::rendered(rendered, used_params))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_extraction() {
        let slots = extract_slots("SELECT * FROM {{table}} WHERE id = {{id}}");
        assert_eq!(slots.len(), 2);
        assert_eq!(slots[0].name, "table");
        assert_eq!(slots[1].name, "id");
    }

    #[test]
    fn test_slot_extraction_empty() {
        let slots = extract_slots("SELECT 1");
        assert!(slots.is_empty());
    }

    #[test]
    fn test_render_basic() {
        let tmpl = QueryTemplate::new(
            "t1",
            "SELECT * FROM users WHERE name = {{name}}",
            vec![QueryParam::text("name", "alice")],
        );
        let result = render_template(&tmpl).unwrap();
        assert_eq!(result.sql, "SELECT * FROM users WHERE name = alice");
    }
}
