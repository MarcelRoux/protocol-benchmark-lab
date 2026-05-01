mod server;
use std::sync::Arc;

use server::run;

mod app;
mod http;
mod loadgen;
mod models;
mod services;
mod storage;

use crate::app::state::AppState;
use crate::loadgen::MockLoadGenerator;
use crate::storage::InMemoryRunStorage;

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
    let storage = Arc::new(InMemoryRunStorage::new());
    let loadgen = Arc::new(MockLoadGenerator::new());

    let state = AppState {
        base_url: std::env::var("RUST_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        scenarios_dir: std::env::var("SCENARIOS_DIR")
            .unwrap_or_else(|_| "../benchmarks/scenarios".to_string()),
        storage,
        loadgen,
    };

    run(state).await;
}
