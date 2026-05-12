use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::ApprovalStore;

pub type SharedStore = Arc<Mutex<ApprovalStore>>;

pub fn create_router(store: SharedStore) -> Router {
    Router::new()
        .route("/api/pending", get(get_pending))
        .route("/api/escalations", get(get_escalations))
        .route("/api/escalations/{id}/ack", post(ack_escalation))
        .with_state(store)
}

async fn get_pending(State(store): State<SharedStore>) -> impl IntoResponse {
    let store = store.lock().await;
    let pending: Vec<_> = store
        .requests
        .iter()
        .filter(|r| r.status == crate::models::ApprovalStatus::Pending)
        .cloned()
        .collect();
    Json(pending)
}

async fn get_escalations(State(store): State<SharedStore>) -> impl IntoResponse {
    let store = store.lock().await;
    let events = store.events.clone();
    Json(events)
}

async fn ack_escalation(
    State(store): State<SharedStore>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut store = store.lock().await;
    if let Some(event) = store.events.iter_mut().find(|e| e.id == id) {
        event.acknowledged = true;
        (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "not found"})))
    }
}
