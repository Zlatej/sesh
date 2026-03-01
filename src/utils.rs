use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn expand_tilde(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    if let Some(path) = path.strip_prefix("~/") {
        let mut home = dirs::home_dir().expect("wdym you dont have a home path");
        home.push(path);
        Ok(home)
    } else if path == "~" {
        Ok(dirs::home_dir().expect("wdym you dont have a home path"))
    } else {
        Ok(PathBuf::from(path))
    }
}

pub fn to_tilde_path(path: &Path) -> Result<String, Box<dyn Error>> {
    let home = dirs::home_dir().expect("wdym you dont have a home path");
    match path.strip_prefix(&home) {
        Ok(relative) => Ok(format!("~/{}", relative.display())),
        Err(_) => Ok(path.display().to_string()),
    }
}

pub fn get_config_file_path(filename: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut path = dirs::config_dir().ok_or("OS config dir not found")?;
    path.push("sesh");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    path.push(filename);
    Ok(path)
}

pub fn get_project_name(path: &str) -> Result<&str, Box<dyn Error>> {
    path.split('/')
        .next_back()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Failed to extract project name from path".into())
}
