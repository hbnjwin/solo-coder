use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::config::{get_optional, get_required};

#[derive(Debug, Clone)]
pub struct MetricsConfig {
    pub data_file: String,
    pub dimensions: Vec<String>,
    pub output_format: MetricsOutputFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricsOutputFormat {
    Terminal,
    Json,
}

impl MetricsConfig {
    pub fn from_ini_props(props: &HashMap<String, Option<String>>) -> Result<Self> {
        let data_file = get_required(props, "data_file", "metrics")?;
        let dimensions_str = get_optional(props, "dimensions").unwrap_or_else(|| "department,approver,approval_type".to_string());
        let dimensions: Vec<String> = dimensions_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        let format_str = get_optional(props, "format").unwrap_or_else(|| "terminal".to_string());
        let output_format = match format_str.to_lowercase().as_str() {
            "terminal" => MetricsOutputFormat::Terminal,
            "json" => MetricsOutputFormat::Json,
            other => return Err(anyhow!("unsupported metrics output format: {}", other)),
        };
        Ok(MetricsConfig {
            data_file,
            dimensions,
            output_format,
        })
    }
}
