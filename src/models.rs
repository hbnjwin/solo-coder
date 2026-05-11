use serde::{Deserialize, Serialize};

/// Represents a single approval record in the export pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRecord {
    pub id: String,
    pub contract_name: String,
    pub amount: f64,
    pub status: String,
    pub approver: String,
    pub date: String,
}

/// Supported filter operations for record matching.
#[derive(Debug, Clone, PartialEq)]
pub enum FilterOp {
    Eq,
    Gt,
    Lt,
    Contains,
}

/// A single filter condition to apply against records.
#[derive(Debug, Clone)]
pub struct FilterCondition {
    pub field: String,
    pub op: FilterOp,
    pub value: String,
}

/// Specifies the sort direction for record ordering.
#[derive(Debug, Clone, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Sort configuration for the export pipeline.
#[derive(Debug, Clone)]
pub struct SortConfig {
    pub field: String,
    pub direction: SortDirection,
}

/// Output format selection.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Csv,
    Json,
}

impl ExportFormat {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(ExportFormat::Csv),
            "json" => Ok(ExportFormat::Json),
            other => anyhow::bail!("unsupported export format: {}", other),
        }
    }
}

impl SortConfig {
    pub fn parse(spec: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = spec.splitn(2, ':').collect();
        if parts.len() != 2 {
            anyhow::bail!("sort spec must be field:direction (e.g. date:desc)");
        }
        let direction = match parts[1] {
            "asc" | "ascending" => SortDirection::Ascending,
            "desc" | "descending" => SortDirection::Descending,
            other => anyhow::bail!("unknown sort direction: {}", other),
        };
        Ok(SortConfig {
            field: parts[0].to_string(),
            direction,
        })
    }
}
