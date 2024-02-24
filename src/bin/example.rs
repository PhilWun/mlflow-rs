use std::error::Error;

use experiment_tracking::{experiment::Experiment, run::{RunTag, Status}};

#[allow(dead_code)]
fn test() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    // let _experiment = Experiment::new(api_root, "new3")?;
    let experiment = Experiment::search_with_name(api_root, "new")?;
    let mut run = experiment.create_run(
        api_root,
        Some("new run"),
        vec![RunTag {
            key: "test".to_owned(),
            value: "test".to_owned(),
        }],
    )?;
    
    // run.log_artifact(api_root, &Path::new("local_file.jpg"), "test.jpg")?;
    run.end_run(api_root, Status::Finished)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    test()?;

    Ok(())
}
