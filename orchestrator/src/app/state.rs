use std::sync::Arc;

use crate::loadgen::MockLoadGenerator;
use crate::storage::InMemoryRunStorage;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) base_url: String,
    pub(crate) scenarios_dir: String,
    pub(crate) storage: Arc<InMemoryRunStorage>,
    pub(crate) loadgen: Arc<MockLoadGenerator>,
}
