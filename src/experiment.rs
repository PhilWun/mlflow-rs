use serde::Deserialize;

use crate::{
    run::{Run, RunTag},
    schemas::{
        CreateExperimentRequest, CreateExperimentResponse, CreateRunRequest, CreateRunResponse,
        GetExperimentByNameRequest, GetExperimentRequest, GetExperimentResponse,
    },
    utils::check_for_error_response,
};

#[derive(Deserialize)]
pub struct Experiment {
    experiment_id: String,
    name: String,
    artifact_location: String,
    lifecycle_stage: String,
    last_update_time: u64,
    creation_time: u64,
}

impl Experiment {
    pub fn new(api_root: &str, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{api_root}/api/2.0/mlflow/experiments/create"))
            .json(&CreateExperimentRequest {
                name: name.to_owned(),
                tags: vec![],
            })
            .send()?
            .error_for_status()?
            .text()?;

        check_for_error_response(&response)?;

        let experiment_id = match serde_json::from_str::<CreateExperimentResponse>(&response) {
            Ok(r) => r.experiment_id,
            Err(e) => return Err(Box::new(e)),
        };

        Self::search_with_id(api_root, &experiment_id)
    }

    pub fn search_with_id(api_root: &str, id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(format!("{api_root}/api/2.0/mlflow/experiments/get"))
            .json(&GetExperimentRequest {
                experiment_id: id.to_owned(),
            })
            .send()?
            .error_for_status()?
            .text()?;

        check_for_error_response(&response)?;

        match serde_json::from_str::<GetExperimentResponse>(&response) {
            Ok(r) => Ok(r.experiment),
            Err(e) => return Err(Box::new(e)),
        }
    }

    pub fn search_with_name(
        api_root: &str,
        name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(format!("{api_root}/api/2.0/mlflow/experiments/get-by-name"))
            .json(&GetExperimentByNameRequest {
                experiment_name: name.to_owned(),
            })
            .send()?
            .error_for_status()?
            .text()?;

        check_for_error_response(&response)?;

        match serde_json::from_str::<GetExperimentResponse>(&response) {
            Ok(r) => Ok(r.experiment),
            Err(e) => return Err(Box::new(e)),
        }
    }

    pub fn create_run(
        &self,
        api_root: &str,
        run_name: Option<&str>,
        tags: Vec<RunTag>,
    ) -> Result<Run, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{api_root}/api/2.0/mlflow/runs/create"))
            .json(&CreateRunRequest {
                experiment_id: self.experiment_id.clone(),
                run_name: run_name.map(|x| x.to_owned()),
                start_time: 0, // TODO
                tags,
            })
            .send()?
            .error_for_status()?
            .text()?;

        check_for_error_response(&response)?;

        let run = serde_json::from_str::<CreateRunResponse>(&response)?.run;

        Ok(run)
    }

    pub fn get_experiment_id(&self) -> &str {
        &self.experiment_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_artifact_location(&self) -> &str {
        &self.artifact_location
    }

    pub fn get_lifecycle_stage(&self) -> &str {
        &self.lifecycle_stage
    }

    pub fn get_last_update_time(&self) -> u64 {
        self.last_update_time
    }

    pub fn get_creating_time(&self) -> u64 {
        self.creation_time
    }
}
