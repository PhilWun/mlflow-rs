use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Run {
    info: RunInfo,
    data: RunData,
}

#[derive(Deserialize)]
struct RunInfo {
    run_uuid: String,
    experiment_id: String,
    run_name: String,
    user_id: String,
    status: String,
    start_time: u64,
    artifact_uri: String,
    lifecycle_stage: String,
    run_id: String,
}

#[derive(Deserialize)]
struct RunData {
    tags: Vec<RunTag>,
}

#[derive(Serialize, Deserialize)]
pub struct RunTag {
    pub key: String,
    pub value: String,
}

impl Run {
    // TODO
}
