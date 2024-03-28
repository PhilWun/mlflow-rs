use std::{error::Error, path::Path, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::sleep, time::Duration};

use log::{error, info, Log};
use mlflow_rs::{
    experiment::Experiment,
    logger::ExperimentLogger,
    run::{Run, RunTag, Status},
};

#[allow(dead_code)]
fn create_experiment() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::new(api_root, "test")?;

    println!("Created experiment {}", experiment.get_name());

    Ok(())
}

#[allow(dead_code)]
fn search_experiment_with_id() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_id(api_root, "1")?;

    println!("Got experiment {}", experiment.get_name());

    Ok(())
}

#[allow(dead_code)]
fn create_run_without_git_diff() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    experiment.create_run(Some("new run"), vec![])?;

    Ok(())
}

#[allow(dead_code)]
fn create_run_with_git_diff() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    Ok(())
}

#[allow(dead_code)]
fn create_run_with_tags() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    experiment.create_run_with_git_diff(
        Some("new run"),
        vec![RunTag {
            key: "key".to_owned(),
            value: "value".to_owned(),
        }],
    )?;

    Ok(())
}

#[allow(dead_code)]
fn end_run() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.end_run(Status::Finished)?;

    Ok(())
}

#[allow(dead_code)]
fn log_metrics() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.log_metric("mse", 1.4, Some(0))?;
    run.log_metric("mse", 1.2, Some(1))?;
    run.log_metric("mse", 0.9, Some(2))?;

    Ok(())
}

#[allow(dead_code)]
fn log_params() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.log_parameter("param1", "value1")?;

    Ok(())
}

#[allow(dead_code)]
fn log_file() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.log_artifact_file(Path::new(".gitignore"), ".gitignore")?;

    Ok(())
}

#[allow(dead_code)]
fn log_bytes() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.log_artifact_bytes("test data".to_owned().into_bytes(), "test.txt")?;

    Ok(())
}

struct TestLogger {}

impl Log for TestLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        println!("{}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

#[allow(dead_code)]
fn experiment_logger() -> Result<(), Box<dyn Error>> {
    let logger = ExperimentLogger::init(TestLogger {})?;

    log::set_max_level(log::LevelFilter::Trace);

    info!("info message");
    error!("error message");

    println!("{}", logger.to_string());

    Ok(())
}

#[allow(dead_code)]
fn log_logger() -> Result<(), Box<dyn Error>> {
    let logger = ExperimentLogger::init(TestLogger {})?;

    log::set_max_level(log::LevelFilter::Trace);

    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    info!("info message");
    error!("error message");

    run.log_logger(logger)?;

    Ok(())
}

fn experiment_function(run: &Run, _: Arc<AtomicBool>) -> Result<(), Box<dyn Error>> {
    info!("info message");
    error!("error message");

    run.log_parameter("learning_rate", "0.001")?;
    run.log_metric("metric", 42.0, Some(0))?;
    run.log_artifact_bytes("test data".to_owned().into_bytes(), "test.txt")?;

    u32::from_str_radix("a", 10).unwrap();  // panics, to show that panics get caught and handled

    Ok(())
}

#[allow(dead_code)]
fn run_experiment() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.run_experiment(experiment_function)?;

    Ok(())
}

#[allow(dead_code)]
fn run_experiment_with_logger() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let logger = TestLogger {};
    log::set_max_level(log::LevelFilter::Info);

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.run_experiment_with_logger(experiment_function, logger)?;

    Ok(())
}

#[allow(dead_code)]
fn ctrl_c_handler() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.run_experiment(|_, was_killed| {
        println!("running");
        
        while !was_killed.load(Ordering::Relaxed) {
            
        }

        Ok(())
    })?;

    Ok(())
}

#[allow(dead_code)]
fn ctrl_c_handler_ignore_signal() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.run_experiment(|_, _| {
        println!("running");
        
        sleep(Duration::from_secs(10));

        Ok(())
    })?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    ctrl_c_handler()?;

    Ok(())
}
