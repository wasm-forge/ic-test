mod arguments;
mod candid_to_rust;
mod common;
mod dependencies;
mod dfx_json;
mod foundry_toml;
mod ic_test_json;
mod test_structure;

use std::{path::Path, process::Command};

use arguments::IcTestArgs;
use clap::Parser;
use common::get_main_project_dir;
use dfx_json::add_canisters;
use foundry_toml::{add_contract, add_contracts};
use git2::{Repository, StatusOptions};
use ic_test_json::{init_test_config, store_test_config, IcTestSetup};

fn has_uncommitted_changes(repo_path: &str) -> Result<bool, git2::Error> {
    let repo = match Repository::open(repo_path) {
        Ok(r) => r,
        Err(_) => return Ok(false),
    };

    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .renames_head_to_index(true)
        .renames_index_to_workdir(true);

    let statuses = repo.statuses(Some(&mut opts))?;
    Ok(!statuses.is_empty())
}

fn process_arguments(args: &IcTestArgs, setup: &mut IcTestSetup) -> anyhow::Result<()> {
    add_canisters(setup)?;

    add_contracts(setup)?;

    match &args.command {
        arguments::Command::Init {} => {}
        arguments::Command::Update {} => {}
        arguments::Command::Add { command } => {
            // either add a canister or a contract to the setup
            match command {
                arguments::AddCommand::Canister { name, wasm: _ } => {
                    println!("Adding canister {name}");
                    // TODO: add canister
                }
                arguments::AddCommand::Contract { name, sol_json } => {
                    println!("Adding contract {name}");
                    add_contract(name, sol_json, setup)?;
                }
            }
        }
    }

    ///////////////////////////////////
    // Generate / regenerate folders
    let ic_test_json = Path::new(&args.ic_test_json);
    let test_folder = Path::new(&setup.test_folder);

    match &args.command {
        arguments::Command::Init {} => {
            // we do not initialize if the tests folder exists already

            if test_folder.exists() {
                return Err(anyhow::anyhow!(
                    format!("The test directory '{}' exists already, select a different test folder to avoid data loss.", test_folder.to_string_lossy().to_string())
                ));
            }

            if ic_test_json.exists() {
                return Err(anyhow::anyhow!(
                    "The 'ic-test.json' was initialized already, use the 'update' command instead."
                ));
            }

            let root = get_main_project_dir()?.to_string_lossy().to_string();
            if has_uncommitted_changes(&root)? {
                return Err(anyhow::anyhow!(
                    "Commit/reject any changes before calling 'ic-test init' to avoid data loss."
                ));
            }

            // init project using cargo
            let _status = Command::new("cargo")
                .arg("new")
                .arg(test_folder.to_string_lossy().to_string())
                .arg("--lib")
                .status()?;
        }
        arguments::Command::Update {} => {
            // we want to avoid update if the ic-test.json is missing
            // (hence, we don't know if we can just regenerate on top of the test folder)
            if !ic_test_json.exists() {
                return Err(anyhow::anyhow!(
                    "The test ic-test was not initialized, use the 'init' command instead."
                ));
            }

            // the test folder must exist already
            if test_folder.exists() {
                return Err(anyhow::anyhow!(
                    format!("The test directory {} does not exist in the project, use the 'init' command instead.", test_folder.to_string_lossy().to_string())
                ));
            }
        }
        arguments::Command::Add { command: _ } => {}
    }

    // generate folder structure
    test_structure::generate(setup, true)?;

    // generate candid files
    candid_to_rust::generate(setup)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = IcTestArgs::try_parse()?;

    let mut setup = init_test_config(&args)?;

    process_arguments(&args, &mut setup)?;

    store_test_config(&args, &setup)?;

    Ok(())
}
