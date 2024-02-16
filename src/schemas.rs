use serde::{Deserialize, Serialize};

use crate::{
    experiment::Experiment,
    run::{Run, RunTag},
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
    pub(crate) start_time: u64,
    pub(crate) tags: Vec<RunTag>,
}

#[derive(Deserialize)]
pub(crate) struct CreateRunResponse {
    pub(crate) run: Run,
}
