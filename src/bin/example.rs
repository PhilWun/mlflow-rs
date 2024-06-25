use std::{
    error::Error,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use log::{error, info, Log};
use mlflow_rs::{
    experiment::Experiment,
    logger::ExperimentLogger,
    run::{Run, RunTag, Status},
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
struct ParamStruct {
    a: String,
    b: u32,
    c: Vec<u32>,
    d: InnerStruct,
}

#[derive(Serialize, Deserialize, Debug)]
struct InnerStruct {
    e: u32,
    f: String,
}

#[allow(dead_code)]
fn log_struct_params() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.log_parameter_struct_as_json(ParamStruct {
        a: "test".to_owned(),
        b: 42,
        c: vec![1, 2, 3, 4, 5],
        d: InnerStruct {
            e: 43,
            f: "test2".to_owned(),
        },
    })?;

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

#[allow(dead_code)]
fn log_struct_as_json() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;
    let run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    let data = ParamStruct {
        a: "test".to_owned(),
        b: 42,
        c: vec![1, 2, 3, 4, 5],
        d: InnerStruct {
            e: 43,
            f: "test2".to_owned(),
        },
    };

    run.log_artifact_struct_as_json(data, "test.json")?;

    Ok(())
}

#[allow(dead_code)]
fn log_struct_as_binary() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;
    let run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    let data = ParamStruct {
        a: "test".to_owned(),
        b: 42,
        c: vec![1, 2, 3, 4, 5],
        d: InnerStruct {
            e: 43,
            f: "test2".to_owned(),
        },
    };

    run.log_artifact_struct_as_binary(data, "test.bin")?;

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

fn experiment_function(run: &Run, _: Arc<AtomicBool>, _: ()) -> Result<(), Box<dyn Error>> {
    info!("info message");
    error!("error message");

    run.log_parameter("learning_rate", "0.001")?;
    run.log_metric("metric", 42.0, Some(0))?;
    run.log_artifact_bytes("test data".to_owned().into_bytes(), "test.txt")?;

    u32::from_str_radix("a", 10).unwrap(); // panics, to show that panics get caught and handled

    Ok(())
}

#[allow(dead_code)]
fn run_experiment() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.run_experiment(experiment_function, ())?;

    Ok(())
}

#[allow(dead_code)]
fn run_experiment_with_logger() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let logger = TestLogger {};
    log::set_max_level(log::LevelFilter::Info);

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.run_experiment_with_logger(experiment_function, (), logger)?;

    Ok(())
}

#[allow(dead_code)]
fn ctrl_c_handler() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.run_experiment(|_, was_killed, _| {
        println!("running");

        while !was_killed.load(Ordering::Relaxed) {}

        Ok(())
    }, ())?;

    Ok(())
}

#[allow(dead_code)]
fn ctrl_c_handler_ignore_signal() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let experiment = Experiment::search_with_name(api_root, "test")?;

    let mut run = experiment.create_run_with_git_diff(Some("new run"), vec![])?;

    run.run_experiment(|_, _, _| {
        println!("running");

        sleep(Duration::from_secs(10));

        Ok(())
    }, ())?;

    Ok(())
}

#[allow(dead_code)]
fn get_run() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let run = Run::get_run(api_root, "e089ac98e5bf46bd9c952b50a6c27889")?;

    println!("{}", run.get_run_name());

    Ok(())
}

#[allow(dead_code)]
fn get_artifact_as_bytes() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let run = Run::get_run(api_root, "f11fa50bbfa0412cbabece559d9a499b")?;

    let log = String::from_utf8(run.get_artifact_as_bytes("log.log")?)?;

    println!("{}", log);

    Ok(())
}

#[allow(dead_code)]
fn get_artifact_as_string() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let run = Run::get_run(api_root, "f11fa50bbfa0412cbabece559d9a499b")?;

    let log = run.get_artifact_as_string("log.log")?;

    println!("{}", log);

    Ok(())
}

#[allow(dead_code)]
fn get_artifact_json_as_struct() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let run = Run::get_run(api_root, "7c758834d40e4226926084560f21aadc")?;

    let data: ParamStruct = run.get_artifact_json_as_struct("test.json")?;

    println!("{:?}", data);

    Ok(())
}

#[allow(dead_code)]
fn get_artifact_binary_as_struct() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    let run = Run::get_run(api_root, "7c758834d40e4226926084560f21aadc")?;

    let data: ParamStruct = run.get_artifact_binary_as_struct("test.bin")?;

    println!("{:?}", data);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    get_run()?;

    Ok(())
}
