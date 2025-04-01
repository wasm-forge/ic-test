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
#[template(path = "Cargo.toml.txt")]
struct CargoTomlTemplate<'a> {
    test_folder: &'a String,
}

pub fn generate(setup: &IcpTestSetup, is_update: bool) -> Result<(), Error> {
    let _ = is_update;

    let project_dir = get_test_project_dir(setup)?;

    fs::create_dir_all(&project_dir)?;

    let mut src_dir = project_dir.clone();
    src_dir.push("src");
    fs::create_dir_all(&src_dir)?;

    let mut lib_rs = src_dir.clone();
    lib_rs.push("lib.rs");

    let mut tests_rs = src_dir.clone();
    tests_rs.push("tests.rs");

    let mut cargo_toml = project_dir.clone();
    cargo_toml.push("Cargo.toml");

    let template = CargoTomlTemplate {
        test_folder: &setup.test_folder,
    };

    let content = template.render()?;
    fs::write(cargo_toml, content)
        .unwrap_or_else(|_| panic!("Could not create the Cargo.toml file"));

    let canisters: Vec<CanisterSetup> = setup
        .icp_setup
        .canisters
        .iter()
        .map(|x| {
            let mut x = x.1.clone();
            let path = Path::new(&x.wasm);
            let relative = get_path_relative_to_test_dir(path, setup).unwrap();
            x.wasm = relative.to_string_lossy().to_string();
            x
        })
        .collect();

    let content = if let Some(evm_setup) = &setup.evm_setup {
        let contracts: Vec<ContractSetup> = evm_setup
            .contracts
            .iter()
            .map(|x| {
                let mut x = x.1.clone();
                let path = Path::new(&x.sol_json);
                let relative = get_path_relative_to_test_dir(path, setup).unwrap();
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

    fs::write(tests_rs, content).unwrap_or_else(|_| panic!("Could not create the tests.rs file"));

    let template = LibRsTemplate {};

    let content = template.render()?;
    fs::write(lib_rs, content).unwrap_or_else(|_| panic!("Could not create the lib.rs file"));

    Ok(())
}
