use crate::models::benchmarks::RunRequest;

pub(crate) struct LoadGenResult {
    pub(crate) summary: String,
    pub(crate) artifact_uri: String,
}

pub(crate) trait LoadGenerator {
    fn execute(&self, run: &RunRequest) -> Result<LoadGenResult, String>;
}
