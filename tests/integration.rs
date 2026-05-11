use chrono::NaiveDate;
use approval_delegation::models::{Approver, DelegationRule, TimeRange};
use approval_delegation::registry::DelegationRegistry;
use approval_delegation::resolver::{resolve_effective_approver, detect_cycle};

fn test_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 6, 15).unwrap()
}

fn make_range() -> TimeRange {
    TimeRange::new(
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
    )
}

#[test]
fn test_single_delegation() {
    let registry = DelegationRegistry::from_rules(vec![
        DelegationRule::new(
            Approver::new("alice"),
            Approver::new("bob"),
            make_range(),
            "conference travel",
        ),
    ]);

    let chain = resolve_effective_approver(&registry, &Approver::new("alice"), test_date()).unwrap();
    assert_eq!(chain.effective.id, "bob");
}

#[test]
fn test_chain_delegation() {
    let registry = DelegationRegistry::from_rules(vec![
        DelegationRule::new(
            Approver::new("alice"),
            Approver::new("bob"),
            make_range(),
            "conference travel",
        ),
        DelegationRule::new(
            Approver::new("bob"),
            Approver::new("carol"),
            make_range(),
            "parental leave",
        ),
    ]);

    let chain = resolve_effective_approver(&registry, &Approver::new("alice"), test_date()).unwrap();
    assert_eq!(
        chain.effective.id, "carol",
        "expected full chain resolution to final delegate"
    );
}

#[test]
fn test_circular_detection() {
    let registry = DelegationRegistry::from_rules(vec![
        DelegationRule::new(
            Approver::new("alice"),
            Approver::new("bob"),
            make_range(),
            "vacation",
        ),
        DelegationRule::new(
            Approver::new("bob"),
            Approver::new("carol"),
            make_range(),
            "sick leave",
        ),
        DelegationRule::new(
            Approver::new("carol"),
            Approver::new("alice"),
            make_range(),
            "training",
        ),
    ]);

    let result = detect_cycle(&registry, test_date());
    assert!(
        result.is_err(),
        "expected cycle detection to find circular delegation"
    );
}

#[test]
fn test_self_delegation() {
    let registry = DelegationRegistry::from_rules(vec![
        DelegationRule::new(
            Approver::new("alice"),
            Approver::new("alice"),
            make_range(),
            "misconfigured",
        ),
    ]);

    let result = detect_cycle(&registry, test_date());
    assert!(
        result.is_err(),
        "self-delegation should be detected as a cycle"
    );
}

#[test]
fn test_no_delegation() {
    let registry = DelegationRegistry::new();

    let chain = resolve_effective_approver(&registry, &Approver::new("dave"), test_date()).unwrap();
    assert_eq!(
        chain.effective.id, "dave",
        "approver with no delegation should resolve to themselves"
    );
}
