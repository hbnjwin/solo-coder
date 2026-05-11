use crate::models::EscalationEvent;
use chrono::Utc;

pub fn get_escalation_target(current_level: u32) -> Option<&'static str> {
    match current_level {
        0 => Some("manager"),
        1 => Some("director"),
        2 => Some("vp"),
        _ => Some("super_admin"),
    }
}

pub fn apply_escalation(
    request_id: &str,
    current_level: u32,
) -> Option<EscalationEvent> {
    let target = get_escalation_target(current_level)?;

    let event = EscalationEvent {
        request_id: request_id.to_string(),
        from_level: current_level,
        to_level: current_level + 1,
        target_approver: target.to_string(),
        triggered_at: Utc::now(),
    };

    Some(event)
}

pub fn escalation_chain(max_level: u32) -> Vec<&'static str> {
    (0..=max_level)
        .filter_map(|level| get_escalation_target(level))
        .collect()
}
