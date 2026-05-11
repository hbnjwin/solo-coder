use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String,
    pub department: String,
    pub approver: String,
    pub approval_type: String,
    pub status: String,
    pub submitted_at: String,
    pub completed_at: Option<String>,
}

impl ApprovalRecord {
    pub fn submitted_datetime(&self) -> Option<NaiveDateTime> {
        NaiveDateTime::parse_from_str(&self.submitted_at, DATETIME_FORMAT).ok()
    }

    pub fn completed_datetime(&self) -> Option<NaiveDateTime> {
        self.completed_at
            .as_ref()
            .and_then(|ts| NaiveDateTime::parse_from_str(ts, DATETIME_FORMAT).ok())
    }

    pub fn duration_hours(&self) -> Option<f64> {
        let start = self.submitted_datetime()?;
        let end = self.completed_datetime()?;
        let duration = end.signed_duration_since(start);
        Some(duration.num_minutes() as f64 / 60.0)
    }

    pub fn is_resolved(&self) -> bool {
        self.status == "approved" || self.status == "rejected"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricResult {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub sample_size: usize,
}

impl fmt::Display for MetricResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:.2}{} (n={})", self.name, self.value, self.unit, self.sample_size)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: String,
    pub end: String,
}

impl TimeRange {
    pub fn contains(&self, record: &ApprovalRecord) -> bool {
        let start = NaiveDateTime::parse_from_str(&self.start, DATETIME_FORMAT);
        let end = NaiveDateTime::parse_from_str(&self.end, DATETIME_FORMAT);
        let submitted = record.submitted_datetime();

        match (start.ok(), end.ok(), submitted) {
            (Some(s), Some(e), Some(ts)) => ts >= s && ts <= e,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GroupKey {
    Department(String),
    Approver(String),
    ApprovalType(String),
    Status(String),
}

impl GroupKey {
    pub fn from_field(record: &ApprovalRecord, field: &str) -> Option<Self> {
        match field {
            "department" => Some(GroupKey::Department(record.department.clone())),
            "approver" => Some(GroupKey::Approver(record.approver.clone())),
            "approval_type" => Some(GroupKey::ApprovalType(record.approval_type.clone())),
            "status" => Some(GroupKey::Status(record.status.clone())),
            _ => None,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            GroupKey::Department(s) => s,
            GroupKey::Approver(s) => s,
            GroupKey::ApprovalType(s) => s,
            GroupKey::Status(s) => s,
        }
    }
}
