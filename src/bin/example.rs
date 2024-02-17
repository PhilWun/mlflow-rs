use std::{error::Error, thread::sleep, time::Duration};

use experiment_tracking::{experiment::Experiment, run::RunTag};

fn main() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    // let _experiment = Experiment::new(api_root, "new3")?;
    let experiment = Experiment::search_with_name(api_root, "new")?;
    let run = experiment.create_run(
        api_root,
        Some("new run"),
        vec![RunTag {
            key: "test".to_owned(),
            value: "test".to_owned(),
        }],
    )?;
    
    run.log_metric(api_root, "test", 42.0, Some(1))?;

    Ok(())
}
