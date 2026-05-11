#!/usr/bin/env python3
"""Update Rust source files to add hidden bugs and test cases matching new prompts"""
import os

BASE = r"d:\work\github\solo-coder\projects\trae-solo"

def w(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

# q7: approval-routing - add advance() node validation bug + test
w(f"{BASE}/approval-routing/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalNode {
    pub id: String,
    pub name: String,
    pub approver: String,
    pub next_node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRoute {
    pub id: String,
    pub name: String,
    pub nodes: Vec<ApprovalNode>,
    pub current_node_idx: usize,
}

impl ApprovalRoute {
    pub fn new(id: &str, name: &str, nodes: Vec<ApprovalNode>) -> Self {
        Self { id: id.into(), name: name.into(), nodes, current_node_idx: 0 }
    }
    pub fn current_node(&self) -> Option<&ApprovalNode> {
        self.nodes.get(self.current_node_idx)
    }
    /// BUG: advance() doesn't validate that next_node_id points to an existing node
    pub fn advance(&mut self) -> Result<&ApprovalNode> {
        if self.current_node_idx + 1 >= self.nodes.len() {
            anyhow::bail!("already at last node");
        }
        self.current_node_idx += 1;
        Ok(&self.nodes[self.current_node_idx])
    }
}

/// Build a simple linear approval route. No conditional branching yet.
pub fn build_linear_route(id: &str, approvers: Vec<&str>) -> ApprovalRoute {
    let nodes: Vec<ApprovalNode> = approvers.iter().enumerate().map(|(i, name)| ApprovalNode {
        id: format!("node-{}", i),
        name: format!("Approval step {}", i + 1),
        approver: name.to_string(),
        next_node_id: if i + 1 < approvers.len() { Some(format!("node-{}", i + 1)) } else { None },
    }).collect();
    ApprovalRoute::new(id, "default-route", nodes)
}

/// Resolve next approver. TODO: add conditional branching based on context.
pub fn resolve_next_approver(route: &ApprovalRoute, _context: &HashMap<String, serde_json::Value>) -> Result<String> {
    match route.current_node() {
        Some(node) => Ok(node.approver.clone()),
        None => anyhow::bail!("no current node in route"),
    }
}

#[derive(Parser, Debug)]
#[command(name = "approval-routing", about = "Approval routing engine")]
struct Cli {
    #[arg(long, default_value = "zhangsan,lisi,wangwu")]
    approvers: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let approvers: Vec<&str> = cli.approvers.split(',').collect();
    let route = build_linear_route("route-1", approvers);
    let ctx = HashMap::new();
    let approver = resolve_next_approver(&route, &ctx)?;
    println!("current approver: {}", approver);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_branching_missing() {
        let route = build_linear_route("r1", vec!["manager"]);
        let mut ctx = HashMap::new();
        ctx.insert("amount".into(), serde_json::json!(2000000));
        let approver = resolve_next_approver(&route, &ctx).unwrap();
        assert_eq!(approver, "manager"); // BUG: should be "ceo" for amount > 1M
    }

    #[test]
    fn test_advance_validates_next_node_exists() {
        let mut route = ApprovalRoute::new("r1", "test", vec![
            ApprovalNode { id: "n1".into(), name: "Step 1".into(), approver: "alice".into(), next_node_id: Some("nonexistent".into()) },
            ApprovalNode { id: "n2".into(), name: "Step 2".into(), approver: "bob".into(), next_node_id: None },
        ]);
        // BUG: advance should check that next_node_id points to a valid node
        // Currently it just increments the index without validation
        let result = route.advance();
        // After fix: should return Err because n1's next_node_id="nonexistent" doesn't match n2
        assert!(result.is_err(), "advance should validate next_node_id points to existing node");
    }
}
''')

# q8: query-def-safety - add Unicode bypass test
w(f"{BASE}/query-def-safety/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTemplate {
    pub id: String,
    pub sql: String,
    pub params: HashMap<String, String>,
}

/// BUG: directly interpolates user input into SQL string - injection risk!
pub fn parse_template(template: &QueryTemplate) -> String {
    let mut sql = template.sql.clone();
    for (key, value) in &template.params {
        sql = sql.replace(&format!("{{{}}}", key), value);
    }
    sql
}

/// BUG: no validation at all - even Unicode bypass passes
pub fn validate_param_value(_value: &str) -> Result<()> {
    Ok(())
}

/// TODO: not yet implemented - should use $1, $2 placeholders
pub fn build_parameterized_query(_template: &QueryTemplate) -> Result<(String, Vec<String>)> {
    anyhow::bail!("not implemented")
}

#[derive(Parser, Debug)]
#[command(name = "query-def-safety", about = "SQL query template engine")]
struct Cli {
    #[arg(long, default_value = "SELECT * FROM contracts WHERE dept = {dept}")]
    template: String,
    #[arg(long, default_value = "finance")]
    dept: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut params = HashMap::new();
    params.insert("dept".into(), cli.dept.clone());
    let tmpl = QueryTemplate { id: "q1".into(), sql: cli.template, params };
    let sql = parse_template(&tmpl);
    println!("generated SQL: {}", sql);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_injection_vulnerability() {
        let mut params = HashMap::new();
        params.insert("dept".into(), "finance; DROP TABLE contracts; --".into());
        let tmpl = QueryTemplate { id: "q1".into(), sql: "SELECT * FROM t WHERE dept = {dept}".into(), params };
        let sql = parse_template(&tmpl);
        assert!(sql.contains("DROP TABLE"), "injection should be blocked but isn't");
    }

    #[test]
    fn test_validate_allows_everything() {
        assert!(validate_param_value("'; DROP TABLE x; --").is_ok());
    }

    #[test]
    fn test_unicode_bypass_not_blocked() {
        // Fullwidth Unicode characters can bypass keyword filtering
        // ＤＲＯＰ (fullwidth) should be detected as DROP equivalent
        let result = validate_param_value("ＤＲｏＰ TABLE x");
        // BUG: should fail for Unicode bypass but doesn't
        assert!(result.is_err(), "Unicode bypass should be detected");
    }

    #[test]
    fn test_parameterized_query_not_implemented() {
        let mut params = HashMap::new();
        params.insert("dept".into(), "finance".into());
        let tmpl = QueryTemplate { id: "q1".into(), sql: "SELECT * FROM t WHERE dept = {dept}".into(), params };
        let result = build_parameterized_query(&tmpl);
        // After fix: should return Ok with ($1, ["finance"])
        assert!(result.is_ok(), "parameterized query should be implemented");
    }
}
''')

# q9: report-validator - add gte/lte boundary bug + test
w(f"{BASE}/report-validator/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub id: String,
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub rule_id: String,
    pub passed: bool,
    pub message: String,
}

pub fn evaluate_rule(rule: &ValidationRule, record: &HashMap<String, serde_json::Value>) -> ValidationResult {
    let field_val = record.get(&rule.field);
    match field_val {
        None => ValidationResult { rule_id: rule.id.clone(), passed: false, message: format!("field '{}' not found", rule.field) },
        Some(val) => {
            let passed = match rule.operator.as_str() {
                // BUG: gte/lte use > and < instead of >= and <= (off-by-one)
                "gt" => val.as_f64().map_or(false, |v| v > rule.value.as_f64().unwrap_or(0.0)),
                "lt" => val.as_f64().map_or(false, |v| v < rule.value.as_f64().unwrap_or(0.0)),
                "eq" => val == &rule.value,
                "gte" => val.as_f64().map_or(false, |v| v > rule.value.as_f64().unwrap_or(0.0)),  // BUG: should be >=
                "lte" => val.as_f64().map_or(false, |v| v < rule.value.as_f64().unwrap_or(0.0)),  // BUG: should be <=
                "not_empty" => !val.to_string().is_empty(),
                _ => false,
            };
            ValidationResult { rule_id: rule.id.clone(), passed, message: if passed { "OK".into() } else { format!("{} {} {:?} failed", rule.field, rule.operator, rule.value) } }
        }
    }
}

/// TODO: add RuleChain with and/or combinations and short-circuit
pub fn validate_record(rules: &[ValidationRule], record: &HashMap<String, serde_json::Value>) -> Vec<ValidationResult> {
    rules.iter().map(|r| evaluate_rule(r, record)).collect()
}

#[derive(Parser, Debug)]
#[command(name = "report-validator", about = "Report validation rule engine")]
struct Cli {}

fn main() -> Result<()> {
    let rules = vec![
        ValidationRule { id: "r1".into(), field: "amount".into(), operator: "gt".into(), value: serde_json::json!(0) },
        ValidationRule { id: "r2".into(), field: "name".into(), operator: "not_empty".into(), value: serde_json::json!("") },
    ];
    let mut record = HashMap::new();
    record.insert("amount".into(), serde_json::json!(-100));
    record.insert("name".into(), serde_json::json!(""));
    let results = validate_record(&rules, &record);
    for r in &results {
        println!("{}: {} - {}", r.rule_id, if r.passed { "PASS" } else { "FAIL" }, r.message);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gte_boundary_bug() {
        // amount = 100, rule: gte 100 => should pass but fails due to bug
        let rule = ValidationRule { id: "r1".into(), field: "amount".into(), operator: "gte".into(), value: serde_json::json!(100) };
        let mut rec = HashMap::new();
        rec.insert("amount".into(), serde_json::json!(100));
        let result = evaluate_rule(&rule, &rec);
        assert!(result.passed, "amount=100 should pass gte 100, but off-by-one bug causes it to fail");
    }

    #[test]
    fn test_lte_boundary_bug() {
        // amount = 100, rule: lte 100 => should pass but fails due to bug
        let rule = ValidationRule { id: "r1".into(), field: "amount".into(), operator: "lte".into(), value: serde_json::json!(100) };
        let mut rec = HashMap::new();
        rec.insert("amount".into(), serde_json::json!(100));
        let result = evaluate_rule(&rule, &rec);
        assert!(result.passed, "amount=100 should pass lte 100, but off-by-one bug causes it to fail");
    }

    #[test]
    fn test_rule_chain_not_supported() {
        let rules = vec![
            ValidationRule { id: "r1".into(), field: "amount".into(), operator: "gt".into(), value: serde_json::json!(0) },
            ValidationRule { id: "r2".into(), field: "amount".into(), operator: "lt".into(), value: serde_json::json!(1000000) },
        ];
        let mut rec = HashMap::new();
        rec.insert("amount".into(), serde_json::json!(5000000));
        let results = validate_record(&rules, &rec);
        assert_eq!(results.len(), 2);
        assert!(results[0].passed);
        assert!(!results[1].passed);
    }
}
''')

# q10: deal-content-parser - add circular reference test
w(f"{BASE}/deal-content-parser/src/main.rs", r'''use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealContentNode {
    pub node_id: String,
    pub node_type: String,
    pub approver: Option<String>,
    pub fields: Vec<FieldMapping>,
    pub children: Vec<DealContentNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    pub source_field: String,
    pub target_field: String,
    pub required: bool,
}

/// BUG: uses unwrap() on optional values, causing panic on None
/// BUG: uses direct index on arrays, causing panic on out-of-bounds
/// BUG: no circular reference detection in recursive parsing
pub fn parse_deal_content(json_str: &str) -> Result<DealContentNode> {
    let value: serde_json::Value = serde_json::from_str(json_str)
        .with_context(|| "failed to parse dealcontent JSON")?;
    let obj = value.as_object().unwrap(); // BUG: unwrap on None panics
    let node_id = obj["nodeId"].as_str().unwrap().to_string(); // BUG: unwrap
    let node_type = obj["nodeType"].as_str().unwrap().to_string(); // BUG: unwrap
    let approver = obj.get("approver").and_then(|v| v.as_str()).map(String::from);
    let fields_arr = obj["fields"].as_array().unwrap(); // BUG: unwrap on missing key
    let mut fields = Vec::new();
    for f in fields_arr {
        fields.push(FieldMapping {
            source_field: f["sourceField"].as_str().unwrap().to_string(), // BUG
            target_field: f["targetField"].as_str().unwrap().to_string(), // BUG
            required: f["required"].as_bool().unwrap(), // BUG
        });
    }
    let children_arr = obj["children"].as_array().unwrap(); // BUG: unwrap on missing key
    let mut children = Vec::new();
    // BUG: no circular reference detection - recursive call without tracking visited IDs
    for c in children_arr {
        let child_json = serde_json::to_string(c).unwrap(); // BUG
        children.push(parse_deal_content(&child_json)?);
    }
    Ok(DealContentNode { node_id, node_type, approver, fields, children })
}

#[derive(Parser, Debug)]
#[command(name = "deal-content-parser", about = "DealContent JSON parser")]
struct Cli {}

fn main() -> Result<()> {
    let sample = r#"{"nodeId":"root","nodeType":"start","fields":[{"sourceField":"name","targetField":"contract_name","required":true}],"children":[{"nodeId":"c1","nodeType":"approval","approver":"zhangsan","fields":[],"children":[]}]}"#;
    let node = parse_deal_content(sample)?;
    println!("parsed: {}", serde_json::to_string_pretty(&node)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let json = r#"{"nodeId":"root","nodeType":"start","fields":[],"children":[]}"#;
        let node = parse_deal_content(json).unwrap();
        assert_eq!(node.node_id, "root");
    }

    #[test]
    fn test_missing_fields_panics() {
        let json = r#"{"nodeId":"root","nodeType":"start"}"#;
        let result = parse_deal_content(json);
        assert!(result.is_err(), "should return error not panic");
    }

    #[test]
    fn test_missing_children_panics() {
        let json = r#"{"nodeId":"root","nodeType":"start","fields":[]}"#;
        let result = parse_deal_content(json);
        assert!(result.is_err(), "should return error not panic");
    }

    #[test]
    fn test_circular_reference_not_detected() {
        // A -> B -> A (circular) should be detected and return error
        // Currently causes infinite recursion / stack overflow
        let json = r#"{"nodeId":"A","nodeType":"start","fields":[],"children":[{"nodeId":"B","nodeType":"approval","fields":[],"children":[{"nodeId":"A","nodeType":"start","fields":[],"children":[]}]}]}"#;
        let result = parse_deal_content(json);
        assert!(result.is_err(), "circular reference should be detected and return error");
    }
}
''')

# q11: approval-delegation - add chain + expired-skip test
w(f"{BASE}/approval-delegation/src/main.rs", r'''use anyhow::Result;
use chrono::Local;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRule {
    pub from_approver: String,
    pub to_approver: String,
    pub start_date: String,
    pub end_date: String,
    pub reason: String,
}

/// Resolve effective approver. Only checks direct delegation, no chain support.
/// BUG: doesn't follow delegation chains (A->B->C returns B instead of C)
pub fn resolve_approver(original: &str, delegations: &[DelegationRule], today: &str) -> Result<String> {
    for d in delegations {
        if d.from_approver == original && today >= &d.start_date && today <= &d.end_date {
            return Ok(d.to_approver.clone());
        }
    }
    Ok(original.to_string())
}

/// BUG: always returns Ok, never detects circular delegation
pub fn detect_circular_delegation(_delegations: &[DelegationRule]) -> Result<()> {
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "approval-delegation", about = "Approval delegation manager")]
struct Cli {
    #[arg(long, default_value = "zhangsan")]
    approver: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let delegations = vec![DelegationRule {
        from_approver: "zhangsan".into(), to_approver: "lisi".into(),
        start_date: "2026-05-01".into(), end_date: "2026-05-10".into(), reason: "vacation".into(),
    }];
    let today = Local::now().format("%Y-%m-%d").to_string();
    let effective = resolve_approver(&cli.approver, &delegations, &today)?;
    println!("effective approver for {}: {}", cli.approver, effective);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_not_detected() {
        let delegations = vec![
            DelegationRule { from_approver: "a".into(), to_approver: "b".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
            DelegationRule { from_approver: "b".into(), to_approver: "a".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
        ];
        assert!(detect_circular_delegation(&delegations).is_err(), "circular should be detected");
    }

    #[test]
    fn test_chain_not_supported() {
        let delegations = vec![
            DelegationRule { from_approver: "a".into(), to_approver: "b".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
            DelegationRule { from_approver: "b".into(), to_approver: "c".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
        ];
        let result = resolve_approver("a", &delegations, "2026-06-01").unwrap();
        assert_eq!(result, "c", "should follow chain to final approver");
    }

    #[test]
    fn test_expired_link_in_chain() {
        // A->B (active), B->C (expired) => should skip B->C and return B
        let delegations = vec![
            DelegationRule { from_approver: "a".into(), to_approver: "b".into(), start_date: "2026-01-01".into(), end_date: "2026-12-31".into(), reason: "".into() },
            DelegationRule { from_approver: "b".into(), to_approver: "c".into(), start_date: "2026-01-01".into(), end_date: "2026-03-01".into(), reason: "".into() },
        ];
        let result = resolve_approver("a", &delegations, "2026-06-01").unwrap();
        assert_eq!(result, "b", "expired link should be skipped, return last valid approver");
    }
}
''')

# q12: audit-trail - add nested diff test
w(f"{BASE}/audit-trail/src/main.rs", r'''use anyhow::Result;
use chrono::Local;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditLevel { Error, Warn, Info, Debug }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub trace_id: String, pub timestamp: String, pub operator: String,
    pub action: String, pub target_id: String, pub level: AuditLevel, pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldChange { pub field: String, pub old_value: String, pub new_value: String }

pub struct AuditStore { entries: Vec<AuditEntry> }

impl AuditStore {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn append(&mut self, entry: AuditEntry) { self.entries.push(entry); }
    pub fn query_by_trace_id(&self, trace_id: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.trace_id == trace_id).collect()
    }
}

/// TODO: not implemented - should support nested objects and arrays
pub fn compute_field_diff(_old: &serde_json::Value, _new: &serde_json::Value) -> Vec<FieldChange> {
    vec![]
}

#[derive(Parser, Debug)]
#[command(name = "audit-trail", about = "Approval audit trail system")]
struct Cli { #[arg(long, default_value = "trace-001")] trace_id: String }

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = AuditStore::new();
    store.append(AuditEntry {
        trace_id: cli.trace_id.clone(), timestamp: Local::now().to_rfc3339(),
        operator: "zhangsan".into(), action: "submit".into(), target_id: "contract-001".into(),
        level: AuditLevel::Info, detail: "submitted for approval".into(),
    });
    for e in store.query_by_trace_id(&cli.trace_id) {
        println!("{}", serde_json::to_string(e)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_field_diff() {
        let old = serde_json::json!({"amount": 10000});
        let new = serde_json::json!({"amount": 15000});
        let changes = compute_field_diff(&old, &new);
        assert!(!changes.is_empty(), "should detect amount change");
        assert!(changes.iter().any(|c| c.field == "amount" && c.old_value == "10000" && c.new_value == "15000"));
    }

    #[test]
    fn test_nested_object_diff() {
        let old = serde_json::json!({"approver": {"name": "zhangsan", "level": 2}});
        let new = serde_json::json!({"approver": {"name": "lisi", "level": 2}});
        let changes = compute_field_diff(&old, &new);
        assert!(!changes.is_empty(), "should detect nested change");
        assert!(changes.iter().any(|c| c.field == "approver.name" && c.old_value == "zhangsan" && c.new_value == "lisi"),
            "should use dot notation for nested fields");
    }

    #[test]
    fn test_array_diff() {
        let old = serde_json::json!({"tags": ["urgent", "finance"]});
        let new = serde_json::json!({"tags": ["urgent", "finance", "contract"]});
        let changes = compute_field_diff(&old, &new);
        assert!(!changes.is_empty(), "should detect array change");
    }
}
''')

# q13: concurrent-approval - add retry-without-reload test
w(f"{BASE}/concurrent-approval/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord { pub id: String, pub status: String, pub approver: String, pub version: u64 }

pub struct ApprovalStore { records: HashMap<String, ApprovalRecord> }

impl ApprovalStore {
    pub fn new() -> Self { Self { records: HashMap::new() } }
    pub fn get(&self, id: &str) -> Option<ApprovalRecord> { self.records.get(id).cloned() }
    /// BUG: does not check version for optimistic locking
    pub fn update(&mut self, record: ApprovalRecord) -> Result<()> {
        self.records.insert(record.id.clone(), record);
        Ok(())
    }
    pub fn insert(&mut self, record: ApprovalRecord) { self.records.insert(record.id.clone(), record); }
}

/// BUG: read-then-write not atomic, no version check
/// BUG: even if retry is added, it would use stale version from first read
pub fn approve_record(store: Arc<Mutex<ApprovalStore>>, record_id: &str, approver: &str) -> Result<()> {
    let mut s = store.lock().unwrap();
    let record = s.get(record_id).ok_or_else(|| anyhow::anyhow!("record not found"))?;
    let mut updated = record.clone();
    updated.status = "approved".into();
    updated.approver = approver.into();
    updated.version += 1;
    s.update(updated)
}

#[derive(Parser, Debug)]
#[command(name = "concurrent-approval", about = "Concurrent approval controller")]
struct Cli { #[arg(long, default_value = "rec-001")] record_id: String }

fn main() -> Result<()> {
    let store = Arc::new(Mutex::new(ApprovalStore::new()));
    store.lock().unwrap().insert(ApprovalRecord { id: "rec-001".into(), status: "pending".into(), approver: "".into(), version: 1 });
    approve_record(store.clone(), "rec-001", "zhangsan")?;
    let rec = store.lock().unwrap().get("rec-001").unwrap();
    println!("record: {}", serde_json::to_string(&rec)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_concurrent_overwrites() {
        let store = Arc::new(Mutex::new(ApprovalStore::new()));
        store.lock().unwrap().insert(ApprovalRecord { id: "rec-1".into(), status: "pending".into(), approver: "".into(), version: 1 });
        let mut handles = vec![];
        for i in 0..5 {
            let s = store.clone();
            handles.push(thread::spawn(move || approve_record(s, "rec-1", &format!("user-{}", i))));
        }
        let successes: Vec<_> = handles.into_iter().filter(|h| h.join().unwrap().is_ok()).count();
        assert!(successes <= 1, "only one should succeed, got {}", successes);
    }

    #[test]
    fn test_version_check_on_update() {
        let mut store = ApprovalStore::new();
        store.insert(ApprovalRecord { id: "r1".into(), status: "pending".into(), approver: "".into(), version: 1 });
        // Simulate: read with version 1, but someone else already updated to version 2
        let mut record = store.get("r1").unwrap();
        record.version = 1; // stale version
        record.status = "approved".into();
        // BUG: update should fail because version mismatch (stored=1, but conceptually should be 2)
        // Currently succeeds because no version check
        let result = store.update(record);
        assert!(result.is_err(), "update with stale version should fail");
    }
}
''')

# q23: approval-csv-export - add CSV escape test + filter test
w(f"{BASE}/approval-csv-export/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String, pub contract_name: String, pub amount: f64,
    pub status: String, pub approver: String, pub create_time: String,
}

pub struct ExportFilter {
    pub status: Option<String>, pub start_date: Option<String>,
    pub end_date: Option<String>, pub approver: Option<String>,
}

/// BUG: filter not implemented - returns all records
pub fn filter_records(records: &[ApprovalRecord], _filter: &ExportFilter) -> Vec<ApprovalRecord> {
    records.to_vec()
}

/// BUG: sort not implemented
pub fn sort_records(_records: &mut [ApprovalRecord], _field: &str, _dir: &str) {}

/// BUG: CSV export doesn't escape commas, quotes, or newlines in field values
pub fn export_csv(records: &[ApprovalRecord]) -> Result<String> {
    let mut lines = Vec::new();
    lines.push("id,contract_name,amount,status,approver,create_time".to_string());
    for rec in records {
        // BUG: no escaping - commas/quotes/newlines in values break CSV format
        lines.push(format!("{},{},{},{},{},{}", rec.id, rec.contract_name, rec.amount, rec.status, rec.approver, rec.create_time));
    }
    Ok(lines.join("\n"))
}

pub fn export_json(records: &[ApprovalRecord]) -> Result<String> {
    Ok(serde_json::to_string_pretty(records)?)
}

#[derive(Parser, Debug)]
#[command(name = "approval-csv-export", about = "Export approval data to CSV/JSON")]
struct Cli {
    #[arg(long, default_value = "csv")] format: String,
    #[arg(long)] status: Option<String>,
    #[arg(long)] start_date: Option<String>,
    #[arg(long)] end_date: Option<String>,
    #[arg(long)] approver: Option<String>,
    #[arg(long, default_value = "create_time:asc")] sort_by: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let records = vec![
        ApprovalRecord { id: "1".into(), contract_name: "Contract A".into(), amount: 500000.0, status: "approved".into(), approver: "zhangsan".into(), create_time: "2026-01-15".into() },
        ApprovalRecord { id: "2".into(), contract_name: "Contract B, Inc.".into(), amount: 2000000.0, status: "pending".into(), approver: "lisi".into(), create_time: "2026-03-20".into() },
    ];
    let filter = ExportFilter { status: cli.status, start_date: cli.start_date, end_date: cli.end_date, approver: cli.approver };
    let filtered = filter_records(&records, &filter);
    match cli.format.as_str() {
        "csv" => println!("{}", export_csv(&filtered)?),
        "json" => println!("{}", export_json(&filtered)?),
        _ => anyhow::bail!("unsupported format"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_escape_comma() {
        let records = vec![ApprovalRecord { id: "1".into(), contract_name: "A, B Corp".into(), amount: 100.0, status: "ok".into(), approver: "zhang".into(), create_time: "2026-01-01".into() }];
        let csv = export_csv(&records).unwrap();
        // Value with comma must be quoted
        assert!(csv.contains("\"A, B Corp\""), "CSV should escape commas with quotes");
    }

    #[test]
    fn test_csv_escape_quote() {
        let records = vec![ApprovalRecord { id: "1".into(), contract_name: "A \"Premium\" Deal".into(), amount: 100.0, status: "ok".into(), approver: "zhang".into(), create_time: "2026-01-01".into() }];
        let csv = export_csv(&records).unwrap();
        // Internal quotes must be doubled
        assert!(csv.contains("\"A \"\"Premium\"\" Deal\""), "CSV should escape internal quotes by doubling");
    }

    #[test]
    fn test_filter_by_status() {
        let records = vec![
            ApprovalRecord { id: "1".into(), contract_name: "A".into(), amount: 100.0, status: "approved".into(), approver: "z".into(), create_time: "2026-01-01".into() },
            ApprovalRecord { id: "2".into(), contract_name: "B".into(), amount: 200.0, status: "pending".into(), approver: "z".into(), create_time: "2026-01-02".into() },
        ];
        let filter = ExportFilter { status: Some("approved".into()), start_date: None, end_date: None, approver: None };
        let filtered = filter_records(&records, &filter);
        assert_eq!(filtered.len(), 1, "should filter to only approved records");
        assert_eq!(filtered[0].id, "1");
    }
}
''')

# q24: config-validator - add duplicate ID + orphan node test
w(f"{BASE}/config-validator/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig { pub id: String, pub name: String, pub start_node: String, pub nodes: Vec<NodeConfig> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig { pub id: String, pub node_type: String, pub next_node: Option<String>, pub approver: Option<String>, pub condition: Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationSeverity { Error, Warning }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue { pub severity: ValidationSeverity, pub node_id: Option<String>, pub message: String, pub suggestion: Option<String> }

/// BUG: only checks id and nodes empty, doesn't check duplicate node IDs
pub fn validate_structure(config: &WorkflowConfig) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    if config.id.is_empty() {
        issues.push(ValidationIssue { severity: ValidationSeverity::Error, node_id: None, message: "config id is empty".into(), suggestion: Some("add a non-empty id".into()) });
    }
    if config.nodes.is_empty() {
        issues.push(ValidationIssue { severity: ValidationSeverity::Error, node_id: None, message: "no nodes defined".into(), suggestion: Some("add at least start and end nodes".into()) });
    }
    // BUG: no duplicate node ID check
    issues
}

/// BUG: not implemented - should check reachability, orphan nodes, etc.
pub fn validate_logic(_config: &WorkflowConfig) -> Vec<ValidationIssue> { vec![] }

#[derive(Parser, Debug)]
#[command(name = "config-validator", about = "Workflow config validator")]
struct Cli { #[arg(long, default_value = "config.json")] config_file: String }

fn main() -> Result<()> {
    let sample = WorkflowConfig {
        id: "wf-1".into(), name: "Sample".into(), start_node: "start".into(),
        nodes: vec![
            NodeConfig { id: "start".into(), node_type: "start".into(), next_node: Some("end".into()), approver: None, condition: None },
            NodeConfig { id: "end".into(), node_type: "end".into(), next_node: None, approver: None, condition: None },
        ],
    };
    for i in validate_structure(&sample) { println!("{:?}: {}", i.severity, i.message); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_node_id_not_detected() {
        let config = WorkflowConfig {
            id: "wf-1".into(), name: "test".into(), start_node: "n1".into(),
            nodes: vec![
                NodeConfig { id: "n1".into(), node_type: "start".into(), next_node: Some("n2".into()), approver: None, condition: None },
                NodeConfig { id: "n1".into(), node_type: "approval".into(), next_node: None, approver: Some("zhangsan".into()), condition: None },
            ],
        };
        let issues = validate_structure(&config);
        assert!(issues.iter().any(|i| i.message.contains("duplicate")), "should detect duplicate node IDs");
    }

    #[test]
    fn test_orphan_node_not_detected() {
        let config = WorkflowConfig {
            id: "wf-1".into(), name: "test".into(), start_node: "n1".into(),
            nodes: vec![
                NodeConfig { id: "n1".into(), node_type: "start".into(), next_node: Some("n2".into()), approver: None, condition: None },
                NodeConfig { id: "n2".into(), node_type: "end".into(), next_node: None, approver: None, condition: None },
                NodeConfig { id: "n3".into(), node_type: "approval".into(), next_node: None, approver: Some("zhangsan".into()), condition: None },
            ],
        };
        let issues = validate_logic(&config);
        assert!(issues.iter().any(|i| i.message.contains("orphan") || i.message.contains("unreachable")), "should detect orphan node n3");
    }

    #[test]
    fn test_end_node_not_reachable() {
        let config = WorkflowConfig {
            id: "wf-1".into(), name: "test".into(), start_node: "n1".into(),
            nodes: vec![
                NodeConfig { id: "n1".into(), node_type: "start".into(), next_node: None, approver: None, condition: None },
                NodeConfig { id: "n2".into(), node_type: "end".into(), next_node: None, approver: None, condition: None },
            ],
        };
        let issues = validate_logic(&config);
        assert!(issues.iter().any(|i| i.message.contains("reachable") || i.message.contains("reach")), "should detect unreachable end node");
    }
}
''')

# q25: approval-metrics - add approval rate bug test (pending in denominator)
w(f"{BASE}/approval-metrics/src/main.rs", r'''use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String, pub dept: String, pub approval_type: String,
    pub status: String, pub submit_time: String, pub complete_time: Option<String>,
}

/// BUG: includes pending records in denominator, lowering the rate
pub fn calculate_approval_rate(records: &[ApprovalRecord]) -> f64 {
    if records.is_empty() { return 0.0; }
    let approved = records.iter().filter(|r| r.status == "approved").count();
    (approved as f64) / (records.len() as f64) * 100.0  // BUG: should exclude pending
}

/// TODO: not implemented
pub fn calculate_avg_duration(_records: &[ApprovalRecord]) -> f64 { 0.0 }

/// TODO: not implemented
pub fn identify_bottlenecks(_records: &[ApprovalRecord]) -> Vec<String> { vec![] }

/// TODO: not implemented
pub fn group_by(_records: &[ApprovalRecord], _field: &str) -> std::collections::HashMap<String, Vec<ApprovalRecord>> {
    std::collections::HashMap::new()
}

#[derive(Parser, Debug)]
#[command(name = "approval-metrics", about = "Approval metrics calculator")]
struct Cli { #[arg(long, default_value = "terminal")] output: String }

fn main() -> Result<()> {
    let records = vec![
        ApprovalRecord { id: "1".into(), dept: "finance".into(), approval_type: "contract".into(), status: "approved".into(), submit_time: "2026-01-01 09:00:00".into(), complete_time: Some("2026-01-02 15:00:00".into()) },
        ApprovalRecord { id: "2".into(), dept: "finance".into(), approval_type: "contract".into(), status: "rejected".into(), submit_time: "2026-01-03 10:00:00".into(), complete_time: Some("2026-01-03 16:00:00".into()) },
        ApprovalRecord { id: "3".into(), dept: "hr".into(), approval_type: "leave".into(), status: "pending".into(), submit_time: "2026-01-05 08:00:00".into(), complete_time: None },
    ];
    println!("approval rate: {:.1}%", calculate_approval_rate(&records));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approval_rate_excludes_pending() {
        // 1 approved, 1 rejected, 1 pending => rate should be 50% (1/2), not 33% (1/3)
        let records = vec![
            ApprovalRecord { id: "1".into(), dept: "f".into(), approval_type: "c".into(), status: "approved".into(), submit_time: "".into(), complete_time: Some("".into()) },
            ApprovalRecord { id: "2".into(), dept: "f".into(), approval_type: "c".into(), status: "rejected".into(), submit_time: "".into(), complete_time: Some("".into()) },
            ApprovalRecord { id: "3".into(), dept: "h".into(), approval_type: "l".into(), status: "pending".into(), submit_time: "".into(), complete_time: None },
        ];
        let rate = calculate_approval_rate(&records);
        // BUG: currently returns 33.3% (1/3), should be 50% (1/2)
        assert!((rate - 50.0).abs() < 0.1, "pending should be excluded from denominator, expected 50% got {:.1}%", rate);
    }

    #[test]
    fn test_avg_duration_not_implemented() {
        let records = vec![ApprovalRecord { id: "1".into(), dept: "f".into(), approval_type: "c".into(), status: "approved".into(), submit_time: "2026-01-01 09:00:00".into(), complete_time: Some("2026-01-02 15:00:00".into()) }];
        let duration = calculate_avg_duration(&records);
        assert!(duration > 0.0, "avg duration should be calculated");
    }

    #[test]
    fn test_group_by_not_implemented() {
        let records = vec![
            ApprovalRecord { id: "1".into(), dept: "finance".into(), approval_type: "contract".into(), status: "approved".into(), submit_time: "".into(), complete_time: None },
            ApprovalRecord { id: "2".into(), dept: "hr".into(), approval_type: "leave".into(), status: "approved".into(), submit_time: "".into(), complete_time: None },
        ];
        let groups = group_by(&records, "dept");
        assert!(!groups.is_empty(), "group_by should return grouped records");
        assert!(groups.contains_key("finance"), "should have finance group");
    }
}
''')

print("All 11 Rust source files updated with hidden bugs and test cases")
