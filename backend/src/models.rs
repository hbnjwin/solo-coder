use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub timeout_minutes: i64,
    pub max_level: u32,
    pub notify_on_escalate: bool,
}

impl Default for EscalationRule {
    fn default() -> Self {
        Self {
            timeout_minutes: 60,
            max_level: 3,
            notify_on_escalate: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub title: String,
    pub submitted_at: DateTime<Utc>,
    pub approver: String,
    pub status: ApprovalStatus,
    pub escalation_level: u32,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationEvent {
    pub request_id: String,
    pub from_level: u32,
    pub to_level: u32,
    pub target_approver: String,
    pub triggered_at: DateTime<Utc>,
}

impl ApprovalRequest {
    pub fn new(id: impl Into<String>, title: impl Into<String>, approver: impl Into<String>, submitted_at: DateTime<Utc>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            submitted_at,
            approver: approver.into(),
            status: ApprovalStatus::Pending,
            escalation_level: 0,
            resolved_at: None,
        }
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self.status, ApprovalStatus::Approved | ApprovalStatus::Rejected)
    }
}
