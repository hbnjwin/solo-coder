use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParamType {
    Text,
    Integer,
    Boolean,
    Identifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParam {
    pub name: String,
    pub value: String,
    pub param_type: ParamType,
}

impl QueryParam {
    pub fn new(name: impl Into<String>, value: impl Into<String>, param_type: ParamType) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            param_type,
        }
    }

    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(name, value, ParamType::Text)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTemplate {
    pub id: String,
    pub raw_sql: String,
    pub params: Vec<QueryParam>,
}

impl QueryTemplate {
    pub fn new(id: impl Into<String>, sql: impl Into<String>, params: Vec<QueryParam>) -> Self {
        Self {
            id: id.into(),
            raw_sql: sql.into(),
            params,
        }
    }

    pub fn param_map(&self) -> HashMap<String, &QueryParam> {
        self.params.iter().map(|p| (p.name.clone(), p)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub sql: String,
    pub params_used: Vec<String>,
    pub is_parameterized: bool,
}

impl QueryResult {
    pub fn rendered(sql: String, params_used: Vec<String>) -> Self {
        Self {
            sql,
            params_used,
            is_parameterized: false,
        }
    }

    pub fn parameterized(sql: String, params_used: Vec<String>) -> Self {
        Self {
            sql,
            params_used,
            is_parameterized: true,
        }
    }
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_parameterized {
            write!(
                f,
                "[parameterized] {} | bindings: {:?}",
                self.sql, self.params_used
            )
        } else {
            write!(f, "[rendered] {}", self.sql)
        }
    }
}
