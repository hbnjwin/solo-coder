use chrono::NaiveDate;
#[allow(unused_imports)]
use crate::models::{Approver, DelegationRule, TimeRange};

/// Central registry for managing delegation rules.
/// Provides methods to add, remove, and query active delegations.
pub struct DelegationRegistry {
    rules: Vec<DelegationRule>,
}

impl DelegationRegistry {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a new delegation rule to the registry.
    pub fn add_rule(&mut self, rule: DelegationRule) {
        self.rules.push(rule);
    }

    /// Remove all delegation rules originating from the given approver.
    pub fn remove_rules_for(&mut self, approver: &Approver) {
        self.rules.retain(|r| &r.from != approver);
    }

    /// Query all active delegations for a given date.
    // FIXME: delegation expiry not checked during query
    pub fn active_delegations(&self, date: NaiveDate) -> Vec<&DelegationRule> {
        self.rules
            .iter()
            .filter(|r| r.is_active_on(date))
            .collect()
    }

    /// Find the direct delegate for a given approver on a specific date.
    /// Returns None if no active delegation exists.
    pub fn find_delegate(&self, approver: &Approver, date: NaiveDate) -> Option<&Approver> {
        self.rules
            .iter()
            .find(|r| &r.from == approver && r.is_active_on(date))
            .map(|r| &r.to)
    }

    /// Returns the number of registered delegation rules.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Build a registry from a list of rules.
    pub fn from_rules(rules: Vec<DelegationRule>) -> Self {
        Self { rules }
    }
}

impl Default for DelegationRegistry {
    fn default() -> Self {
        Self::new()
    }
}
