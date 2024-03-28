# MLflow_rs

This is a client library for experiment tracking with [MLflow](https://mlflow.org/).
Improvements over the official Python library:
- uncommitted changes will be correctly handled to ensure reproducibility
- logs from [log](https://crates.io/crates/log) compatible loggers can be stored with the experiment results
- experiment code gets notified if the user wants to terminate the experiment which provides the opportunity to e.g. finish the current iteration / save the current state etc.
- compile time configuration `disable_experiment_tracking` disables experiment tracking and removes most of the code which should result in minimal overhead when experiment tracking needs to be disabled temporarily

## Usage

```toml
[dependencies]
mlflow_rs = "0.1"
```

```rust
use std::{error::Error, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::sleep, time::Duration};

use env_logger::Builder;
use log::{error, info};
use mlflow_rs::{experiment::Experiment, run::{Run, RunTag}};

/// Function that executes the experiment
fn experiment_function(run: &Run, was_killed: Arc<AtomicBool>) -> Result<(), Box<dyn Error>> {
    info!("info message");
    error!("error message");

    run.log_parameter("learning_rate", "0.001")?;
    run.log_metric("metric", 42.0, Some(0))?;
    run.log_artifact_bytes("test data".to_owned().into_bytes(), "test.txt")?;

    for _ in 0..10 {
        if was_killed.load(Ordering::Relaxed) {
            return Ok(())
        }

        sleep(Duration::from_secs(1));
    }

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

When you want to disable tracking temporarily:

Create the file `.cargo/config.toml` and add:
```toml
[build]
rustflags = ["--cfg", "disable_experiment_tracking"]
```

or run cargo with:

```shell
cargo rustc --lib -- --cfg disable_experiment_tracking
```

or set the RUSTFLAGS environment variable:

```shell
RUSTFLAGS="--cfg disable_experiment_tracking" cargo build --lib
```

other ways can be found here: https://doc.rust-lang.org/cargo/reference/config.html?highlight=rustflags#buildrustflags
