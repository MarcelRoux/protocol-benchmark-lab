use crate::models::benchmarks::RunRequest;

pub(crate) struct LoadGenResult {}

pub(crate) trait LoadGenerator {
    fn execute(&self, run: &RunRequest) -> LoadGenResult;
}
