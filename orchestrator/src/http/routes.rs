use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::fs;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::app::state::AppState;
use crate::loadgen::load_generator::LoadGenerator;
use crate::models::benchmarks::{Protocols, RunError, RunId, RunRecord, RunRequest, RunStatus};
use crate::storage::RunStorage;

/*
Plan:
- Orchestrator is a thin wrapper to interface with load generator.
- Load generator is initially built into the orchestrator for simplicity.
- Load generator needs to be kept contained to simplify future refactor.

Endpoint design:
--> GET  /health
--> GET  /targets
--> GET  /scenarios
POST /benchmarks/run
GET  /benchmarks/{run_id}
GET  /benchmarks
*/

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/targets", get(targets))
        .route("/scenarios", get(scenarios))
        .route("/benchmarks/run", post(benchmark_run))
        .route("/benchmarks/{run_id}", get(benchmark_fetch))
        .route("/benchmarks/{run_id}/cancel", post(benchmark_cancel))
        .route("/benchmarks", get(benchmark_fetch_all))
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

async fn benchmark_run(
    State(state): State<AppState>,
    Json(req): Json<RunRequest>,
) -> Result<(StatusCode, Json<RunStatus>), (StatusCode, String)> {
    let run_req = RunRequest {
        language: req.language,
        protocol: req.protocol,
        scenario: req.scenario.clone(),
    };
    let run_id = state.storage.create(req).await.map_err(map_storage_err)?;

    state
        .storage
        .mark_running(run_id)
        .await
        .map_err(map_storage_err)?;
    let result = match state.loadgen.execute(&run_req) {
        Ok(res) => res,
        Err(err) => {
            let _ = state.storage.mark_failed(run_id).await;
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("loadgen failed: {err}"),
            ));
        }
    };
    let record = state
        .storage
        .mark_completed(run_id, result.summary, result.artifact_uri)
        .await
        .map_err(map_storage_err)?;

    Ok((
        StatusCode::ACCEPTED,
        Json(RunStatus {
            id: record.id,
            status: record.status,
        }),
    ))
}

async fn benchmark_fetch(
    State(state): State<AppState>,
    Path(run_id): Path<RunId>,
) -> Result<Json<RunRecord>, (StatusCode, String)> {
    state
        .storage
        .get(run_id)
        .await
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, format!("run {run_id} not found")))
}

async fn benchmark_cancel(
    State(state): State<AppState>,
    Path(run_id): Path<RunId>,
) -> Result<Json<RunStatus>, (StatusCode, String)> {
    let record = state
        .storage
        .mark_cancelled(run_id)
        .await
        .map_err(map_storage_err)?;

    Ok(Json(RunStatus {
        id: record.id,
        status: record.status,
    }))
}

async fn benchmark_fetch_all(State(state): State<AppState>) -> Json<Vec<RunRecord>> {
    Json(state.storage.list().await)
}

fn map_storage_err(err: RunError) -> (StatusCode, String) {
    match err {
        RunError::NotFound { run_id } => (StatusCode::NOT_FOUND, format!("run {run_id} not found")),
        RunError::InvalidTransition { run_id, from, to } => (
            StatusCode::CONFLICT,
            format!("invalid transition for {run_id}: {from:?} -> {to:?}"),
        ),
        RunError::Storage(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
    }
}
