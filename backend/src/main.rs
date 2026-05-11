use anyhow::Result;
use chrono::Utc;
use clap::Parser;

use workflow_escalation_backend::checker::{check_escalation, pending_requests};
use workflow_escalation_backend::escalator::apply_escalation;
use workflow_escalation_backend::models::{ApprovalRequest, EscalationRule};

#[derive(Parser, Debug)]
#[command(name = "workflow-escalation", about = "Approval workflow escalation engine")]
struct Cli {
    #[arg(long, default_value = "60")]
    timeout_minutes: i64,

    #[arg(long, default_value = "3")]
    max_level: u32,

    #[arg(long)]
    input: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let rule = EscalationRule {
        timeout_minutes: cli.timeout_minutes,
        max_level: cli.max_level,
        notify_on_escalate: true,
    };

    let requests: Vec<ApprovalRequest> = match &cli.input {
        Some(path) => {
            let data = std::fs::read_to_string(path)?;
            serde_json::from_str(&data)?
        }
        None => Vec::new(),
    };

    let now = Utc::now();
    let overdue = check_escalation(&requests, &rule, now);

    let pending = pending_requests(&requests);
    println!("Pending requests: {}", pending.len());
    println!("Overdue request IDs: {:?}", overdue);

    for id in &overdue {
        if let Some(req) = requests.iter().find(|r| &r.id == id) {
            if let Some(event) = apply_escalation(&req.id, req.escalation_level) {
                println!("Escalation: {} -> {}", event.target_approver, event.to_level);
            }
        }
    }

    Ok(())
}
