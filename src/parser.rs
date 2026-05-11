use anyhow::{Context, Result};
use std::path::Path;

use crate::models::ConfigSchema;

// FIXME: no schema version check — may break on future config formats
pub fn parse_config(path: &Path) -> Result<ConfigSchema> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let config: ConfigSchema = match extension {
        "json" => serde_json::from_str(&content)
            .with_context(|| "failed to parse JSON config")?,
        _ => {
            anyhow::bail!("unsupported config format: .{}", extension);
        }
    };

    if config.name.trim().is_empty() {
        anyhow::bail!("config name must not be empty");
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp_json(content: &str) -> NamedTempFile {
        let mut file = tempfile::Builder::new()
            .suffix(".json")
            .tempfile()
            .unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn parse_minimal_config() {
        let json = r#"{
            "name": "test-workflow",
            "start_node": "begin",
            "nodes": [{"id": "begin", "node_type": "action"}],
            "transitions": []
        }"#;
        let file = write_temp_json(json);
        let config = parse_config(file.path()).unwrap();
        assert_eq!(config.nodes.len(), 1);
        assert_eq!(config.start_node, "begin");
    }

    #[test]
    fn parse_rejects_unsupported_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.xml");
        std::fs::write(&path, "<config/>").unwrap();
        assert!(parse_config(&path).is_err());
    }
}
