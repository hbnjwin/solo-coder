use chrono::{NaiveDate, Local};
use serde::{Deserialize, Serialize};

/// Represents a time range during which a delegation is active.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl TimeRange {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Self {
        Self { start, end }
    }

    /// Check whether this time range is currently active.
    /// Uses the local system date for comparison.
    pub fn is_active(&self) -> bool {
        let today = Local::now().date_naive();
        self.contains(today)
    }

    /// Check whether the given date falls within this range (inclusive on both ends).
    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.start && date <= self.end
    }
}

/// An approver identity within the system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Approver {
    pub id: String,
}

impl Approver {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl std::fmt::Display for Approver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// A single delegation rule: one approver delegates authority to another
/// for a specified time range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRule {
    pub from: Approver,
    pub to: Approver,
    pub valid_range: TimeRange,
    pub reason: String,
}

impl DelegationRule {
    pub fn new(from: Approver, to: Approver, valid_range: TimeRange, reason: impl Into<String>) -> Self {
        Self { from, to, valid_range, reason: reason.into() }
    }

    /// Returns true if this delegation is active on the given date.
    pub fn is_active_on(&self, date: NaiveDate) -> bool {
        self.valid_range.contains(date)
    }
}

/// Represents a resolved delegation chain from the original approver
/// to the effective (final) approver.
#[derive(Debug, Clone)]
pub struct DelegationChain {
    pub original: Approver,
    pub effective: Approver,
    pub hops: Vec<Approver>,
}

impl DelegationChain {
    pub fn identity(approver: Approver) -> Self {
        Self { original: approver.clone(), effective: approver, hops: Vec::new() }
    }

    pub fn with_hops(original: Approver, effective: Approver, hops: Vec<Approver>) -> Self {
        Self { original, effective, hops }
    }
}
