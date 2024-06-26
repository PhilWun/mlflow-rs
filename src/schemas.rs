use serde::{Deserialize, Serialize};

use crate::{
    experiment::Experiment,
    run::{ArtifactInfo, Run, RunInfo, RunTag, Status},
};

#[derive(Serialize)]
pub(crate) struct CreateExperimentRequest {
    pub(crate) name: String,
    pub(crate) tags: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct CreateExperimentResponse {
    pub(crate) experiment_id: String,
}

#[derive(Serialize)]
pub(crate) struct GetExperimentRequest {
    pub(crate) experiment_id: String,
}

#[derive(Serialize)]
pub(crate) struct GetExperimentByNameRequest {
    pub(crate) experiment_name: String,
}

#[derive(Deserialize)]
pub(crate) struct GetExperimentResponse {
    pub(crate) experiment: Experiment,
}

#[derive(Serialize)]
pub(crate) struct CreateRunRequest {
    pub(crate) experiment_id: String,
    pub(crate) run_name: Option<String>,
    pub(crate) start_time: u128,
    pub(crate) tags: Vec<RunTag>,
}

#[derive(Deserialize)]
pub(crate) struct CreateRunResponse {
    pub(crate) run: Run,
}

#[derive(Serialize)]
pub(crate) struct GetRunRequest {
    pub(crate) run_id: String,
}

#[derive(Deserialize)]
pub(crate) struct GetRunResponse {
    pub(crate) run: Run,
}

#[derive(Serialize)]
pub(crate) struct UpdateRunRequest {
    pub(crate) run_id: String,
    pub(crate) status: Status,
    pub(crate) end_time: u128,
}

#[derive(Deserialize)]
pub(crate) struct UpdateRunResponse {
    pub(crate) run_info: RunInfo,
}

#[derive(Serialize)]
pub(crate) struct LogMetricRequest {
    pub(crate) run_id: String,
    pub(crate) key: String,
    pub(crate) value: f32,
    pub(crate) timestamp: u128,
    pub(crate) step: Option<u64>,
}

#[derive(Deserialize)]
pub(crate) struct LogMetricResponse {}

#[derive(Serialize)]
pub(crate) struct LogParameterRequest {
    pub(crate) run_id: String,
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Deserialize)]
pub(crate) struct LogParameterResponse {}

#[derive(Serialize)]
pub(crate) struct ListArtifactsRequest {
    pub(crate) run_id: String,
    pub(crate) path: String,
}

#[derive(Deserialize)]
pub(crate) struct ListArtifactsResponse {
    pub(crate) files: Vec<ArtifactInfo>,
}
