use std::{error::Error, fmt::Display, time::SystemTime};

use log::error;
use serde::Deserialize;

use crate::{
    git_utils::{create_diff, does_repo_contain_subfolders_with_repos, does_repo_contain_submodules, get_commit_hash, is_repo_clean},
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
        write!(f, "The repository contains uncommitted changes.")
    }
}

impl Error for DirtyRepoError {}

#[derive(Debug)]
pub struct RepoContainsSubmodulesError {}

impl Display for RepoContainsSubmodulesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The repository contains one or more submodules which is currently not supported.")
    }
}

impl Error for RepoContainsSubmodulesError {}

#[derive(Debug)]
pub struct RepoContainsSubfolderReposError {}

impl Display for RepoContainsSubfolderReposError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The repository contains one or more subfolder which themselves container git repos which is currently not supported.")
    }
}

impl Error for RepoContainsSubfolderReposError {}

#[derive(Deserialize)]
pub struct Experiment {
    #[serde(skip)]
    api_root: String,
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
        ).map_err(|err| {
            error!("an experiment with the name {} might exist already or still exists in a deleted state.", name);
            err
    })?;

        Self::search_with_id(api_root, &response.experiment_id)
    }

    pub fn search_with_id(api_root: &str, id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let response: GetExperimentResponse = checked_get_request(
            &format!("{api_root}/api/2.0/mlflow/experiments/get"),
            &GetExperimentRequest {
                experiment_id: id.to_owned(),
            },
        )?;

        let mut experiment = response.experiment;
        experiment.api_root = api_root.to_owned();

        Ok(experiment)
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

        let mut experiment = response.experiment;
        experiment.api_root = api_root.to_owned();

        Ok(experiment)
    }

    fn create_run_unchecked(
        &self,
        run_name: Option<&str>,
        mut tags: Vec<RunTag>,
    ) -> Result<Run, Box<dyn std::error::Error>> {
        tags.push(RunTag {
            key: "mlflow.source.git.commit".to_owned(),
            value: get_commit_hash()?,
        });

        // TODO: log which binary was executed

        let response: CreateRunResponse = checked_post_request(
            &format!("{}/api/2.0/mlflow/runs/create", self.api_root),
            &CreateRunRequest {
                experiment_id: self.experiment_id.clone(),
                run_name: run_name.map(|x| x.to_owned()),
                start_time: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_millis(),
                tags,
            },
        )?;

        let mut run = response.run;
        run.set_api_root(&self.api_root);

        Ok(run)
    }

    pub fn create_run(
        &self,
        run_name: Option<&str>,
        tags: Vec<RunTag>,
    ) -> Result<Run, Box<dyn std::error::Error>> {
        if does_repo_contain_submodules()? {
            Err(RepoContainsSubmodulesError {})?
        }

        if does_repo_contain_subfolders_with_repos()? {
            Err(RepoContainsSubfolderReposError {})?
        }

        if !is_repo_clean()? {
            Err(DirtyRepoError {})?
        }

        self.create_run_unchecked(run_name, tags)
    }

    pub fn create_run_with_git_diff(
        &self,
        run_name: Option<&str>,
        tags: Vec<RunTag>,
    ) -> Result<Run, Box<dyn std::error::Error>> {
        if does_repo_contain_submodules()? {
            Err(RepoContainsSubmodulesError {})?
        }

        if does_repo_contain_subfolders_with_repos()? {
            Err(RepoContainsSubfolderReposError {})?
        }

        let run = self.create_run_unchecked(run_name, tags)?;

        if !is_repo_clean()? {
            let diff = create_diff()?;

            run.log_artifact_bytes(diff, "uncommitted.patch")?;
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
