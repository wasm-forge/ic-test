mod arguments;
mod candid_to_rust;
mod candid_value_to_rust;
mod common;
mod dependencies;
mod dfx_json;
mod foundry_toml;
mod ic_test_json;
mod interactive_setup;
mod test_structure;

use std::path::Path;

use arguments::IcpTestArgs;
use clap::Parser;
use common::get_main_project_dir;
use git2::{Repository, Status, StatusOptions};
use ic_test_json::{init_test_config, store_test_config, IcpTestSetup};
use log::{debug, error};

fn has_uncommitted_changes(repo_path: &str, setup: &IcpTestSetup) -> Result<bool, git2::Error> {
    let repo = match Repository::open(repo_path) {
        Ok(r) => r,
        Err(_) => return Ok(false),
    };

    let statuses = repo.statuses(Some(
        StatusOptions::new()
            .include_untracked(true)
            .renames_head_to_index(true)
            .renames_index_to_workdir(true),
    ))?;

    let test_folder = format!(
        "{}/",
        Path::new(&setup.test_folder)
            .to_string_lossy()
            .to_string()
            .trim_end_matches('/')
    );

    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            if (path == "Cargo.toml" || path.starts_with(&test_folder))
                && entry.status() != Status::CURRENT
            {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn process_arguments(args: &IcpTestArgs, setup: &mut IcpTestSetup) -> anyhow::Result<()> {
    // Generate files based on the setup prepared

    match &args.command {
        arguments::Command::New {
            test_folder: _,
            force: _,
        } => {
            // we do not initialize if the tests folder exists already
            let test_folder = Path::new(&setup.test_folder);

            if test_folder.exists() {
                let err_msg = format!("The test directory '{}' exists already, select a different test folder to avoid data loss.", test_folder.to_string_lossy());
                error!("{err_msg}");
                return Err(anyhow::anyhow!(err_msg));
            }

            let ic_test_json = Path::new(&args.ic_test_json);
            if ic_test_json.exists() {
                let err_msg = "The 'ic-test.json' was initialized already, use the 'ic-test update' command instead.";
                error!("{err_msg}");
                return Err(anyhow::anyhow!(err_msg));
            }

            let root = get_main_project_dir()?.to_string_lossy().to_string();
            if !setup.forced && has_uncommitted_changes(&root, setup)? {
                let err_msg =
                    "Commit/reject any changes before calling 'ic-test new' to avoid data loss.";
                error!("{err_msg}");
                return Err(anyhow::anyhow!(err_msg));
            }

            // init project using cargo
            let _status = std::process::Command::new("cargo")
                .arg("new")
                .arg(test_folder.to_string_lossy().to_string())
                .arg("--lib")
                .status()?;

            // we only regenerate cargo when creating the test project
            setup.regenerate_cargo = true;
        }
        arguments::Command::Update {
            force: _,
            name: _,
            wasm: _,
            init_arg_file: _,
            init_arg: _,
            sol_json: _,
        } => {
            let test_folder = Path::new(&setup.test_folder);

            let ic_test_json = Path::new(&args.ic_test_json);

            // we want to avoid update if the ic-test.json is missing
            // (hence, we don't know if we can just regenerate on top of the test folder)
            if !ic_test_json.exists() {
                let err_msg =
                    "The test ic-test.json was not initialized yet, use the 'new' command instead.";
                error!("{err_msg}");
                return Err(anyhow::anyhow!(err_msg));
            }

            // the test folder must exist already
            if !test_folder.exists() {
                let err_msg = format!("The test directory '{}' does not exist in the project, use the 'new' command instead.", test_folder.to_string_lossy());
                error!("{err_msg}");
                return Err(anyhow::anyhow!(err_msg));
            }
        }
        arguments::Command::Add { command: _ } => {}
    }

    test_structure::generate_cargo_toml(setup)?;

    test_structure::generate_lib_rs(setup)?;

    candid_to_rust::generate_bindings(setup)?;

    test_structure::generate_test_setup_test_rs(args, setup)?;

    setup.is_complete = true;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let version = env!("CARGO_PKG_VERSION");
    debug!("ic-test V{version}");

    let args = {
        let args: Vec<String> = std::env::args().collect();
        let argc = args.len();

        if argc > 1 {
            let result = IcpTestArgs::parse();

            // if the root parameter was provided, change directory before doing anything else
            if let Some(root) = &result.root {
                std::env::set_current_dir(root)
                    .expect("Failed to find the root directory, where to run the ic-test!");
            }

            if args[1] == "new" && argc == 2 {
                interactive_setup::interactive_arguments()?
            } else {
                result
            }
        } else {
            interactive_setup::interactive_arguments()?
        }
    };

    debug!("args: {args:?}");

    // initialize generator setup
    let mut setup = init_test_config(&args)?;

    debug!("setup: {setup:?}");

    process_arguments(&args, &mut setup)?;

    store_test_config(&args, &setup)?;

    if setup.is_complete {
        println!(
            "Successfully generated test bindings in project '{}'.",
            setup.test_folder
        );

        if let arguments::Command::New {
            test_folder: _,
            force: _,
        } = args.command
        {
            if setup.test_setup_rs_regenerated {
                println!(
                "A sample test file has been created: '{}/src/tests.rs'. You can modify this file to write your own tests.", setup.test_folder
            );
            }
        }
    }

    Ok(())
}
