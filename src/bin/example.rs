use std::{error::Error, path::Path};

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
    
    run.log_artifact(api_root, &Path::new("local_file.jpg"), "test.jpg")?;

    Ok(())
}
