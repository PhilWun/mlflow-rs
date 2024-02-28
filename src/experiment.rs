use std::{error::Error, fmt::Display, time::SystemTime};

use serde::Deserialize;

use crate::{
    git_utils::{create_diff, get_commit_hash, is_repo_clean},
    run::{Run, RunTag},
    schemas::{
        CreateExperimentRequest, CreateExperimentResponse, CreateRunRequest, CreateRunResponse,
        GetExperimentByNameRequest, GetExperimentRequest, GetExperimentResponse,
    },
    utils::{checked_get_request, checked_post_request},
};

#[derive(Debug)]
pub struct DirtyRepoError {}

impl Display for DirtyRepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The repository contains uncommited changes.")
    }
}

impl Error for DirtyRepoError {}

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
        let response: CreateExperimentResponse = checked_post_request(
            &format!("{api_root}/api/2.0/mlflow/experiments/create"),
            &CreateExperimentRequest {
                name: name.to_owned(),
                tags: vec![],
            },
        )?;

        Self::search_with_id(api_root, &response.experiment_id)
    }

    pub fn search_with_id(api_root: &str, id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let response: GetExperimentResponse = checked_get_request(
            &format!("{api_root}/api/2.0/mlflow/experiments/get"),
            &GetExperimentRequest {
                experiment_id: id.to_owned(),
            },
        )?;

        Ok(response.experiment)
    }

    pub fn search_with_name(
        api_root: &str,
        name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let response: GetExperimentResponse = checked_get_request(
            &format!("{api_root}/api/2.0/mlflow/experiments/get-by-name"),
            &GetExperimentByNameRequest {
                experiment_name: name.to_owned(),
            },
        )?;

        Ok(response.experiment)
    }

    fn create_run_unchecked(
        &self,
        api_root: &str,
        run_name: Option<&str>,
        mut tags: Vec<RunTag>,
    ) -> Result<Run, Box<dyn std::error::Error>> {
        tags.push(RunTag {
            key: "mlflow.source.git.commit".to_owned(),
            value: get_commit_hash()?,
        });

        let response: CreateRunResponse = checked_post_request(
            &format!("{api_root}/api/2.0/mlflow/runs/create"),
            &CreateRunRequest {
                experiment_id: self.experiment_id.clone(),
                run_name: run_name.map(|x| x.to_owned()),
                start_time: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_millis(),
                tags,
            },
        )?;

        let run = response.run;

        Ok(run)
    }

    pub fn create_run(
        &self,
        api_root: &str,
        run_name: Option<&str>,
        tags: Vec<RunTag>,
    ) -> Result<Run, Box<dyn std::error::Error>> {
        if !is_repo_clean()? {
            Err(DirtyRepoError {})?
        }

        self.create_run_unchecked(api_root, run_name, tags)
    }

    pub fn create_run_with_git_diff(
        &self,
        api_root: &str,
        run_name: Option<&str>,
        tags: Vec<RunTag>,
    ) -> Result<Run, Box<dyn std::error::Error>> {
        let run = self.create_run_unchecked(api_root, run_name, tags)?;

        if !is_repo_clean()? {
            let diff = create_diff()?;

            run.log_artifact_bytes(api_root, diff, "uncommitted.patch")?;
        }

        Ok(run)
    }

    // TODO: search run

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
