use std::{error::Error, fs};

use crate::{
    config::Cfg,
    path::ProjectPath,
    utils::{self},
};

pub fn scan(cfg: &Cfg) -> Result<Vec<ProjectPath>, Box<dyn Error>> {
    let mut projects = Vec::new();

    for ws in cfg.workspaces.clone() {
        let path = utils::expand_tilde(&ws)?;
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

    for bm in cfg.bookmarks.clone() {
        let Ok(path) = ProjectPath::from_relative(&bm) else {
            continue;
        };
        projects.push(path);
    }

    projects.sort();
    Ok(projects)
}
