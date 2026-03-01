use std::{error::Error, os::unix::process::CommandExt, process::Command};

use crate::{config::Preset, utils::get_project_name};

/// launch prepares tmux session and then attaches with exec.
/// This function will never return or return error, it basically "exits" to tmux, meaning no
/// destructors will be called.
pub fn launch(project: &str, preset: &Preset) -> Result<(), Box<dyn Error>> {
    let sesh_name = get_project_name(project)?;
    let exists = Command::new("tmux")
        .args(["has-session", "-t", sesh_name])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !exists {
        run_tmux_cmd(&["new-session", "-d", "-s", sesh_name, "-c", project])?;
        setup_windows(sesh_name, project, preset)?;
    }

    Err(Command::new("tmux")
        .args(["attach-session", "-t", sesh_name])
        .exec()
        .into())
}

fn setup_windows(sesh_name: &str, project: &str, preset: &Preset) -> Result<(), Box<dyn Error>> {
    let mut first = true;

    for w in &preset.windows {
        if first {
            first = false;
            if let Some(win_name) = &w.name {
                run_tmux_cmd(&["rename-window", "-t", &format!("{}:", sesh_name), win_name])?;
            }
        } else {
            let mut args = vec!["new-window", "-t", sesh_name, "-c", project];
            if let Some(name) = &w.name {
                args.extend(["-n", name.as_str()]);
            }
            run_tmux_cmd(&args)?;
        }

        if !w.cmd.is_empty() {
            run_tmux_cmd(&["send-keys", "-t", sesh_name, &w.cmd, "Enter"])?;
        }
    }

    Ok(())
}

fn run_tmux_cmd(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let ok = Command::new("tmux").args(args).status()?.success();
    if !ok {
        Err(format!("failed to run tmux command with args: {}", args.join(" ")).into())
    } else {
        Ok(())
    }
}
