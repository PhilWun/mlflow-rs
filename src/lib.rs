#![cfg_attr(disable_experiment_tracking, allow(unused))] // disables warning about unused code when experiment tracking is disabled

pub mod experiment;
mod git_utils;
pub mod logger;
pub mod run;
mod schemas;
mod utils;

// TODO: add log function to log serde serializable structs as parameters
