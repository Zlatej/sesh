use std::{
    error::Error,
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
    match home.strip_prefix(&home){
        Ok(relative) => Ok(format!("~/{}", relative.display())),
        Err(_) => Ok(path.display().to_string()),
            
    }
}
