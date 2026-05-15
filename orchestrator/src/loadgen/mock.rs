use crate::loadgen::load_generator::{LoadGenResult, LoadGenerator};
use crate::models::benchmarks::RunRequest;

pub(crate) struct MockLoadGenerator;

impl MockLoadGenerator {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LoadGenerator for MockLoadGenerator {
    fn execute(&self, _run: &RunRequest) -> Result<LoadGenResult, String> {
        Ok(LoadGenResult {
            summary: "mock-summary".to_string(),
            artifact_uri: "artifacts/mock.json".to_string(),
        })
    }
}
