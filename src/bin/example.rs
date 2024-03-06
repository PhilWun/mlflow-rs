use std::{error::Error, path::Path};

use experiment_tracking::{
    experiment::Experiment, logger::ExperimentLogger, run::{RunTag, Status}
};
use log::{error, info, Log};

#[allow(dead_code)]
fn create_experiment() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::new(api_root, "test")?;

    println!("Created experiment {}", experiment.get_name());

    Ok(())
}

#[allow(dead_code)]
fn create_run_without_git_diff() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    experiment.create_run(api_root, Some("new run"), vec![])?;

    Ok(())
}

#[allow(dead_code)]
fn create_run_with_git_diff() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    experiment.create_run_with_git_diff(api_root, Some("new run"), vec![])?;

    Ok(())
}

#[allow(dead_code)]
fn create_run_with_tags() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    experiment.create_run_with_git_diff(
        api_root,
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

    let mut run = experiment.create_run_with_git_diff(
        api_root,
        Some("new run"),
        vec![],
    )?;

    run.end_run(api_root, Status::Finished)?;

    Ok(())
}

#[allow(dead_code)]
fn log_metrics() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(
        api_root,
        Some("new run"),
        vec![],
    )?;

    run.log_metric(api_root, "mse", 1.4, Some(0))?;
    run.log_metric(api_root, "mse", 1.2, Some(1))?;
    run.log_metric(api_root, "mse", 0.9, Some(2))?;

    Ok(())
}

#[allow(dead_code)]
fn log_params() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(
        api_root,
        Some("new run"),
        vec![],
    )?;

    run.log_parameter(api_root, "param1", "value1")?;

    Ok(())
}

#[allow(dead_code)]
fn log_file() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(
        api_root,
        Some("new run"),
        vec![],
    )?;

    run.log_artifact_file(api_root, Path::new(".gitignore"), ".gitignore")?;

    Ok(())
}

#[allow(dead_code)]
fn log_bytes() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(
        api_root,
        Some("new run"),
        vec![],
    )?;

    run.log_artifact_bytes(api_root, "test data".to_owned().into_bytes(), "test.txt")?;

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

    fn flush(&self) {
        
    }
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

    let run = experiment.create_run_with_git_diff(
        api_root,
        Some("new run"),
        vec![],
    )?;

    info!("info message");
    error!("error message");

    run.log_logger(api_root, logger)?;

    Ok(())
}

fn experiment_function() -> Result<(), Box<dyn Error>> {
    info!("info message");
    error!("error message");

    u32::from_str_radix("a", 10).unwrap();

    Ok(())
}

#[allow(dead_code)]
fn run_experiment() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let mut run = experiment.create_run_with_git_diff(
        api_root,
        Some("new run"),
        vec![],
    )?;

    run.run_experiment(api_root, experiment_function)?;

    Ok(())
}

#[allow(dead_code)]
fn run_experiment_with_logger() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let logger = TestLogger {};
    log::set_max_level(log::LevelFilter::Info);

    let mut run = experiment.create_run_with_git_diff(
        api_root,
        Some("new run"),
        vec![],
    )?;

    run.run_experiment_with_logger(api_root, experiment_function, logger)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run_experiment_with_logger()?;

    Ok(())
}
