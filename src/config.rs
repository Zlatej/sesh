use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs};

use crate::utils::get_config_file_path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    #[serde(default)]
    pub workspaces: Vec<String>,
    #[serde(default)]
    pub bookmarks: Vec<String>,
    #[serde(default)]
    pub presets: Option<HashMap<String, Preset>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    pub windows: Vec<Window>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Window {
    pub name: Option<String>,
    #[serde(default)]
    pub cmd: String,
}

impl Default for Cfg {
    fn default() -> Self {
        let mut presets = HashMap::new();
        presets.insert(
            "default".to_string(),
            Preset {
                windows: vec![
                    Window { name: None, cmd: String::new() },
                    Window { name: None, cmd: String::new() },
                ],
            },
        );
        Self {
            workspaces: vec!["~/repos".to_string()],
            bookmarks: Vec::new(),
            presets: Some(presets),
        }
    }
}

impl Cfg {
    pub fn get_preset_windows(&self, name: &str) -> Option<&[Window]> {
        Some(self.presets.as_ref()?.get(name)?.windows.as_slice())
    }
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
        } else {
            let cfg = Cfg::default();
            let toml_str = toml::to_string_pretty(&cfg)
                .map_err(|e| format!("test config serialization: {e}"))?;
            fs::write(&test_path, toml_str)?;
        }
    }

    if !path.exists() {
        let cfg = Cfg::default();
        let toml_str = toml::to_string_pretty(&cfg)
            .map_err(|e| format!("default config serialization: {e}"))?;
        fs::write(&path, toml_str)?;
        let display_path = path.display();
        println!("Created new config on path {display_path}");
    }
    let content = fs::read_to_string(&path)?;
    let cfg: Cfg =
        toml::from_str(&content).map_err(|e| format!("parsing {}: {e}", path.display()))?;
    Ok(cfg)
}
