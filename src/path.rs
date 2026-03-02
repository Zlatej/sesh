use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::utils::{expand_tilde, to_tilde_path};

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
pub struct ProjectPath {
    pub tilde: String,
    pub real: PathBuf,
    pub sesh_name: String,
}

impl ProjectPath {
    pub fn from_relative(path: &str) -> Result<Self, Box<dyn Error>> {
        let real = expand_tilde(path)?;
        let sesh_name = tmuxize_name(&real)?;
        Ok(Self {
            tilde: path.to_string(),
            real,
            sesh_name,
        })
    }

    pub fn from_path(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let tilde = to_tilde_path(&path)?;
        let sesh_name = tmuxize_name(&path)?;
        Ok(Self {
            tilde,
            real: path,
            sesh_name,
        })
    }
}

fn tmuxize_name(path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(last_component(path)?.replace(".", "_"))
}

fn last_component(path: &Path) -> Result<String, Box<dyn Error>> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(str::to_owned)
        .ok_or_else(|| format!("failed to derive name from path: {}", path.display()).into())
}
