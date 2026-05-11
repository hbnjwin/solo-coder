use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractApprovalRequest {
    pub contract_name: Option<String>,
    pub sheet_amount: Option<f64>,
    pub approver: Option<String>,
    pub department: Option<String>,
    pub submission_date: Option<String>,
    pub urgent: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractApprovalResponse {
    pub request_id: String,
    pub status: String,
    pub priority: String,
    pub parsed_date: Option<ParsedDate>,
    pub validation_errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedDate {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ContractApprovalRequest {
    pub fn has_required_fields(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if self.contract_name.as_ref().map_or(true, |s| s.is_empty()) {
            errors.push("contract_name is required".to_string());
        }

        if self.sheet_amount.is_none() {
            errors.push("sheet_amount is required".to_string());
        } else if let Some(amount) = self.sheet_amount {
            if amount <= 0.0 {
                errors.push("sheet_amount must be positive".to_string());
            }
            if amount > 10_000_000.0 {
                warnings.push("sheet_amount exceeds standard limit".to_string());
            }
        }

        if self.approver.as_ref().map_or(true, |s| s.is_empty()) {
            errors.push("approver is required".to_string());
        }

        if self.submission_date.as_ref().map_or(true, |s| s.is_empty()) {
            errors.push("submission_date is required".to_string());
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}
