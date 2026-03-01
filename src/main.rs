use clap::Subcommand;
use std::error::Error;

use clap::Parser;

use crate::{cache::Cache, tmux::launch};

mod cache;
mod config;
mod scanner;
mod tmux;
mod ui;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "sesh", about = "tmux session launcher")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Manage,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Manage) => todo!(),
        None => launch_project()?,
    }
    Ok(())
}

fn launch_project() -> Result<(), Box<dyn Error>> {
    let cfg = config::load()?;
    let mut cache = Cache::load()?;
    let projects = scanner::scan(&cfg.workspaces)?;
    let Some(project) = ui::pick_project(&projects)? else {
        return Ok(());
    };
    let preset_name = match cache.get_preset(&project) {
        Some(name) => name.to_owned(),
        None => {
            let names = cfg
                .presets
                .as_ref()
                .map(|p| p.keys().cloned().collect())
                .unwrap_or_default();
            let Some(picked) = ui::pick_preset(&names)? else {
                return Ok(());
            };
            cache.set_preset(project.clone(), picked.clone());
            picked
        }
    };
    let preset = cfg.get_preset(&preset_name).ok_or("preset not found")?;
    cache.save()?;
    launch(&project, preset)?;
    Ok(())
}
