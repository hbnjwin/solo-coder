use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApprovalState {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Escalated,
    Completed,
}

impl fmt::Display for ApprovalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Submitted => write!(f, "submitted"),
            Self::UnderReview => write!(f, "under_review"),
            Self::Approved => write!(f, "approved"),
            Self::Rejected => write!(f, "rejected"),
            Self::Escalated => write!(f, "escalated"),
            Self::Completed => write!(f, "completed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstance {
    pub id: String,
    pub state: ApprovalState,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub version: u64,
    pub assignee: Option<String>,
    pub priority: Priority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRule {
    pub from: ApprovalState,
    pub to: ApprovalState,
    pub requires_comment: bool,
    pub min_priority: Option<Priority>,
}

impl TransitionRule {
    pub fn new(from: ApprovalState, to: ApprovalState) -> Self {
        Self {
            from,
            to,
            requires_comment: false,
            min_priority: None,
        }
    }

    pub fn with_comment_required(mut self) -> Self {
        self.requires_comment = true;
        self
    }

    pub fn with_min_priority(mut self, p: Priority) -> Self {
        self.min_priority = Some(p);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionEvent {
    pub workflow_id: String,
    pub from: ApprovalState,
    pub to: ApprovalState,
    pub timestamp: String,
    pub actor: Option<String>,
    pub comment: Option<String>,
}
