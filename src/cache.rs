use std::{collections::HashMap, error::Error, fs};

use serde::{Deserialize, Serialize};

use crate::utils::get_config_file_path;

#[derive(Debug, Deserialize)]
struct ProjectsFile {
    projects: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    projects: HashMap<String, String>,
    #[serde(skip)]
    dirty: bool,
}

impl Drop for Cache {
    fn drop(&mut self) {
        let _ = self.save();
    }
}

impl Cache {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = get_config_file_path("projects.toml")?;
        if !path.exists() {
            fs::write(&path, "[projects]\n")?;
        }
        let content = fs::read_to_string(path)?;
        let file: ProjectsFile = toml::from_str(&content)?;
        Ok(Cache {
            projects: file.projects,
            dirty: false,
        })
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.dirty {
            return Ok(());
        }
        let path = get_config_file_path("projects.toml")?;
        if !path.exists() {
            fs::write(&path, "[projects]\n")?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        self.dirty = false;

        Ok(())
    }

    pub fn get_preset(&mut self, project: &str) -> Option<&str> {
        self.projects.get(project).map(|x| x.as_str())
    }

    pub fn set_preset(&mut self, project: String, preset: String) {
        self.projects.insert(project, preset);
        self.dirty = true;
    }

    pub fn remove_project(&mut self, project: &str) -> bool {
        let removed = self.projects.remove(project).is_some();
        if removed {
            self.dirty = true;
        }
        removed
    }
}
