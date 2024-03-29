use std::{panic, path::Path, process::exit, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::SystemTime};

use log::Log;
use serde::{Deserialize, Serialize};

use crate::{logger::ExperimentLogger, schemas::{LogMetricRequest, LogMetricResponse, LogParameterRequest, LogParameterResponse, UpdateRunRequest, UpdateRunResponse}, utils::checked_post_request};

#[derive(Deserialize)]
pub struct Run {
    #[serde(skip)]
    api_root: String,
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub enum Status {
    Finished,
    Killed,
    Failed
}

impl Run {
    pub fn end_run(&mut self, status: Status) -> Result<(), Box<dyn std::error::Error>> {
        let new_run_info = checked_post_request::<UpdateRunRequest, UpdateRunResponse>(
            &format!("{}/api/2.0/mlflow/runs/update", self.api_root),
            &UpdateRunRequest {
                run_id: self.info.run_id.clone(),
                status,
                end_time: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_millis()
            },
        )?.run_info;

        self.info = new_run_info;

        Ok(())
    }

    pub fn log_metric(&self, key: &str, value: f32, step: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
        checked_post_request::<LogMetricRequest, LogMetricResponse>(
            &format!("{}/api/2.0/mlflow/runs/log-metric", self.api_root),
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

    pub fn log_parameter(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        checked_post_request::<LogParameterRequest, LogParameterResponse>(
            &format!("{}/api/2.0/mlflow/runs/log-parameter", self.api_root),
            &LogParameterRequest{
                run_id: self.info.run_id.clone(),
                key: key.to_owned(),
                value: value.to_owned()
            }
        )?;

        Ok(())
    }

    pub fn log_artifact_file(&self, path_on_disk: &Path, path_destination: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let file = std::fs::File::open(path_on_disk)?;

        client.post(format!("{}/ajax-api/2.0/mlflow/upload-artifact", self.api_root))
            .body(file)
            .query(&[("run_uuid", self.info.run_id.as_str()), ("path", path_destination)])
            .send()?
            .error_for_status()?;

        Ok(())
    }

    pub fn log_artifact_bytes(&self, data: Vec<u8>, path_destination: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        client.post(format!("{}/ajax-api/2.0/mlflow/upload-artifact", self.api_root))
            .body(data)
            .query(&[("run_uuid", self.info.run_id.as_str()), ("path", path_destination)])
            .send()?
            .error_for_status()?;

        Ok(())
    }

    pub fn log_logger<L: Log + 'static>(&self, logger: &ExperimentLogger<L>) -> Result<(), Box<dyn std::error::Error>> {
        self.log_artifact_bytes(logger.to_string().into_bytes(), "log.log")
    }

    pub fn run_experiment(&mut self, experiment_function: fn(&Run, Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>>) -> Result<(), Box<dyn std::error::Error>> {
        let was_killed = Arc::new(AtomicBool::new(false));
        let was_killed_clone = was_killed.clone();

        ctrlc::set_handler(move || {
            if was_killed_clone.load(Ordering::Relaxed) {
                println!();
                println!("The experiment will be forced to terminate. The status of the run will remain at UNFINISHED.");
                exit(1);
            } else {
                was_killed_clone.store(true, Ordering::Relaxed);
                println!();
                println!("The experiment was asked to terminate. If you want to force termination, press Ctrl+C again.");
            }
        })?;
        
        // catch panics (might not catch all panics, see Rust docs)
        let result = panic::catch_unwind(|| {
            experiment_function(&self, was_killed.clone())
        });

        let successful = match result {
            Ok(inner_result) => match inner_result {
                Ok(_) => true,
                Err(_) => false, // TODO: return error
            },
            Err(_) => false, // TODO: return error
        };

        if was_killed.load(Ordering::Relaxed) {
            self.end_run(Status::Killed)?;

            return Ok(());
        }

        if successful {
            self.end_run(Status::Finished)?;
        } else {
            self.end_run(Status::Failed)?;
        }

        Ok(())
    }

    pub fn run_experiment_with_logger<L: Log + 'static>(&mut self, experiment_function: fn(&Run, Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>>, logger: L) -> Result<(), Box<dyn std::error::Error>> {
        let experiment_logger = ExperimentLogger::init(logger)?;

        self.run_experiment(experiment_function)?;
        self.log_logger(experiment_logger)?;

        Ok(())
    }

    pub fn get_api_root(&self) -> &str {
        &self.api_root
    }

    pub fn set_api_root(&mut self, api_root: &str) {
        self.api_root = api_root.to_owned()
    }

    pub fn get_run_uuid(&self) -> &str {
        &self.info.run_uuid
    }

    pub fn get_experiment_id(&self) -> &str {
        &self.info.experiment_id
    }

    pub fn get_run_name(&self) -> &str {
        &self.info.run_name
    }

    pub fn get_user_id(&self) -> &str {
        &self.info.user_id
    }

    pub fn get_status(&self) -> &str {
        &self.info.status
    }

    pub fn get_start_time(&self) -> u64 {
        self.info.start_time
    }

    pub fn get_artifact_uri(&self) -> &str {
        &self.info.artifact_uri
    }

    pub fn get_lifecycle_stage(&self) -> &str {
        &self.info.lifecycle_stage
    }

    pub fn get_tags(&self) -> &Vec<RunTag> {
        &self.data.tags
    }
}
