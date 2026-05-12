use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

use escalation_notify::api::create_router;
use escalation_notify::escalation::{add_request, check_timeouts};
use escalation_notify::models::ApprovalStore;

#[derive(Parser, Debug)]
#[command(name = "escalation-notify", about = "Approval escalation with HTTP API")]
struct Cli {
    #[arg(long, default_value_t = 3000)]
    port: u16,

    #[arg(long, default_value_t = 60)]
    timeout_minutes: i64,

    #[arg(long, default_value_t = 3)]
    max_escalation: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut store = ApprovalStore::new(vec![
        "team-lead".to_string(),
        "manager".to_string(),
        "director".to_string(),
        "vp".to_string(),
    ]);
    store.config.timeout_minutes = cli.timeout_minutes;
    store.config.max_escalation_level = cli.max_escalation;

    add_request(&mut store, "alice", "team-lead");
    add_request(&mut store, "bob", "manager");

    let shared_store = Arc::new(Mutex::new(store));

    let checker_store = shared_store.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            let mut s = checker_store.lock().await;
            let events = check_timeouts(&mut s);
            for evt in &events {
                println!("[ESCALATION] {} -> {} for request {}", evt.from_approver, evt.to_approver, evt.request_id);
            }
        }
    });

    let app = create_router(shared_store);
    let addr = format!("0.0.0.0:{}", cli.port);
    println!("Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
