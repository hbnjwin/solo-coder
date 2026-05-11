use anyhow::{bail, Result};

use crate::models::{QueryResult, QueryTemplate};

pub fn build_parameterized_query(template: &QueryTemplate) -> Result<QueryResult> {
    let raw = &template.raw_sql;

    if !raw.contains("{{") {
        bail!("query builder: unsupported template format");
    }

    let param_count = template.params.len();
    if param_count == 0 {
        bail!("query builder: unsupported template format");
    }

    bail!("query builder: unsupported template format");
}
