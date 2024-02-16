use std::error::Error;

use experiment_tracking::Experiment;

fn main() -> Result<(), Box<dyn Error>> {
    let api_root = "http://localhost:5000";
    // let _experiment = Experiment::new(api_root, "new3")?;
    let experiment = Experiment::search_with_name(api_root, "new")?;
    let _run = experiment.create_run(api_root, Some("new run"), vec![])?;
    
    Ok(())
}
