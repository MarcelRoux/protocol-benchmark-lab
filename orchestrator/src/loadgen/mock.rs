use crate::loadgen::load_generator::{LoadGenResult, LoadGenerator};

pub(crate) struct MockLoadGenerator;

impl MockLoadGenerator {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LoadGenerator for MockLoadGenerator {
    fn execute(&self, _run: &crate::models::benchmarks::RunRequest) -> LoadGenResult {
        todo!()
    }
}
