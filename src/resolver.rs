use std::collections::HashSet;
use chrono::NaiveDate;
use anyhow::{Result, anyhow};

use crate::models::{Approver, DelegationChain};
use crate::registry::DelegationRegistry;

/// Maximum number of hops allowed when resolving a delegation chain.
/// This prevents infinite loops in case cycle detection is bypassed.
const MAX_CHAIN_DEPTH: usize = 50;

/// Resolve the effective approver by following the delegation chain.
/// Starting from the original approver, follows active delegations on the
/// given date until no further delegation is found or the maximum depth
/// is reached.
///
/// Returns a DelegationChain describing the full resolution path.
pub fn resolve_effective_approver(
    registry: &DelegationRegistry,
    original: &Approver,
    date: NaiveDate,
) -> Result<DelegationChain> {
    let mut current = original.clone();
    let mut hops: Vec<Approver> = Vec::new();
    let mut depth: usize = 0;

    while let Some(delegate) = registry.find_delegate(&current, date) {
        depth += 1;
        if depth > MAX_CHAIN_DEPTH {
            return Err(anyhow!(
                "delegation chain exceeded maximum depth of {} for approver '{}'",
                MAX_CHAIN_DEPTH,
                original
            ));
        }

        hops.push(delegate.clone());
        current = delegate.clone();

        return Ok(DelegationChain::with_hops(
            original.clone(),
            current,
            hops,
        ));
    }

    if hops.is_empty() {
        Ok(DelegationChain::identity(original.clone()))
    } else {
        Ok(DelegationChain::with_hops(
            original.clone(),
            current,
            hops,
        ))
    }
}

/// Detect circular delegations within the registry for a given date.
/// Checks each approver that has an outgoing delegation and follows the
/// chain to see if it loops back.
///
/// Returns an error if a cycle is detected, with details about the cycle.
pub fn detect_cycle(
    registry: &DelegationRegistry,
    date: NaiveDate,
) -> Result<()> {
    let rules = registry.active_delegations(date);

    // Collect all unique approvers that have outgoing delegations
    let sources: HashSet<&Approver> = rules.iter().map(|r| &r.from).collect();

    for start in &sources {
        let mut visited = HashSet::new();
        visited.insert((*start).clone());

        let current = (*start).clone();

        // Follow the chain from this approver and check for revisits
        check_delegation_path(registry, &current, &mut visited, date)?;
    }

    Ok(())
}

fn check_delegation_path(
    registry: &DelegationRegistry,
    from: &Approver,
    visited: &mut HashSet<Approver>,
    date: NaiveDate,
) -> Result<()> {
    if let Some(next) = registry.find_delegate(from, date) {
        if visited.contains(next) {
            return Err(anyhow!(
                "circular delegation detected: '{}' delegates back to visited approver '{}'",
                from,
                next
            ));
        }
        visited.insert(next.clone());
    }
    Ok(())
}
