use std::{error::Error, process::Command};

pub(crate) fn get_commit_hash() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;
    let converted = String::from_utf8(output.stdout)?.trim().to_owned();

    Ok(converted)
}

pub(crate) fn is_repo_clean() -> Result<bool, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;
    let converted = String::from_utf8(output.stdout)?.trim().to_owned();

    let is_clean = converted == "";

    Ok(is_clean)
}

pub(crate) fn create_diff() -> Result<Vec<u8>, Box<dyn Error>> {
    // Create a stash including untracked files
    Command::new("git")
        .arg("stash")
        .arg("push")
        .arg("-u")
        .output()?;

    // Create diff from stash
    let diff = Command::new("git")
        .arg("stash")
        .arg("show")
        .arg("-u")
        .arg("-p")
        .output();

    // Pop stash to recreate the exact state including staged files and remove the stash
    Command::new("git")
        .arg("stash")
        .arg("pop")
        .arg("--index")
        .output()?;

    Ok(diff?.stdout)
}

pub(crate) fn does_repo_contain_submodules() -> Result<bool, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("submodule")
        .arg("status")
        .arg("--recursive")
        .output()?;

    let converted = String::from_utf8(output.stdout)?.trim().to_owned();

    Ok(converted != "")
}

fn get_repo_root() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    let converted = String::from_utf8(output.stdout)?.trim().to_owned();

    Ok(converted)
}

pub(crate) fn does_repo_contain_subfolders_with_repos() -> Result<bool, Box<dyn Error>> {
    let root_path = get_repo_root()?;

    let output = Command::new("find")
        .arg(root_path)
        .arg("-name")
        .arg(".git")
        .output()?;

    let converted = String::from_utf8(output.stdout)?.trim().to_owned();
    let git_repo_paths: Vec<&str> = converted.lines().collect();

    Ok(git_repo_paths.len() > 1)
}
