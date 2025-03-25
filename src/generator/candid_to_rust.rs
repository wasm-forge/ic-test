use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Error;
use candid::Principal;
use ic_cdk_bindgen::code_generator;

use crate::{
    common::get_path_relative_to_test_dir,
    ic_test_json::{CanisterSetup, ContractSetup, IcTestSetup},
};

use askama::Template;

#[derive(Template)]
#[template(path = "mod.rs.txt")]
struct ModRsTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
}

#[derive(Template)]
#[template(path = "mod.evm.rs.txt")]
struct ModRsEvmTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
    contracts: &'a Vec<ContractSetup>,
}

pub fn generate(setup: &IcTestSetup) -> Result<(), Error> {
    // current folder
    let mut bindings_path = env::current_dir()?;
    bindings_path.push(setup.test_folder.clone());
    bindings_path.push("src/bindings");

    fs::create_dir_all(&bindings_path)?;

    generate_mod_rs(setup, &bindings_path)?;

    // generate candid files for each canister
    for (_canister_name, canister) in setup.icp_setup.canisters.iter() {
        if let Some(gen_candid_file) = &canister.gen_candid_file {
            // read candid
            let candid_path = Path::new(&gen_candid_file);

            let mut canister_file = bindings_path.clone();
            canister_file.push(format!("{}.rs", &canister.name));

            // try parse candid file
            let mut config = code_generator::Config::new();

            config.set_target(code_generator::Target::Builder);
            config.set_service_name(format!("{}Canister", canister.name));
            if let Some(specified_id) = canister.specified_id.clone() {
                config.set_canister_id(Principal::from_text(&specified_id).unwrap());
            }

            let mut path = PathBuf::new();
            path.push(canister.wasm.clone());

            let path = get_path_relative_to_test_dir(path.as_path(), setup)?;

            config.set_canister_wasm_path(path.to_string_lossy().to_string());

            let (env, actor) = candid_parser::typing::pretty_check_file(candid_path).unwrap();

            let content = ic_cdk_bindgen::code_generator::compile(&config, &env, &actor);

            fs::write(&canister_file, content)
                .unwrap_or_else(|_| panic!("Could not write to file: {}", &canister.name));
            let output = std::process::Command::new("rustfmt")
                .arg(&canister_file)
                .output();
            println!("{:?}", output);
        }
    }

    Ok(())
}

fn generate_mod_rs(setup: &IcTestSetup, bindings_path: &Path) -> Result<(), Error> {
    let mut mod_file: PathBuf = bindings_path.to_path_buf();
    mod_file.push("mod.rs");

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

    // are we using EVM template?
    if let Some(evm_setup) = &setup.evm_setup {
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

        let mod_template = ModRsEvmTemplate {
            canisters: &canisters,
            contracts: &contracts,
        };
        let mod_content = mod_template.render()?;

        fs::write(mod_file, mod_content)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file"));
    } else {
        let mod_template = ModRsTemplate {
            canisters: &canisters,
        };
        let mod_content = mod_template.render()?;

        fs::write(mod_file, mod_content)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file"));
    }

    Ok(())
}
