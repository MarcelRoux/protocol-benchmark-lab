use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;
use std::time::Duration;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

/*
Plan:
- Orchestrator is a thin wrapper to interface with load generator.
- Load generator is initially built into the orchestrator for simplicity.
- Load generator needs to be kept contained to simplify future refactor.

Endpoint design:
--> GET  /health
GET  /targets
GET  /scenarios
POST /benchmarks/run
GET  /benchmarks/{run_id}
GET  /benchmarks
*/

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/targets", get(targets))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ))
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Orchestrator server is running.");

    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health { status: "OK" })
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Protocols {
    Http,
}

#[derive(Serialize)]
struct Target {
    id: &'static str,
    name: &'static str,
    base_url: String,
    protocols: &'static [Protocols],
}

static PROTOCOLS: &[Protocols] = &[Protocols::Http];

async fn targets() -> Json<[Target; 1]> {
    let base_url =
        std::env::var("RUST_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    Json([Target {
        id: "rust",
        name: "Rust Axum",
        base_url,
        protocols: PROTOCOLS,
    }])
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Endpoint not found")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler.");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler.")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("{}", shutdown_message());},
        _ = terminate => {
            tracing::info!("{}", shutdown_message());},
    }
}

fn shutdown_message() -> &'static str {
    "Server is shutting down gracefully."
}
