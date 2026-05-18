use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs};

use crate::utils::get_config_file_path;

#[derive(Debug, Deserialize)]
pub struct Cfg {
    #[serde(default)]
    pub workspaces: Vec<String>,
    #[serde(default)]
    pub bookmarks: Vec<String>,
    #[serde(default)]
    pub presets: Option<HashMap<String, Preset>>,
}

impl Cfg {
    pub fn get_preset(&self, name: &str) -> Option<&Preset> {
        self.presets.as_ref()?.get(name)
    }
}

#[derive(Debug, Deserialize)]
pub struct Preset {
    pub windows: Vec<Window>,
}

#[derive(Debug, Deserialize)]
pub struct Window {
    pub name: Option<String>,
    #[serde(default)]
    pub cmd: String,
}

pub fn load() -> Result<Cfg, Box<dyn Error>> {
    let mut path = get_config_file_path("config.toml")?;

    // try using config in project root folder, only in debug builds
    #[cfg(debug_assertions)]
    {
        let test_path = std::path::PathBuf::from("config.toml");
        if test_path.exists() {
            eprintln!("using config in project root folder");
            path = test_path;
        }
    }

    if !path.exists() {
        fs::write(&path, "workspaces = []\nbookmarks = []\n")?;
    }
    let content = fs::read_to_string(&path)?;
    let cfg: Cfg =
        toml::from_str(&content).map_err(|e| format!("parsing {}: {e}", path.display()))?;
    Ok(cfg)
}
