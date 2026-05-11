use anyhow::Result;
use clap::Parser;

use workflow_state_machine::engine::{auto_advance, StateMachine};
use workflow_state_machine::models::*;
use workflow_state_machine::store::{cleanup_stale_workflows, get_workflow_stats, WorkflowStore};
use workflow_state_machine::history::TransitionHistory;

#[derive(Parser, Debug)]
#[command(name = "workflow-engine", about = "Approval workflow state machine engine")]
struct Cli {
    #[arg(long, default_value = "3600")]
    ttl_secs: u64,

    #[arg(long, default_value = "advance")]
    action: String,

    #[arg(long)]
    workflow_id: Option<String>,

    #[arg(long)]
    target_state: Option<String>,
}

fn parse_state(s: &str) -> Result<ApprovalState> {
    match s {
        "draft" => Ok(ApprovalState::Draft),
        "submitted" => Ok(ApprovalState::Submitted),
        "under_review" => Ok(ApprovalState::UnderReview),
        "approved" => Ok(ApprovalState::Approved),
        "rejected" => Ok(ApprovalState::Rejected),
        "escalated" => Ok(ApprovalState::Escalated),
        "completed" => Ok(ApprovalState::Completed),
        _ => anyhow::bail!("unknown state: {}", s),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let machine = StateMachine::new();
    let mut store = WorkflowStore::new();
    let mut history = TransitionHistory::new(1000);

    store.insert(WorkflowInstance {
        id: "wf-001".into(),
        state: ApprovalState::Draft,
        title: "Contract approval #2026-001".into(),
        created_at: "2026-05-01T09:00:00Z".into(),
        updated_at: "2026-05-01T09:00:00Z".into(),
        version: 1,
        assignee: Some("zhang.wei".into()),
        priority: Priority::Normal,
    });

    store.insert(WorkflowInstance {
        id: "wf-002".into(),
        state: ApprovalState::Submitted,
        title: "Purchase order #PO-445".into(),
        created_at: "2026-04-20T14:30:00Z".into(),
        updated_at: "2026-04-20T14:30:00Z".into(),
        version: 2,
        assignee: Some("li.ming".into()),
        priority: Priority::High,
    });

    match cli.action.as_str() {
        "advance" => {
            let wf_id = cli.workflow_id.unwrap_or_else(|| "wf-001".into());
            let target = cli
                .target_state
                .map(|s| parse_state(&s))
                .transpose()?
                .unwrap_or(ApprovalState::Submitted);

            if let Some(instance) = store.get_mut(&wf_id) {
                let events = auto_advance(&machine, instance, target)?;
                for event in &events {
                    history.record(event.clone());
                }
                println!("Advanced {} through {} steps", wf_id, events.len());
            } else {
                println!("Workflow {} not found", wf_id);
            }
        }
        "cleanup" => {
            let removed = cleanup_stale_workflows(&mut store, cli.ttl_secs);
            println!("Removed {} stale workflows", removed.len());
        }
        "stats" => {
            let stats = get_workflow_stats(&store);
            println!("{}", stats);
        }
        _ => {
            anyhow::bail!("unknown action: {}", cli.action);
        }
    }

    Ok(())
}
