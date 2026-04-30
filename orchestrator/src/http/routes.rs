use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;
use std::time::Duration;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::app::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/targets", get(targets))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ))
        .fallback(handler_404)
        .with_state(state)
}

#[derive(Serialize)]
pub(crate) struct Health {
    status: &'static str,
}

pub(crate) async fn health() -> Json<Health> {
    Json(Health { status: "OK" })
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Protocols {
    Http,
}

#[derive(Serialize)]
pub(crate) struct Target {
    id: &'static str,
    name: &'static str,
    base_url: String,
    protocols: &'static [Protocols],
}

static PROTOCOLS: &[Protocols] = &[Protocols::Http];

pub(crate) async fn targets(State(state): State<AppState>) -> Json<[Target; 1]> {
    Json([Target {
        id: "rust",
        name: "Rust Axum",
        base_url: state.base_url.clone(),
        protocols: PROTOCOLS,
    }])
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Endpoint not found")
}
