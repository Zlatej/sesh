use std::{error::Error, io::Cursor};

use skim::{
    Skim,
    prelude::{SkimItemReader, SkimOptionsBuilder},
};

use crate::path::ProjectPath;

pub fn pick_project(projects: &[ProjectPath]) -> Result<Option<&ProjectPath>, Box<dyn Error>> {
    let display: Vec<String> = projects.iter().map(|x| x.tilde.clone()).collect();
    let Some(picked) = pick(&display, "  projects>  ")? else {
        return Ok(None);
    };
    Ok(projects.iter().find(|p| p.tilde == picked))
}

pub fn pick_preset(presets: &[String]) -> Result<Option<String>, Box<dyn Error>> {
    pick(presets, "  preset>  ")
}

fn pick(items: &[String], prompt: &str) -> Result<Option<String>, Box<dyn Error>> {
    if items.is_empty() {
        return Ok(None);
    }

    let opts = SkimOptionsBuilder::default().prompt(prompt).build()?;
    let input = items.join("\n");
    let reader = SkimItemReader::default();
    let rx = reader.of_bufread(Cursor::new(input));

    let output = match Skim::run_with(opts, Some(rx)) {
        Ok(o) if !o.is_abort => o,
        _ => return Ok(None),
    };
    Ok(output
        .selected_items
        .first()
        .map(|item| item.output().to_string()))
}
