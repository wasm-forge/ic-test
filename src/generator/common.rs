use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::{dfx_json::DfxCanister, ic_test_json::IcpTestSetup};

pub const FOUNDRY_TOML: &str = "foundry.toml";

//
pub fn get_home_dir() -> PathBuf {
    dirs::home_dir().expect("Home directory not found!")
}

//
pub fn expand_path(path: &Path) -> PathBuf {
    let path_str = path.to_string_lossy();

    let ret = if let Some(suffix) = path_str.strip_prefix("$HOME/") {
        let home = get_home_dir();
        home.join(suffix)
    } else {
        path.to_path_buf()
    };

    ret
}

//
pub fn get_main_project_dir() -> Result<PathBuf> {
    // TODO: check if we need to return one of the parent folders
    let cur_dir = env::current_dir()?;

    // the main project dir should contain Cargo.toml
    let toml = cur_dir.join("Cargo.toml");
    if !toml.is_file() {
        return Err(anyhow::anyhow!(
            "The main project directory containing 'Cargo.toml' was not found. Please start the tool from the root folder of your project.",
        ));
    }

    Ok(cur_dir)
}

//
pub fn get_test_project_dir(setup: &IcpTestSetup) -> Result<PathBuf> {
    // TODO: check if we need to return one of the parent folders
    let mut cur_dir = env::current_dir()?;
    cur_dir.push(setup.test_folder.clone());
    Ok(cur_dir)
}

// get path relative to the main project or the user home folder
pub fn get_relative_path(target_path: &Path) -> Result<PathBuf> {
    let relative_path = if target_path.is_absolute() {
        let project_dir = get_main_project_dir()?;

        if let Ok(path) = target_path.strip_prefix(&project_dir) {
            // is relative to project?
            PathBuf::from(path)
        } else {
            // try to find it relative to $HOME
            let home: PathBuf = get_home_dir();

            let stripped = target_path.strip_prefix(home.as_path())?;

            let home = PathBuf::from("$HOME");
            home.join(stripped)
        }
    } else {
        let mut rel_path = PathBuf::new();
        rel_path.push(target_path);
        rel_path
    };

    Ok(relative_path)
}

// path prefix to get from the test folder to the target path
pub fn get_path_relative_to_test_dir(target_path: &Path, test_folder: &str) -> Result<PathBuf> {
    if target_path.starts_with("$HOME") || target_path.starts_with("/") {
        // do not try to process the absolute paths
        return Ok(PathBuf::new().join(target_path));
    }

    let mut ret = PathBuf::new();

    // for each test path part add ".."
    for _ in test_folder.trim().split("/") {
        ret.push("..");
    }

    let relative_path = get_relative_path(target_path)?;

    ret.push(relative_path);

    Ok(ret)
}

pub fn get_pull_folder(canister: &DfxCanister) -> Option<PathBuf> {
    if let Some(canister_type) = &canister.canister_type {
        if canister_type == "pull" {
            if let Some(id) = &canister.id {
                let cache_canister_dir =
                    PathBuf::new().join(format!("$HOME/.cache/dfinity/pulled/{id}"));

                return Some(cache_canister_dir);
            }
        }
    }

    None
}

pub fn find_candid(canister_name: &str, canister: &DfxCanister) -> Option<PathBuf> {
    let mut files = Vec::new();

    // 1. try finding the candid in the dfx config
    if let Some(candid) = &canister.candid {
        files.push(PathBuf::from(candid.clone()));
    }

    // 2. try finding the candid file for the pulled canister
    let pull_dir = get_pull_folder(canister);
    if let Some(pull) = pull_dir {
        files.push(pull.join("service.did"));
    }

    // 3. try using dfx cached .did file
    files.push(PathBuf::from(format!(
        ".dfx/local/canisters/{canister_name}/constructor.did"
    )));

    for file in files {
        if file.exists() && file.is_file() {
            let candid = get_relative_path(file.as_path()).expect("Error finding relative path!");

            return Some(candid);
        }
    }

    None
}

