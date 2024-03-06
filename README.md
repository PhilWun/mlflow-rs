# MLflow_rs

This is a client library for experiment tracking with [MLflow](https://mlflow.org/).
Improvements over the official Python library:
- uncommitted changes will be correctly handled to ensure reproducibility
- logs from [log](https://crates.io/crates/log) compatible loggers can be stored with the experiment results

## Usage

```toml
[dependencies]
mlflow_rs = "0.1"
```

```rust
use std::error::Error;

use env_logger::Builder;
use log::{error, info};
use mlflow_rs::{experiment::Experiment, run::{Run, RunTag}};

/// Function that executes the experiment
fn experiment_function(run: &Run) -> Result<(), Box<dyn Error>> {
    info!("info message");
    error!("error message");

    run.log_parameter("learning_rate", "0.001")?;
    run.log_metric("metric", 42.0, Some(0))?;
    run.log_artifact_bytes("test data".to_owned().into_bytes(), "test.txt")?;

    u32::from_str_radix("a", 10).unwrap();  // panics, to show that panics get caught and handled

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let experiment = Experiment::new("http://localhost:5000", "test")?;

    let mut logger_builder = Builder::from_default_env();
    let logger = logger_builder.build();

    let mut run = experiment.create_run_with_git_diff(
        Some("new run"),
        vec![RunTag {
            key: "tag_name".to_owned(),
            value: "tag_value".to_owned(),
        }],
    )?;

    run.run_experiment_with_logger(experiment_function, logger)?;

    Ok(())
}
```
