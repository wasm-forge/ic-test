use std::path::Path;

use anyhow::Error;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

use crate::{
    arguments::{self, IcpTestArgs},
    common::get_main_project_dir,
};

fn check_test_folder(test_folder: &str, project_dir: &Path) -> Result<(), Error> {
    if project_dir.join(test_folder).exists() {
        let p = format!("The file or folder '{}' exists already.", &test_folder);

        return Err(anyhow::anyhow!(p));
    }

    Ok(())
}

fn check_dfx_json_exists() -> anyhow::Result<()> {
    // check if dfx.json is found
    let dfx_json_path = Path::new("dfx.json");

    if !(dfx_json_path.exists() || dfx_json_path.is_file()) {
        return Err(anyhow::anyhow!("'dfx.json' not found! Make sure you are starting the ic-test at the root of your canister project."));
    }

    Ok(())
}

pub fn interactive_arguments() -> Result<IcpTestArgs, Error> {
    // check if we are in the main project folder
    let project_dir = get_main_project_dir()?;

    let args: Vec<String> = std::env::args().collect();
    let argc = args.len();

    if argc > 1 {
        let result = IcpTestArgs::parse();

        check_dfx_json_exists()?;

        if args[1] == "new" && argc == 2 {
            // Special case: "new" command with no additional args — continue to UI mode
        } else {
            return Ok(result);
        }
    }

    check_dfx_json_exists()?;

    let mut command = if project_dir.join("ic-test.json").is_file() {
        arguments::Command::Update {
            force: false,
            name: Some("".to_string()),
            wasm: None,
            init_arg_file: None,
            init_arg: None,
            sol_json: None,
        }
    } else {
        arguments::Command::New {
            test_folder: "tests".to_owned(),
        }
    };

    let theme = ColorfulTheme::default();
    let yes_no = vec!["yes", "no"];

    match command {
        arguments::Command::New { test_folder } => {
            let version = env!("CARGO_PKG_VERSION");

            let create_test_project = FuzzySelect::with_theme(&theme)
                            .with_prompt(format!(
                                "Welcome to IC Test framework V{version}!\n\nThe project searches for the .wasm and .did files inside the .dfx folder.\nFor the best result make sure the project is fully compiled and built with 'dfx build'.\n\nDo you want to create a new canister test project now?"),
                            )
                            .items(&yes_no)
                            .default(0)
                            .interact_opt()?
                            == Some(0);

            if !create_test_project {
                // return default help message
                let result = IcpTestArgs::parse();
                return Ok(result);
            }

            let mut test_folder: String = Input::with_theme(&theme)
                .with_prompt("Enter test folder name:")
                .with_initial_text(&test_folder)
                .interact_text()?;

            while let Err(er) = check_test_folder(&test_folder, &project_dir) {
                let p = format!("{}\nPlease enter another test folder name:", er);

                test_folder = Input::with_theme(&theme)
                    .with_prompt(p)
                    .with_initial_text(&test_folder)
                    .interact_text()?;
            }

            command = crate::arguments::Command::New { test_folder };
        }
        arguments::Command::Update {
            name: _,
            wasm: _,
            init_arg_file: _,
            init_arg: _,
            sol_json: _,
            force: _,
        } => {
            let version = env!("CARGO_PKG_VERSION");

            let regenerate = FuzzySelect::with_theme(&theme)
                            .with_prompt(format!(
                                "Welcome to IC Test framework V{version}!\n\nYou are about to regenerate you test project bindings.\nFor the best result make sure the project is fully compiled and built with 'dfx build'.\n\nDo you want to regenerate the bindings now?"),
                            )
                            .items(&yes_no)
                            .default(0)
                            .interact_opt()?
                            == Some(0);

            if !regenerate {
                // return default help message
                let result = IcpTestArgs::parse();
                return Ok(result);
            }
        }
        arguments::Command::Add { command: _ } => todo!(),
    }

    let ic_test_json = "ic-test.json".to_owned();

    Ok(IcpTestArgs {
        command,
        ic_test_json,
        skip_dfx_json: None,
        skip_foundry_toml: None,
        ui: Some(true),
    })
}
