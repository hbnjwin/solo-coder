use crate::evaluator::evaluate_rule;
use crate::models::*;

pub fn evaluate_chain(chain: &RuleChain, record: &Record) -> ChainResult {
    let mut details = Vec::new();

    match chain.mode {
        ChainMode::And => {
            let mut all_passed = true;
            for rule in &chain.rules {
                let result = evaluate_rule(rule, record);
                if !result.passed {
                    all_passed = false;
                }
                details.push(result);
            }
            ChainResult {
                chain_id: chain.id.clone(),
                mode: ChainMode::And,
                passed: all_passed,
                details,
            }
        }
        ChainMode::Or => {
            let mut any_passed = false;
            for rule in &chain.rules {
                let result = evaluate_rule(rule, record);
                if result.passed {
                    any_passed = true;
                }
                details.push(result);
            }
            ChainResult {
                chain_id: chain.id.clone(),
                mode: ChainMode::Or,
                passed: any_passed,
                details,
            }
        }
    }
}

pub fn evaluate_chains(chains: &[RuleChain], record: &Record) -> Vec<ChainResult> {
    chains.iter().map(|c| evaluate_chain(c, record)).collect()
}
