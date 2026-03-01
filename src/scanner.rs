use std::{error::Error, fs};

use crate::{
    path::ProjectPath,
    utils::{self},
};

pub fn scan(workspaces: &[String]) -> Result<Vec<ProjectPath>, Box<dyn Error>> {
    let mut projects = Vec::new();

    for ws in workspaces {
        let path = utils::expand_tilde(ws)?;
        if !path.exists() || path.is_file() {
            continue;
        }
        for item in fs::read_dir(path)? {
            let path = (item?).path();
            if path.is_file() {
                continue;
            }

            projects.push(ProjectPath::from_path(path)?);
        }
    }

    projects.sort();
    Ok(projects)
}
