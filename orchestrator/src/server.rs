use crate::app::state::AppState;
use crate::http::routes::router;

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

pub async fn run(state: AppState) {
    tracing_subscriber::fmt::init();

    let app = router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Orchestrator server is running.");

    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
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
