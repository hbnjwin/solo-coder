use anyhow::Result;
use clap::Parser;
use std::sync::{Arc, Mutex};

use concurrent_approval::audit::AuditLog;
use concurrent_approval::models::ApprovalRecord;
use concurrent_approval::service::ApprovalService;
use concurrent_approval::store::ApprovalStore;

#[derive(Parser, Debug)]
#[command(name = "concurrent-approval", about = "Concurrent approval workflow controller")]
struct Cli {
    /// The record ID to operate on
    #[arg(long, default_value = "rec-001")]
    record_id: String,

    /// The approver name
    #[arg(long, default_value = "admin")]
    approver: String,

    /// Action to perform: approve, reject, status
    #[arg(long, default_value = "approve")]
    action: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let store = Arc::new(Mutex::new(ApprovalStore::new()));
    let audit = Arc::new(Mutex::new(AuditLog::new()));
    let service = ApprovalService::new(store.clone(), audit.clone());

    // Create a sample record if it doesn't exist
    let record_id = &cli.record_id;
    if !store.lock().unwrap().contains(record_id) {
        let record = ApprovalRecord::new(record_id, "Sample Approval Request");
        store.lock().unwrap().insert(record)?;
    }

    match cli.action.as_str() {
        "approve" => {
            service.approve_record(record_id, &cli.approver)?;
            let rec = service.get_record(record_id)?;
            println!("{}", serde_json::to_string_pretty(&rec)?);
        }
        "reject" => {
            service.reject_record(record_id, &cli.approver)?;
            let rec = service.get_record(record_id)?;
            println!("{}", serde_json::to_string_pretty(&rec)?);
        }
        "status" => {
            let rec = service.get_record(record_id)?;
            println!("{}", serde_json::to_string_pretty(&rec)?);
        }
        other => {
            eprintln!("unknown action: {}", other);
            std::process::exit(1);
        }
    }

    Ok(())
}
