use chrono::Utc;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    models::benchmarks::{RunError, RunId, RunRecord, RunRequest, RunState},
    storage::RunStorage,
};

pub(crate) struct InMemoryRunStorage {
    runs: Arc<RwLock<HashMap<RunId, RunRecord>>>,
}

impl InMemoryRunStorage {
    pub(crate) fn new() -> Self {
        Self {
            runs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl RunStorage for InMemoryRunStorage {
    async fn create(&self, req: RunRequest) -> Result<RunId, RunError> {
        let run_id = RunId::new_v4();

        let record = RunRecord {
            id: run_id,
            requested_at: Utc::now().timestamp_millis(),
            started_at: None,
            finished_at: None,
            artifact_uri: None,
            summary: None,
            status: RunState::Queued,
            language: req.language,
            protocol: req.protocol,
            scenario: req.scenario,
        };

        // Store RunRequest.
        let mut runs = self.runs.write().await;
        runs.insert(run_id, record);

        Ok(run_id)
    }

    async fn get(&self, run_id: RunId) -> Option<RunRecord> {
        let runs = self.runs.read().await;

        runs.get(&run_id).cloned()
    }

    async fn list(&self) -> Vec<RunRecord> {
        let runs = self.runs.read().await;

        let mut runs_sorted: Vec<RunRecord> = runs.values().cloned().collect();
        runs_sorted.sort_by_key(|b| std::cmp::Reverse(b.requested_at));

        runs_sorted
    }

    async fn mark_running(&self, run_id: RunId) -> Result<RunRecord, RunError> {
        // Get mutable write reference.
        let mut runs = self.runs.write().await;

        // Get mutable record with get_mut, return not found error.
        let record = runs.get_mut(&run_id).ok_or(RunError::NotFound { run_id })?;

        // Enforce valid transition: Queued -> Running.
        if record.status != RunState::Queued {
            return Err(RunError::InvalidTransition {
                run_id,
                from: record.status.clone(),
                to: RunState::Running,
            });
        }

        // Mutate record.
        record.status = RunState::Running;
        record.started_at = Some(Utc::now().timestamp_millis());

        // Return clone.
        Ok(record.clone())
    }

    async fn mark_completed(
        &self,
        run_id: RunId,
        summary: String,
        artifact_uri: String,
    ) -> Result<RunRecord, RunError> {
        // Get mutable write reference.
        let mut runs = self.runs.write().await;

        // Get mutable record with get_mut, return not found error.
        let record = runs.get_mut(&run_id).ok_or(RunError::NotFound { run_id })?;

        // Enforce valid transition: Running -> Completed.
        if record.status != RunState::Running {
            return Err(RunError::InvalidTransition {
                run_id,
                from: record.status.clone(),
                to: RunState::Completed,
            });
        }

        // Mutate record.
        record.status = RunState::Completed;
        record.finished_at = Some(Utc::now().timestamp_millis());
        record.summary = Some(summary);
        record.artifact_uri = Some(artifact_uri);

        // Return clone.
        Ok(record.clone())
    }

    async fn mark_cancelled(&self, run_id: RunId) -> Result<RunRecord, RunError> {
        // Get mutable write reference.
        let mut runs = self.runs.write().await;

        // Get mutable record with get_mut, return not found error.
        let record = runs.get_mut(&run_id).ok_or(RunError::NotFound { run_id })?;

        // Enforce valid transition: Queued -> Cancelled OR Running -> Cancelled.
        if record.status != RunState::Queued && record.status != RunState::Running {
            return Err(RunError::InvalidTransition {
                run_id,
                from: record.status.clone(),
                to: RunState::Cancelled,
            });
        }

        // Mutate record.
        record.status = RunState::Cancelled;
        record.finished_at = Some(Utc::now().timestamp_millis());

        // Return clone.
        Ok(record.clone())
    }

    async fn mark_failed(&self, run_id: RunId) -> Result<RunRecord, RunError> {
        // Get mutable write reference.
        let mut runs = self.runs.write().await;

        // Get mutable record with get_mut, return not found error.
        let record = runs.get_mut(&run_id).ok_or(RunError::NotFound { run_id })?;

        // Enforce valid transition: Running -> Failed.
        if record.status != RunState::Running {
            return Err(RunError::InvalidTransition {
                run_id,
                from: record.status.clone(),
                to: RunState::Failed,
            });
        }

        // Mutate record.
        record.status = RunState::Failed;
        record.finished_at = Some(Utc::now().timestamp_millis());

        // Return clone.
        Ok(record.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::benchmarks::{Languages, Protocols, RunRequest, RunState};

    fn req() -> RunRequest {
        RunRequest {
            language: Languages::Rust,
            protocol: Protocols::Http,
            scenario: "http_baseline".to_string(),
        }
    }

    #[tokio::test]
    async fn create_and_get_returns_record() {
        let store = InMemoryRunStorage::new();
        let id = store.create(req()).await.unwrap();
        let rec = store.get(id).await.unwrap();

        assert_eq!(rec.status, RunState::Queued);
        assert!(rec.started_at.is_none())
    }

    #[tokio::test]
    async fn list_returns_sorted_records() {
        let store = InMemoryRunStorage::new();

        let n_records = 3;

        for _ in 1..=n_records {
            store.create(req()).await.unwrap();
        }

        let list = store.list().await;

        assert_eq!(list[0].status, RunState::Queued);
        assert!(list[0].started_at.is_none());
        assert_eq!(list.len(), n_records);
        assert!(list.is_sorted_by(|a, b| b.requested_at >= a.requested_at));
    }

    #[tokio::test]
    async fn mark_running_transition_valid() {
        let storage = InMemoryRunStorage::new();

        let run_id = storage.create(req()).await.expect("create should succeed");

        let updated = storage
            .mark_running(run_id)
            .await
            .expect("mark_running should succeed");

        assert_eq!(updated.status, RunState::Running);
        assert!(updated.started_at.is_some());
        assert!(updated.finished_at.is_none());
    }

    #[tokio::test]
    async fn mark_completed_transition_valid() {
        let storage = InMemoryRunStorage::new();

        let run_id = storage.create(req()).await.expect("create should succeed");

        storage
            .mark_running(run_id)
            .await
            .expect("mark_running should succeed");

        let updated = storage
            .mark_completed(
                run_id,
                "mock-summary".to_string(),
                "artifacts/mock.json".to_string(),
            )
            .await
            .expect("mark_completed should succeed from running");

        assert_eq!(updated.status, RunState::Completed);
        assert!(updated.started_at.is_some());
        assert!(updated.finished_at.is_some());
    }
    #[tokio::test]
    async fn mark_cancelled_transition_valid() {
        let storage = InMemoryRunStorage::new();

        let run_id = storage.create(req()).await.expect("create should succeed");

        let updated = storage
            .mark_cancelled(run_id)
            .await
            .expect("mark_cancelled should succeed from queued");

        assert_eq!(updated.status, RunState::Cancelled);
        assert!(updated.started_at.is_none());
        assert!(updated.finished_at.is_some());
    }
    #[tokio::test]
    async fn mark_failed_transition_valid() {
        let storage = InMemoryRunStorage::new();

        let run_id = storage.create(req()).await.expect("create should succeed");

        storage
            .mark_running(run_id)
            .await
            .expect("mark_running should succeed");

        let updated = storage
            .mark_failed(run_id)
            .await
            .expect("mark_failed should succeed from running");

        assert_eq!(updated.status, RunState::Failed);
        assert!(updated.started_at.is_some());
        assert!(updated.finished_at.is_some());
    }
}
