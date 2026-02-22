use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Cfg {
    pub workspaces: Vec<String>,
    pub presets: Option<HashMap<String, Preset>>,
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
    let mut path = dirs::config_dir().ok_or("OS config dir not found")?;
    path.push("sesh");
    fs::create_dir_all(&path)?;
    path.push("config.toml");
    if !path.exists() {
        fs::write(&path, "workspaces = []")?;
    }
    let content = fs::read_to_string(path)?;
    let cfg: Cfg = toml::from_str(&content)?;
    Ok(cfg)
}
