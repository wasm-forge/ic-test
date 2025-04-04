use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::ic_test_json::IcpTestSetup;

pub const FOUNDRY_TOML: &str = "foundry.toml";

//
pub fn get_main_project_dir() -> Result<PathBuf> {
    // TODO: check if we need to return one of the parent folders
    let cur_dir = env::current_dir()?;
    Ok(cur_dir)
}

//
pub fn get_test_project_dir(setup: &IcpTestSetup) -> Result<PathBuf> {
    // TODO: check if we need to return one of the parent folders
    let mut cur_dir = env::current_dir()?;
    cur_dir.push(setup.test_folder.clone());
    Ok(cur_dir)
}

// get path relative to the main project
pub fn get_relative_path(target_path: &Path, setup: &IcpTestSetup) -> Result<PathBuf> {
    let _ = setup;
    let mut ret = PathBuf::new();

    let relative_path = if target_path.is_absolute() {
        let project_dir = get_main_project_dir()?;
        target_path.strip_prefix(&project_dir)?
    } else {
        target_path
    };

    ret.push(relative_path);

    Ok(ret)
}

// path prefix to get from the test folder to the target path
pub fn get_path_relative_to_test_dir(target_path: &Path, setup: &IcpTestSetup) -> Result<PathBuf> {
    let mut ret = PathBuf::new();

    // for each test path part add ".."
    for _ in setup.test_folder.trim().split("/") {
        ret.push("..");
    }

    let relative_path = get_relative_path(target_path, setup)?;

    ret.push(relative_path);

    Ok(ret)
}

// try find wasm for a given canister name
pub fn find_wasm(canister_name: &str, setup: &IcpTestSetup) -> Result<String> {
    let mut canister_dir = get_main_project_dir()?;

    canister_dir.push(format!(".dfx/local/canisters/{canister_name}"));

    let names = [
        format!("{canister_name}.wasm"),
        format!("{canister_name}.wasm.gz"),
    ];

    for name in &names {
        let mut wasm_file = canister_dir.clone();
        wasm_file.push(name);

        if wasm_file.exists() && wasm_file.is_file() {
            let relative_wasm = get_relative_path(wasm_file.as_path(), setup)?;

            return Ok(relative_wasm.to_string_lossy().to_string());
        }
    }

    Err(anyhow::anyhow!(format!(
        "Wasm file for the canister {canister_name} was not found."
    )))
}
