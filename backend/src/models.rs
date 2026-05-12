use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub applicant: String,
    pub current_approver: String,
    pub original_approver: String,
    pub escalation_level: u32,
    pub created_at: DateTime<Utc>,
    pub status: ApprovalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationEvent {
    pub id: String,
    pub request_id: String,
    pub from_approver: String,
    pub to_approver: String,
    pub escalation_level: u32,
    pub reason: String,
    pub created_at: DateTime<Utc>,
    pub acknowledged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationConfig {
    pub check_interval_secs: u64,
    pub timeout_minutes: i64,
    pub max_escalation_level: u32,
}

impl Default for EscalationConfig {
    fn default() -> Self {
        Self {
            check_interval_secs: 300,
            timeout_minutes: 60,
            max_escalation_level: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalStore {
    pub requests: Vec<ApprovalRequest>,
    pub events: Vec<EscalationEvent>,
    pub config: EscalationConfig,
    pub approver_hierarchy: Vec<String>,
}

impl ApprovalStore {
    pub fn new(hierarchy: Vec<String>) -> Self {
        Self {
            requests: Vec::new(),
            events: Vec::new(),
            config: EscalationConfig::default(),
            approver_hierarchy: hierarchy,
        }
    }
}
