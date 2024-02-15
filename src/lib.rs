use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Experiment {
    experiment_id: String,
    name: String,
    artifact_location: String,
    lifecycle_stage: String,
    last_update_time: u64,
    creation_time: u64,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error_code: String,
    message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("error code: {}\n", self.error_code))?;
        f.write_str(&format!("message: {}", self.message))?;
        Ok(())
    }
}

impl std::error::Error for ErrorResponse {}

#[derive(Serialize)]
struct CreateExperimentRequest {
    name: String,
    tags: Vec<String>,
}

#[derive(Deserialize)]
struct CreateExperimentResponse {
    experiment_id: String,
}

#[derive(Serialize)]
struct GetExperimentRequest {
    experiment_id: String,
}

#[derive(Serialize)]
struct GetExperimentByNameRequest {
    experiment_name: String,
}

#[derive(Deserialize)]
struct GetExperimentResponse {
    experiment: Experiment,
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
            .text()?;

        match serde_json::from_str::<ErrorResponse>(&response) {
            Ok(e) => return Err(Box::new(e)),
            Err(_) => (),
        }

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
            .text()?;

        match serde_json::from_str::<ErrorResponse>(&response) {
            Ok(e) => return Err(Box::new(e)),
            Err(_) => (),
        }

        match serde_json::from_str::<GetExperimentResponse>(&response) {
            Ok(r) => Ok(r.experiment),
            Err(e) => return Err(Box::new(e)),
        }
    }

    pub fn search_with_name(api_root: &str, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(format!("{api_root}/api/2.0/mlflow/experiments/get-by-name"))
            .json(&GetExperimentByNameRequest {
                experiment_name: name.to_owned(),
            })
            .send()?
            .text()?;

        match serde_json::from_str::<ErrorResponse>(&response) {
            Ok(e) => return Err(Box::new(e)),
            Err(_) => (),
        }

        match serde_json::from_str::<GetExperimentResponse>(&response) {
            Ok(r) => Ok(r.experiment),
            Err(e) => return Err(Box::new(e)),
        }
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
