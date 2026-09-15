use std::{error::Error, io, os::unix::process::CommandExt, process::Command};

use crate::{config::Preset, path::ProjectPath};

/// launch prepares tmux session and then attaches with exec.
/// This function will never return or return error, it basically "exits" to tmux, meaning no
/// destructors will be called.
pub fn launch(project: &ProjectPath, preset: Option<&Preset>) -> Result<(), Box<dyn Error>> {
    let exists = Command::new("tmux")
        .args(["has-session", "-t", &project.target()])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !exists {
        run_tmux_cmd(&[
            "new-session",
            "-d",
            "-s",
            &project.sesh_name,
            "-c",
            &project.real.display().to_string(),
        ])?;
        if let Some(p) = preset {
            setup_windows(project, p)?;
        }
        run_tmux_cmd(&["select-window", "-t", &format!("{}:^", project.target())])?;
    }

    Err(attach_sesh(project).into())
}

fn setup_windows(project: &ProjectPath, preset: &Preset) -> Result<(), Box<dyn Error>> {
    let mut first = true;
    let real = project.real.display().to_string();

    for w in &preset.windows {
        if first {
            first = false;
            if let Some(win_name) = &w.name {
                run_tmux_cmd(&[
                    "rename-window",
                    "-t",
                    &format!("{}:", project.target()),
                    win_name,
                ])?;
            }
        } else {
            let target = project.target();
            let mut args = vec!["new-window", "-t", &target, "-c", &real];
            if let Some(name) = &w.name {
                args.extend(["-n", name.as_str()]);
            }
            run_tmux_cmd(&args)?;
        }

        if !w.cmd.is_empty() {
            run_tmux_cmd(&[
                "send-keys",
                "-t",
                &format!("{}:", project.target()),
                &w.cmd,
                "Enter",
            ])?;
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

fn attach_sesh(project: &ProjectPath) -> io::Error {
    let cmd = match is_in_tmux_session() {
        false => "attach-session",
        true => "switch-client",
    };

    Command::new("tmux")
        .args([cmd, "-t", &project.target()])
        .exec()
}

fn is_in_tmux_session() -> bool {
    std::env::var("TERM_PROGRAM").is_ok_and(|program| program == "tmux")
}
