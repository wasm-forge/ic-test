use std::{fs, path::Path};

use anyhow::Error;
use askama::Template;

use crate::{
    common::{get_path_relative_to_test_dir, get_test_project_dir},
    ic_test_json::{CanisterSetup, ContractSetup, IcpTestSetup},
};

#[derive(Template)]
#[template(path = "lib.rs.txt")]
struct LibRsTemplate {}

#[derive(Template)]
#[template(path = "icp/tests.rs.txt")]
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

pub fn generate_test_rs(setup: &IcpTestSetup) -> Result<(), Error> {
    let project_dir = get_test_project_dir(setup)?;

    let mut src_dir = project_dir.clone();
    src_dir.push("src");
    fs::create_dir_all(&src_dir)?;

    let canisters: Vec<CanisterSetup> = setup
        .icp_setup
        .canisters
        .iter()
        .map(|x| {
            let mut x = x.1.clone();
            let path = Path::new(&x.wasm);
            let relative = get_path_relative_to_test_dir(path, &setup.test_folder).unwrap();
            x.wasm = relative.to_string_lossy().to_string();
            x
        })
        .collect();

    // generate test.rs
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

    println!("{content}");

    let tests_rs = src_dir.join("tests.rs");

    if !tests_rs.exists() || setup.forced {
        fs::write(tests_rs, content)
            .unwrap_or_else(|_| panic!("Could not create the tests.rs file"));
    }

    Ok(())
}

pub fn generate_cargo_toml(setup: &IcpTestSetup) -> Result<(), Error> {
    let project_dir = get_test_project_dir(setup)?;

    let version = env!("CARGO_PKG_VERSION").to_string();

    let content = if let Some(_evm_setup) = &setup.evm_setup {
        let template = CargoTomlIcpEvmTemplate {
            test_folder: &setup.test_folder,
            ic_test_version: &version,
        };

        template.render()?
    } else {
        let template = CargoTomlIcpTemplate {
            test_folder: &setup.test_folder,
            ic_test_version: &version,
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
