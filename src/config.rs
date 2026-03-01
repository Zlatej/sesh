use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs};

use crate::utils::get_config_file_path;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Cfg {
    pub workspaces: Vec<String>,
    pub presets: Option<HashMap<String, Preset>>,
}

impl Cfg {
    pub fn get_preset(&self, name: &str) -> Option<&Preset> {
        self.presets.as_ref()?.get(name)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Preset {
    pub windows: Vec<Window>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Window {
    pub name: Option<String>,
    pub cmd: String,
}

pub fn load() -> Result<Cfg, Box<dyn Error>> {
    let path = get_config_file_path("config.toml")?;
    if !path.exists() {
        fs::write(&path, "workspaces = []")?;
    }
    let content = fs::read_to_string(path)?;
    let cfg: Cfg = toml::from_str(&content)?;
    Ok(cfg)
}
