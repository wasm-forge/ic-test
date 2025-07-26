use std::{fs, path::Path};

use anyhow::Error;
use askama::Template;
use dialoguer::{theme::ColorfulTheme, Input};
use log::info;

use crate::{
    arguments::{Command, IcpTestArgs},
    candid_to_rust::get_generatable_canisters,
    common::{get_path_relative_to_test_dir, get_test_project_dir},
    ic_test_json::{CanisterSetup, ContractSetup, IcpTestSetup},
};

#[derive(Template)]
#[template(path = "lib.rs.txt")]
struct LibRsTemplate {}

#[derive(Template)]
#[template(path = "icp/test_setup.rs.txt")]
struct TestSetupRsIcpTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
}

#[derive(Template)]
#[template(path = "icp_evm/test_setup.rs.txt")]
struct TestSetupRsIcpEvmTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
    contracts: &'a Vec<ContractSetup>,
}

#[derive(Template)]
#[template(path = "icp/tests.rs.txt")]
#[allow(unused)]
struct TestsRsIcpTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
}

#[derive(Template)]
#[template(path = "icp_evm/tests.rs.txt")]
struct TestsRsIcpEvmTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
    contracts: &'a Vec<ContractSetup>,
}

#[derive(Template)]
#[template(path = "icp/Cargo.toml.txt")]
struct CargoTomlIcpTemplate<'a> {
    test_folder: &'a String,
    ic_test_version: &'a String,
}

#[derive(Template)]
#[template(path = "icp_evm/Cargo.toml.txt")]
struct CargoTomlIcpEvmTemplate<'a> {
    test_folder: &'a String,
    ic_test_version: &'a String,
}

pub fn generate_test_setup_test_rs(
    args: &IcpTestArgs,
    setup: &mut IcpTestSetup,
) -> Result<(), Error> {
    let test_setup_rs = get_test_project_dir(setup)?
        .join("src")
        .join("test_setup.rs");

    let tests_rs = get_test_project_dir(setup)?.join("src").join("tests.rs");

    // check if we are ok to overwrite the 'test_setup.rs' file if we are in "ui" mode and this is an update command
    if args.ui == Some(true) && test_setup_rs.exists() {
        if let Command::Update {
            force: _,
            name: _,
            wasm: _,
            init_arg_file: _,
            init_arg: _,
            sol_json: _,
        } = &args.command
        {
            let theme = ColorfulTheme::default();

            let prompt = "Do you also want to regenerate the existing 'test_setup.rs' file, type 'YES' to confirm:".to_string();

            let answer: String = Input::with_theme(&theme)
                .with_prompt(prompt)
                .allow_empty(true)
                .interact_text()?;

            setup.forced = answer == "YES";
        }
    }

    let project_dir = get_test_project_dir(setup)?;
    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let canisters: Vec<CanisterSetup> = get_generatable_canisters(setup);

    // generate test_setup.rs
    if !test_setup_rs.exists() || setup.forced {
        let content = if let Some(evm_setup) = &setup.evm_setup {
            let contracts: Vec<ContractSetup> = evm_setup
                .contracts
                .iter()
                .map(|x| {
                    let mut x = x.1.clone();
                    let path = Path::new(&x.sol_json);
                    let relative = get_path_relative_to_test_dir(path, &setup.test_folder).unwrap();
                    x.sol_json = relative.to_string_lossy().to_string();
                    x
                })
                .collect();

            let template = TestSetupRsIcpEvmTemplate {
                canisters: &canisters,
                contracts: &contracts,
            };
            template.render()?
        } else {
            let template = TestSetupRsIcpTemplate {
                canisters: &canisters,
            };
            template.render()?
        };

        if test_setup_rs.exists() {
            info!("Overwriting 'test_setup.rs'...")
        }

        fs::write(&test_setup_rs, content)
            .unwrap_or_else(|_| panic!("Could not create the 'test_setup.rs' file"));

        let _output = std::process::Command::new("rustfmt")
            .arg(&test_setup_rs)
            .arg("--edition")
            .arg("2021")
            .output()?;

        setup.test_setup_rs_regenerated = true;
    } else {
        println!(
            "Skipped regenerating 'test_setup.rs'. Use the --force option to overwrite this file."
        );
    }

    // generate tests.rs
    if !tests_rs.exists() {
        let content = if let Some(evm_setup) = &setup.evm_setup {
            let contracts: Vec<ContractSetup> = evm_setup
                .contracts
                .iter()
                .map(|x| {
                    let mut x = x.1.clone();
                    let path = Path::new(&x.sol_json);
                    let relative = get_path_relative_to_test_dir(path, &setup.test_folder).unwrap();
                    x.sol_json = relative.to_string_lossy().to_string();
                    x
                })
                .collect();

            let template = TestsRsIcpEvmTemplate {
                canisters: &canisters,
                contracts: &contracts,
            };
            template.render()?
        } else {
            let template = TestsRsIcpTemplate {
                canisters: &canisters,
            };
            template.render()?
        };

        fs::write(&tests_rs, content)
            .unwrap_or_else(|_| panic!("Could not create the 'tests.rs' file"));

        let _output = std::process::Command::new("rustfmt")
            .arg(&tests_rs)
            .arg("--edition")
            .arg("2021")
            .output()?;

        setup.test_setup_rs_regenerated = true;
    }

    Ok(())
}

pub fn generate_cargo_toml(setup: &IcpTestSetup) -> Result<(), Error> {
    let project_dir = get_test_project_dir(setup)?;

    let full_version = env!("CARGO_PKG_VERSION").to_string();
    let version_parts: Vec<&str> = full_version.split('.').collect();

    let ic_test_dep_version = if full_version.contains("cdk") {
        format!("{}.{}.0+ic-cdk.0.18", version_parts[0], version_parts[1])
    } else {
        format!("{}.{}", version_parts[0], version_parts[1])
    };

    let content = if let Some(_evm_setup) = &setup.evm_setup {
        let template = CargoTomlIcpEvmTemplate {
            test_folder: &setup.test_folder,
            ic_test_version: &ic_test_dep_version,
        };

        template.render()?
    } else {
        let template = CargoTomlIcpTemplate {
            test_folder: &setup.test_folder,
            ic_test_version: &ic_test_dep_version,
        };

        template.render()?
    };

    let mut cargo_toml = project_dir.clone();
    cargo_toml.push("Cargo.toml");
    if !cargo_toml.exists() || setup.regenerate_cargo {
        fs::write(cargo_toml, content)
            .unwrap_or_else(|_| panic!("Could not create the Cargo.toml file"));
    }

    Ok(())
}

pub fn generate_lib_rs(setup: &IcpTestSetup) -> Result<(), Error> {
    let project_dir = get_test_project_dir(setup)?;

    let mut src_dir = project_dir.clone();
    src_dir.push("src");
    fs::create_dir_all(&src_dir)?;

    let lib_rs = src_dir.join("lib.rs");
    let template = LibRsTemplate {};

    let content = template.render()?;
    fs::write(lib_rs, content).unwrap_or_else(|_| panic!("Could not create the lib.rs file"));

    Ok(())
}
