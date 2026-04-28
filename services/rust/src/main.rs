use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use std::time::Duration;
use tokio;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/echo", post(echo))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server is running.");

    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
}

async fn root() -> &'static str {
    "Hello, World! - Rust Service."
}

async fn health() -> &'static str {
    "OK"
}

async fn echo(body: String) -> String {
    body
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
