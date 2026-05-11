use crate::models::{ContractApprovalRequest, ContractApprovalResponse, ParsedDate};
use uuid::Uuid;

pub fn parse_submission_date(date_str: &str) -> Result<ParsedDate, String> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() < 3 {
        return Err(format!("invalid date format: expected YYYY-MM-DD, got '{}'", date_str));
    }

    let year = parts[0].parse::<u32>()
        .map_err(|_| format!("invalid year component: '{}'", parts[0]))?;
    let month = parts[1].parse::<u32>()
        .map_err(|_| format!("invalid month component: '{}'", parts[1]))?;
    let day = parts[2].parse::<u32>()
        .map_err(|_| format!("invalid day component: '{}'", parts[2]))?;

    if month < 1 || month > 12 {
        return Err(format!("month out of range: {}", month));
    }
    if day < 1 || day > 31 {
        return Err(format!("day out of range: {}", day));
    }

    Ok(ParsedDate { year, month, day })
}

pub fn determine_priority(urgent_value: &Option<String>) -> String {
    match urgent_value {
        Some(val) if val == "yes" => "high".to_string(),
        Some(val) if val == "no" => "normal".to_string(),
        Some(_) => "normal".to_string(),
        None => "normal".to_string(),
    }
}

pub fn process_approval(json_body: &str) -> ContractApprovalResponse {
    let request: ContractApprovalRequest = match serde_json::from_str(json_body) {
        Ok(req) => req,
        Err(e) => {
            return ContractApprovalResponse {
                request_id: Uuid::new_v4().to_string(),
                status: "error".to_string(),
                priority: "normal".to_string(),
                parsed_date: None,
                validation_errors: vec![format!("failed to parse request body: {}", e)],
            };
        }
    };

    let validation = request.has_required_fields();
    if !validation.is_valid {
        return ContractApprovalResponse {
            request_id: Uuid::new_v4().to_string(),
            status: "rejected".to_string(),
            priority: "normal".to_string(),
            parsed_date: None,
            validation_errors: validation.errors,
        };
    }

    let parsed_date = match request.submission_date.as_deref() {
        Some(d) => match parse_submission_date(d) {
            Ok(pd) => Some(pd),
            Err(e) => {
                return ContractApprovalResponse {
                    request_id: Uuid::new_v4().to_string(),
                    status: "error".to_string(),
                    priority: "normal".to_string(),
                    parsed_date: None,
                    validation_errors: vec![e],
                };
            }
        },
        None => None,
    };

    let priority = determine_priority(&request.urgent);

    ContractApprovalResponse {
        request_id: Uuid::new_v4().to_string(),
        status: "accepted".to_string(),
        priority,
        parsed_date,
        validation_errors: vec![],
    }
}
