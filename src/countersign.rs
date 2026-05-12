use crate::models::{CountersignResult, CountersignVote};

pub fn evaluate_countersign(
    node_id: &str,
    votes: &[CountersignVote],
    pass_ratio: f64,
) -> CountersignResult {
    let total_voters = votes.len();
    let approved_count = votes.iter().filter(|v| v.approved).count();
    let rejected_count = total_voters - approved_count;

    let actual_ratio = if total_voters > 0 {
        approved_count as f64 / total_voters as f64
    } else {
        0.0
    };

    CountersignResult {
        node_id: node_id.to_string(),
        total_voters,
        approved_count,
        rejected_count,
        pass_ratio: actual_ratio,
        passed: actual_ratio >= pass_ratio && total_voters > 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_vote(voter: &str, approved: bool) -> CountersignVote {
        CountersignVote {
            voter: voter.to_string(),
            approved,
            comment: None,
        }
    }

    #[test]
    fn test_all_approve() {
        let votes = vec![make_vote("a", true), make_vote("b", true), make_vote("c", true)];
        let result = evaluate_countersign("n1", &votes, 0.6);
        assert!(result.passed);
        assert_eq!(result.approved_count, 3);
    }

    #[test]
    fn test_majority_approve() {
        let votes = vec![make_vote("a", true), make_vote("b", true), make_vote("c", false)];
        let result = evaluate_countersign("n1", &votes, 0.6);
        assert!(result.passed);
        assert_eq!(result.approved_count, 2);
    }

    #[test]
    fn test_minority_approve() {
        let votes = vec![make_vote("a", true), make_vote("b", false), make_vote("c", false)];
        let result = evaluate_countersign("n1", &votes, 0.6);
        assert!(!result.passed);
    }

    #[test]
    fn test_any_reject_blocks_full_consensus() {
        let votes = vec![make_vote("a", true), make_vote("b", true), make_vote("c", false)];
        let result = evaluate_countersign("n1", &votes, 1.0);
        assert!(!result.passed);
    }

    #[test]
    fn test_no_votes() {
        let result = evaluate_countersign("n1", &[], 0.6);
        assert!(!result.passed);
        assert_eq!(result.total_voters, 0);
    }
}
