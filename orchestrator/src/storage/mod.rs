pub mod in_memory;
pub(crate) use in_memory::InMemoryRunStorage;

use crate::models::benchmarks::{RunError, RunId, RunRecord, RunRequest};

pub(crate) trait RunStorage {
    async fn create(&self, req: RunRequest) -> Result<RunId, RunError>;

    #[cfg(test)]
    async fn get(&self, run_id: RunId) -> Option<RunRecord>;

    #[cfg(test)]
    async fn list(&self) -> Vec<RunRecord>;

    async fn mark_running(&self, run_id: RunId) -> Result<RunRecord, RunError>;

    async fn mark_completed(&self, run_id: RunId) -> Result<RunRecord, RunError>;

    #[cfg(test)]
    async fn mark_cancelled(&self, run_id: RunId) -> Result<RunRecord, RunError>;

    #[cfg(test)]
    async fn mark_failed(&self, run_id: RunId) -> Result<RunRecord, RunError>;
}
