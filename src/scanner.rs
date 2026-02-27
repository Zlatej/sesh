use std::error::Error;

use dirs::home_dir;

pub fn scan_dirs(workspaces: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
    let home = home_dir();
    let mut projects = Vec::new();

    for ws in workspaces {
        let abs_path = 
    }

    Ok(projects)
}
