use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) type RunId = Uuid;

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Languages {
    Rust,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Protocols {
    Http,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RunState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct RunRequest {
    pub(crate) language: Languages,
    pub(crate) protocol: Protocols,
    pub(crate) scenario: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct RunStatus {
    pub(crate) id: RunId,
    pub(crate) status: RunState,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct RunRecord {
    pub(crate) id: RunId,
    pub(crate) requested_at: i64,
    pub(crate) started_at: Option<i64>,
    pub(crate) finished_at: Option<i64>,
    pub(crate) artifact_uri: Option<String>,
    pub(crate) summary: Option<String>,
    pub(crate) status: RunState,
    pub(crate) language: Languages,
    pub(crate) protocol: Protocols,
    pub(crate) scenario: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum RunError {
    NotFound {
        run_id: RunId,
    },
    InvalidTransition {
        run_id: RunId,
        from: RunState,
        to: RunState,
    },
    Storage(String),
}
