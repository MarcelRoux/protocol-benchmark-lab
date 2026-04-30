use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::fs;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::app::state::AppState;

/*
Plan:
- Orchestrator is a thin wrapper to interface with load generator.
- Load generator is initially built into the orchestrator for simplicity.
- Load generator needs to be kept contained to simplify future refactor.

Endpoint design:
--> GET  /health
--> GET  /targets
GET  /scenarios
POST /benchmarks/run
GET  /benchmarks/{run_id}
GET  /benchmarks
*/

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/targets", get(targets))
        .route("/scenarios", get(scenarios))
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

#[derive(Deserialize, Serialize)]
pub(crate) struct ScenarioFile {
    name: String,
    protocol: String,
    concurrency: u32,
    duration_seconds: u32,
    payload_size: u32,
    connection_reuse: bool,
    compression: bool,
    multiplexing: bool,
}

#[derive(Serialize)]
pub(crate) struct ScenarioResponse {
    file: String,
    scenario: ScenarioFile,
}

pub(crate) async fn scenarios(
    State(state): State<AppState>,
) -> Result<Json<Vec<ScenarioResponse>>, (StatusCode, String)> {
    let mut dir = fs::read_dir(&state.scenarios_dir).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("read_dir failed: {err}"),
        )
    })?;

    let mut out = Vec::new();

    while let Some(entry) = dir.next_entry().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("next_entry failed: {e}"),
        )
    })? {
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext != "yaml" && ext != "yml" {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "invalid filename".to_string(),
            ))?
            .to_string();

        let raw = fs::read_to_string(&path).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("read_to_string failed: {e}"),
            )
        })?;

        let parsed: ScenarioFile = yaml_serde::from_str(&raw).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("invalid YAML in {file_name}: {e}"),
            )
        })?;

        out.push(ScenarioResponse {
            file: file_name,
            scenario: parsed,
        });
    }

    out.sort_by(|a, b| a.file.cmp(&b.file));

    Ok(Json(out))
}
