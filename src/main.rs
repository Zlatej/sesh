use std::error::Error;

use clap::Parser;

use crate::{cache::Cache, tmux::launch};

mod cache;
mod config;
mod path;
mod scanner;
mod tmux;
mod ui;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "sesh", about = "tmux session launcher", version)]
struct Cli;

fn main() -> Result<(), Box<dyn Error>> {
    let _cli = Cli::parse();
    launch_project()?;
    Ok(())
}

fn launch_project() -> Result<(), Box<dyn Error>> {
    let cfg = config::load()?;
    let mut cache = Cache::load()?;
    let projects = scanner::scan(&cfg)?;
    let Some(project) = ui::pick_project(&projects)? else {
        return Ok(());
    };
    let has_presets = cfg.presets.as_ref().map(|p| !p.is_empty()).unwrap_or(false);

    let preset = if has_presets {
        let preset_name = match cache.get_preset(&project.tilde) {
            Some(name) => name.to_owned(),
            None => {
                let names: Vec<String> = cfg.presets.as_ref().unwrap().keys().cloned().collect();
                let Some(picked) = ui::pick_preset(&names)? else {
                    return Ok(());
                };
                cache.set_preset(project.tilde.clone(), picked.clone());
                picked
            }
        };
        Some(cfg.get_preset(&preset_name).ok_or("preset not found")?)
    } else {
        None
    };

    cache.save()?;
    launch(project, preset)?;
    Ok(())
}