/// try find wasm for a given canister name
pub fn find_wasm(
    canister_name: &str,
    canister: &DfxCanister,
    _setup: &IcpTestSetup,
) -> Result<Option<String>> {
    let mut files = Vec::new();

    // 1. direct wasm property search form the dfx setup
    if let Some(wasm) = &canister.wasm {
        files.push(PathBuf::from(wasm.clone()));
    }

    if let Some(wasm) = &canister.wasm {
        if wasm.ends_with(".wasm") {
            files.push(PathBuf::from(wasm.to_owned() + ".gz"));
        }
    }

    // 2. try checking the wasm target folder
    let canister_dir = get_main_project_dir()?.join(format!(
        "target/wasm32-unknown-unknown/release/{canister_name}.wasm"
    ));
    files.push(canister_dir);
    let canister_dir = get_main_project_dir()?.join(format!(
        "target/wasm32-unknown-unknown/release/{canister_name}.wasm.gz"
    ));
    files.push(canister_dir);

    let canister_dir = get_main_project_dir()?.join(format!(
        "target/wasm32-unknown-unknown/debug/{canister_name}.wasm"
    ));
    files.push(canister_dir);
    let canister_dir = get_main_project_dir()?.join(format!(
        "target/wasm32-unknown-unknown/debug/{canister_name}.wasm.gz"
    ));
    files.push(canister_dir);

    // 3. try checking the .dfx folder
    let canister_dir = get_main_project_dir()?.join(format!(
        ".dfx/local/canisters/{canister_name}/{canister_name}.wasm"
    ));
    files.push(canister_dir);

    let canister_dir = get_main_project_dir()?.join(format!(
        ".dfx/local/canisters/{canister_name}/{canister_name}.wasm.gz"
    ));
    files.push(canister_dir);

    // 4. try the pull folder
    let pull_dir = get_pull_folder(canister);

    if let Some(dir) = pull_dir {
        files.push(dir.join("canister.wasm"));
        files.push(dir.join("canister.wasm.gz"));
    }

    for wasm_file in &files {
        if wasm_file.exists() && wasm_file.is_file() {
            let relative_wasm =
                get_relative_path(wasm_file.as_path()).expect("Failed to get relative path: {}");
            return Ok(Some(relative_wasm.to_string_lossy().to_string()));
        }
    }

    Ok(None)
}

/// Recursively search for a file with a given name starting from a specified directory.
///
/// # Returns
///
/// * `Some(PathBuf)` - The full path to the found file if it exists within the depth limit (includes the search file name).
/// * `None` - If the file is not found within the specified depth.
///
pub fn search_file_recursively(path: &Path, depth: u32, search_name: &str) -> Option<PathBuf> {
    let path = Path::new(path);

    let file = path.join(search_name);

    if file.is_file() {
        return Some(file);
    }

    if depth == 0 {
        return None;
    }

    let entries =
        std::fs::read_dir(path).expect("Failed to read the directory during search: {path:?}");

    // continue search in each subdirectory
    for entry in entries.filter_map(|entry| entry.ok()) {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let cur_path = path.join(&file_name);

        if file_name.starts_with(".") || file_name == "target" {
            continue;
        }

        if !cur_path.is_dir() {
            continue;
        }

        let found = search_file_recursively(&cur_path, depth - 1, search_name);

        if found.is_some() {
            return found;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn relative_path_inside_project_dir() {
        let project_dir = get_main_project_dir().unwrap();
        let target = project_dir.join("src/main.rs");

        let result = get_relative_path(&target).unwrap();

        assert_eq!(result, PathBuf::from("src/main.rs"));
    }

    #[test]
    fn relative_path_for_relative_input() {
        let target = Path::new("data/config.json");

        let result = get_relative_path(target).unwrap();
        assert_eq!(result, PathBuf::from("data/config.json"));
    }

    #[test]
    fn error_if_not_under_project_or_home() {
        // Create a temp dir unrelated to home or project
        let unrelated_path = Path::new("/tmp/file.txt");

        let result = get_relative_path(unrelated_path);

        assert!(
            result.is_err(),
            "Expected error when path is outside home/project"
        );
    }

    #[test]
    fn search_rec() {
        let r = search_file_recursively(Path::new("."), 2, "main.rs");
        assert!(r.is_some());

        // check it doesn't search under target
        let r = search_file_recursively(Path::new("."), 1, "main.rs");
        assert!(r.is_none());
    }

    #[test]
    fn expand() {
        let expanded = expand_path(Path::new("$HOME/test"));

        println!("{expanded:?}");
    }
}
