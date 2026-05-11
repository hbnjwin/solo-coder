use crate::models::*;

pub struct ValidationReport {
    pub results: Vec<ValidationResult>,
    pub chain_results: Vec<ChainResult>,
    pub total_rules: usize,
    pub passed_count: usize,
    pub failed_count: usize,
}

impl ValidationReport {
    pub fn from_results(results: Vec<ValidationResult>, chain_results: Vec<ChainResult>) -> Self {
        let total_rules = results.len();
        let passed_count = results.iter().filter(|r| r.passed).count();
        let failed_count = total_rules - passed_count;

        Self {
            results,
            chain_results,
            total_rules,
            passed_count,
            failed_count,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.failed_count == 0 && self.chain_results.iter().all(|c| c.passed)
    }

    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "Validation: {} passed, {} failed out of {} rules",
            self.passed_count, self.failed_count, self.total_rules
        ));

        for chain in &self.chain_results {
            let status = if chain.passed { "PASS" } else { "FAIL" };
            lines.push(format!(
                "  Chain '{}' ({:?}): {}",
                chain.chain_id, chain.mode, status
            ));
        }

        for result in &self.results {
            if !result.passed {
                lines.push(format!("  FAIL: {}", result.message));
            }
        }

        lines.join("\n")
    }
}
