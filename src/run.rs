use std::{path::Path, time::SystemTime};

use serde::{Deserialize, Serialize};

use crate::{schemas::{LogMetricRequest, LogMetricResponse, LogParameterRequest, LogParameterResponse, UpdateRunRequest, UpdateRunResponse}, utils::checked_post_request};

#[derive(Deserialize)]
pub struct Run {
    info: RunInfo,
    data: RunData,
}

#[derive(Deserialize)]
pub(crate) struct RunInfo {
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
    pub fn end_run_successfully(&self, api_root: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.end_run_with_status(api_root, "FINISHED")
    }

    pub fn end_run_forced(&self, api_root: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.end_run_with_status(api_root, "KILLED")
    }

    pub fn end_run_failed(&self, api_root: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.end_run_with_status(api_root, "FAILED")
    }

    fn end_run_with_status(&self, api_root: &str, status: &str) -> Result<(), Box<dyn std::error::Error>> {
        checked_post_request::<UpdateRunRequest, UpdateRunResponse>(
            &format!("{api_root}/api/2.0/mlflow/runs/update"),
            &UpdateRunRequest {
                run_id: self.info.run_id.clone(),
                status: status.to_owned(),
                end_time: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_millis()
            },
        )?;

        Ok(())
    }

    pub fn log_metric(&self, api_root: &str, key: &str, value: f32, step: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
        checked_post_request::<LogMetricRequest, LogMetricResponse>(
            &format!("{api_root}/api/2.0/mlflow/runs/log-metric"),
            &LogMetricRequest{
                run_id: self.info.run_id.clone(),
                key: key.to_owned(),
                value,
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_millis(),
                step
            }
        )?;

        Ok(())
    }

    pub fn log_parameter(&self, api_root: &str, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        checked_post_request::<LogParameterRequest, LogParameterResponse>(
            &format!("{api_root}/api/2.0/mlflow/runs/log-parameter"),
            &LogParameterRequest{
                run_id: self.info.run_id.clone(),
                key: key.to_owned(),
                value: value.to_owned()
            }
        )?;

        Ok(())
    }

    pub fn log_artifact(&self, api_root: &str, path_on_disk: &Path, path_destination: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let file = std::fs::File::open(path_on_disk)?;

        client.post(format!("{api_root}/ajax-api/2.0/mlflow/upload-artifact"))
            .body(file)
            .query(&[("run_uuid", self.info.run_id.as_str()), ("path", path_destination)])
            .send()?
            .error_for_status()?;

        Ok(())
    }
}
