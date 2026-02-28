use std::{error::Error, fs};

use crate::utils::{self, to_tilde_path};

pub fn scan(workspaces: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
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

            projects.push(to_tilde_path(&path)?);
        }
    }

    projects.sort();
    Ok(projects)
}
