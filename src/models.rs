use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents the lifecycle status of an approval record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

impl ApprovalStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApprovalStatus::Pending => "pending",
            ApprovalStatus::Approved => "approved",
            ApprovalStatus::Rejected => "rejected",
            ApprovalStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(ApprovalStatus::Pending),
            "approved" => Some(ApprovalStatus::Approved),
            "rejected" => Some(ApprovalStatus::Rejected),
            "cancelled" => Some(ApprovalStatus::Cancelled),
            _ => None,
        }
    }

    /// Returns true if the status transition is valid according to the workflow rules.
    pub fn can_transition_to(&self, target: &ApprovalStatus) -> bool {
        match (self, target) {
            (ApprovalStatus::Pending, ApprovalStatus::Approved) => true,
            (ApprovalStatus::Pending, ApprovalStatus::Rejected) => true,
            (ApprovalStatus::Pending, ApprovalStatus::Cancelled) => true,
            _ => false,
        }
    }
}

/// Core approval record representing a single approval request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String,
    pub title: String,
    pub status: ApprovalStatus,
    pub approver: Option<String>,
    pub version: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl ApprovalRecord {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            id: id.into(),
            title: title.into(),
            status: ApprovalStatus::Pending,
            approver: None,
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }

    /// Validates the record's internal consistency.
    /// Returns true if the record passes all validation checks.
    pub fn validate(&self) -> bool {
        if self.id.is_empty() {
            return false;
        }
        if self.version < 1 {
            return false;
        }
        // Approved/rejected records must have an approver set
        if matches!(self.status, ApprovalStatus::Approved | ApprovalStatus::Rejected) {
            if self.approver.is_none() {
                return false;
            }
        }
        // Timestamps: updated_at must be >= created_at
        if self.updated_at < self.created_at {
            return false;
        }
        true
    }
}

/// Represents a single entry in the audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub record_id: String,
    pub action: String,
    pub actor: String,
    pub timestamp: u64,
    pub from_status: String,
    pub to_status: String,
    pub version_at_time: u64,
}

impl AuditEntry {
    pub fn new(
        record_id: impl Into<String>,
        action: impl Into<String>,
        actor: impl Into<String>,
        from_status: &ApprovalStatus,
        to_status: &ApprovalStatus,
        version: u64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            record_id: record_id.into(),
            action: action.into(),
            actor: actor.into(),
            timestamp,
            from_status: from_status.as_str().to_string(),
            to_status: to_status.as_str().to_string(),
            version_at_time: version,
        }
    }
}
