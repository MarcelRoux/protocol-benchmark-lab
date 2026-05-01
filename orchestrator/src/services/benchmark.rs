use crate::models::benchmarks::{RunError, RunId, RunRecord, RunRequest};

pub(crate) trait BenchmarkService {
    fn start_run(&self, req: RunRequest) -> Result<RunId, RunError>;

    fn get_run(&self, run_id: RunId) -> Result<RunRecord, RunError>;

    fn list_runs(&self) -> Result<Vec<RunRecord>, RunError>;

    fn spawn_execution(&self, run_id: RunId) -> Result<RunId, RunError>;
}
