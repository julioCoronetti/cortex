use axum::{routing::get, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> axum::Json<Value> {
    axum::Json(json!({ "status": "ok" }))
}
