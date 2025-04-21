mod arguments;
mod candid_to_rust;
mod common;
mod dependencies;
mod dfx_json;
mod foundry_toml;
mod ic_test_json;
mod test_structure;

use std::{
    net::TcpStream,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};

use arguments::IcpTestArgs;
use clap::Parser;
use common::get_main_project_dir;
use git2::{Repository, Status, StatusOptions};
use ic_test_json::{init_test_config, store_test_config, IcpTestSetup};
use log::{error, info};

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

fn is_dfx_running() -> bool {
    let connection = TcpStream::connect("127.0.0.1:4943");

    if let Ok(connection) = connection {
        let _ = connection.shutdown(std::net::Shutdown::Both);
        return true;
    }

    false
}

fn check_dfx_folder(_args: &IcpTestArgs) -> anyhow::Result<()> {
    // check, if we have .dfx folder
    let dfx_path = Path::new(".dfx");

    if !dfx_path.exists() {
        // Check if dfx is installed
        let dfx_check = Command::new("dfx")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match dfx_check {
            Ok(status) if status.success() => {
                info!("dfx is found");
            }
            _ => {
                let err_msg = "dfx is not installed or not available in PATH";
                error!("{err_msg}");
                return Err(anyhow::anyhow!(err_msg));
            }
        }

        let dfx_running = is_dfx_running();

        if !dfx_running {
            info!("Start dfx...");
            let mut _dfx_process = Command::new("dfx")
                .arg("start")
                .arg("--background")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        }

        // Give it some time to boot up
        std::thread::sleep(Duration::from_secs(2));

        info!("exec: dfx canister create --all");
        // create all canisters
        let _status = Command::new("dfx")
            .arg("canister")
            .arg("create")
            .arg("--all")
            .status()?;

        info!("exec: dfx deps pull");
        // dfx deps pull
        let _status = Command::new("dfx").arg("deps").arg("pull").status()?;

        info!("exec: dfx build");
        // dfx build
        let _status = Command::new("dfx").arg("build").status()?;

        if !dfx_running {
            info!("Stop dfx...");
            let _status = Command::new("dfx").arg("stop").status()?;
        }
    }

    Ok(())
}

fn process_arguments(args: &IcpTestArgs, setup: &mut IcpTestSetup) -> anyhow::Result<()> {
    // Generate files based on the setup prepared

    match &args.command {
        arguments::Command::New { test_folder: _ } => {
            // we do not initialize if the tests folder exists already
            let test_folder = Path::new(&setup.test_folder);

            if test_folder.exists() {
                let err_msg = format!("The test directory '{}' exists already, select a different test folder to avoid data loss.", test_folder.to_string_lossy().to_string());
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
            if has_uncommitted_changes(&root, setup)? {
                let err_msg =
                    "Commit/reject any changes before calling 'ic-test new' to avoid data loss.";
                error!("{err_msg}");
                return Err(anyhow::anyhow!(err_msg));
            }

            // init project using cargo
            let _status = Command::new("cargo")
                .arg("new")
                .arg(test_folder.to_string_lossy().to_string())
                .arg("--lib")
                .status()?;

            // we only regenerate cargo when creating the test project
            setup.regenerate_cargo = true;
        }
        arguments::Command::Update { force: _ } => {
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
                let err_msg = format!("The test directory '{}' does not exist in the project, use the 'new' command instead.", test_folder.to_string_lossy().to_string());
                error!("{err_msg}");
                return Err(anyhow::anyhow!(err_msg));
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
    let args = IcpTestArgs::try_parse()?;

    check_dfx_folder(&args)?;

    let mut setup = init_test_config(&args)?;

    process_arguments(&args, &mut setup)?;

    store_test_config(&args, &setup)?;

    Ok(())
}
