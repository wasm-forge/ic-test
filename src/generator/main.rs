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

use std::{net::TcpStream, path::Path, process::Stdio, time::Duration};

use arguments::IcpTestArgs;
use common::get_main_project_dir;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use git2::{Repository, Status, StatusOptions};
use ic_test_json::{init_test_config, store_test_config, IcpTestSetup};
use log::{debug, error, info};

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

fn run_dfx_commands() -> Result<(), anyhow::Error> {
    std::thread::sleep(Duration::from_secs(2));
    info!("exec: dfx canister create --all");
    let _status = std::process::Command::new("dfx")
        .arg("canister")
        .arg("create")
        .arg("--all")
        .status()?;
    info!("exec: dfx deps pull");
    let _status = std::process::Command::new("dfx")
        .arg("deps")
        .arg("pull")
        .status()?;
    info!("exec: dfx build");
    let _status = std::process::Command::new("dfx").arg("build").status()?;
    Ok(())
}

fn check_dfx_folder(args: &IcpTestArgs) -> anyhow::Result<()> {
    // check, if we have .dfx folder
    let dfx_path = get_main_project_dir()?.join(".dfx");
    let mut run_dfx_build = false;

    if !dfx_path.exists() && args.ui == Some(true) {
        let prompt =
            "The .dfx folder is not present, do you want to attempt to run the 'dfx build' now?"
                .to_string();

        run_dfx_build = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&["yes", "no"])
            .default(0)
            .interact_opt()?
            == Some(0);
    }

    if run_dfx_build {
        let dfx_was_running = start_dfx()?;

        let res = run_dfx_commands();

        if !dfx_was_running {
            info!("Stopping dfx...");
            let _status = std::process::Command::new("dfx").arg("stop").status()?;
        }

        res?;
    } else {
        debug!(".dfx was found")
    }

    Ok(())
}

fn start_dfx() -> anyhow::Result<bool> {
    // Check if dfx is installed
    let dfx_check = std::process::Command::new("dfx")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match dfx_check {
        Ok(status) if status.success() => {
            info!("dfx is found");
        }

        _ => {
            let err_msg = "dfx is not installed or not available in PATH!";
            error!("{err_msg}");
            return Err(anyhow::anyhow!(err_msg));
        }
    }

    let dfx_was_running = is_dfx_running();
    if !dfx_was_running {
        info!("Starting dfx...");
        let mut _dfx_process = std::process::Command::new("dfx")
            .arg("start")
            .arg("--background")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    } else {
        info!("found dfx running!")
    }

    Ok(dfx_was_running)
}

fn check_dfx_folder_after_canister_search(
    args: &IcpTestArgs,
    setup: &mut IcpTestSetup,
) -> anyhow::Result<()> {
    // check, if we have .dfx folder
    let mut run_dfx_build = false;

    if setup.rerun_dfx_build && args.ui == Some(true) {
        let theme = ColorfulTheme::default();
        let yes_no = vec!["yes", "no"];

        let prompt =
            "Some of the canisters were not found in .dfx, do you want to attempt to run the 'dfx build' now?"
                .to_string();

        run_dfx_build = FuzzySelect::with_theme(&theme)
            .with_prompt(prompt)
            .items(&yes_no)
            .default(0)
            .interact_opt()?
            == Some(0);

        setup.rerun_dfx_build = true;
    }

    if run_dfx_build {
        let dfx_was_running = start_dfx()?;

        let res = run_dfx_commands();

        if !dfx_was_running {
            info!("Stopping dfx...");
            let _status = std::process::Command::new("dfx").arg("stop").status()?;
        }

        res?;
    } else {
        debug!(".dfx was found")
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
            if has_uncommitted_changes(&root, setup)? {
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
            command: _,
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

    test_structure::generate_test_rs(args, setup)?;

    setup.is_complete = true;

    Ok(())
}

fn _test() -> anyhow::Result<()> {
    let candid_path = Path::new("tests/chain_fusion.did");
    let candid_value_path = Path::new("tests/initArgument.did");

    let (env, actor) = candid_parser::typing::pretty_check_file(candid_path).unwrap();

    let candid_value = std::fs::read_to_string(candid_value_path)?;

    let arg_value = candid_parser::parse_idl_args(&candid_value)?;

    //let init_args_rust = type2rust::generate_init_args_rust(candid_path, candid_value_path)?;

    let rust =
        candid_value_to_rust::generate_init_values("chain_fusion", &env, &actor, &arg_value.args);
    println!("{}", rust);

    let rust = candid_value_to_rust::generate_default_values("chain_fusion", &env, &actor);
    println!("{}", rust);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    //return _test();

    env_logger::init();

    let version = env!("CARGO_PKG_VERSION");
    debug!("ic-test V{}", version);

    let args = interactive_setup::interactive_arguments()?;

    debug!("args: {:?}", args);

    // check if we want to run dfx build
    check_dfx_folder(&args)?;

    // initialize generator setup
    let mut setup = init_test_config(&args)?;

    debug!("setup: {:?}", setup);

    // check if we still want to re-run dfx build
    if setup.rerun_dfx_build {
        check_dfx_folder_after_canister_search(&args, &mut setup)?;
        setup = init_test_config(&args)?;
    }

    process_arguments(&args, &mut setup)?;

    store_test_config(&args, &setup)?;

    if setup.is_complete {
        println!(
            "Successfully generated test bindings in project '{}'.",
            setup.test_folder
        );
        if setup.tests_rs_regenerated {
            println!(
                "A sample test file has been created: '{}/src/tests.rs'. You can modify this file and write your own tests.", setup.test_folder
            );
        }
    }

    Ok(())
}
