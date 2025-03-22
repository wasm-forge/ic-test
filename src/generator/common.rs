use std::{env, path::PathBuf};

pub fn get_project_dir() -> anyhow::Result<PathBuf> {
    // TODO: check if we need to return one of the parent folders
    let cur_dir = env::current_dir()?;
    Ok(cur_dir)
}
