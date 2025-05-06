use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
};

use anyhow::Error;
use candid::Principal;
use wf_cdk_bindgen::code_generator;

use crate::{
    candid_value_to_rust,
    common::{expand_path, get_path_relative_to_test_dir},
    ic_test_json::{CanisterSetup, ContractSetup, IcpTestSetup},
};

use askama::Template;

#[derive(Template)]
#[template(path = "icp/mod.rs.txt")]
struct ModRsIcpTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
}

#[derive(Template)]
#[template(path = "icp_evm/mod.rs.txt")]
struct ModRsIcpEvmTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
    contracts: &'a Vec<ContractSetup>,
}

pub fn generate_bindings(setup: &mut IcpTestSetup) -> Result<(), Error> {
    // current folder
    let bindings_path = env::current_dir()?
        .join(setup.test_folder.clone())
        .join("src/bindings");

    fs::create_dir_all(&bindings_path)?;

    generate_mod_rs(setup, &bindings_path)?;

    let test_folder = setup.test_folder.clone();

    // generate candid files for each canister
    for (canister_name, canister) in setup.icp_setup.canisters.iter_mut() {
        if let Some(candid) = &canister.candid_path {
            // read candid
            let candid_path = expand_path(Path::new(&candid))?;

            let mut canister_file = bindings_path.clone();
            canister_file.push(format!("{}.rs", &canister.var_name));

            // try parse candid file
            let mut config = code_generator::Config::new();

            config.set_target(code_generator::Target::Builder);

            config.set_service_name(canister.service_name.clone());

            if let Some(specified_id) = canister.specified_id.clone() {
                config.set_canister_id(Principal::from_text(&specified_id).unwrap());
            }

            let mut path = PathBuf::new();
            path.push(canister.wasm.clone());

            let path = get_path_relative_to_test_dir(path.as_path(), &test_folder)?;

            config.set_canister_wasm_path(path.to_string_lossy().to_string());

            let (env, actor) =
                candid_parser::typing::pretty_check_file(candid_path.as_path()).unwrap();

            let candid_value_string = if let Some(path) = canister.init_args_path.clone() {
                std::fs::read_to_string(path)?
            } else if let Some(args) = canister.init_args.clone() {
                args
            } else {
                "".to_string()
            };

            if !candid_value_string.trim().is_empty() {
                let values = candid_parser::parse_idl_args(&candid_value_string)?;

                // generate from candid_value
                canister.init_args_rust = candid_value_to_rust::generate_init_values(
                    canister_name,
                    &env,
                    &actor,
                    &values.args,
                );
            } else {
                // generate from candid type
                canister.init_args_rust =
                    candid_value_to_rust::generate_default_values(canister_name, &env, &actor);
            }

            let content = wf_cdk_bindgen::code_generator::compile(&config, &env, &actor);

            fs::write(&canister_file, content)
                .unwrap_or_else(|_| panic!("Could not write to file: {}", &canister.var_name));

            let _output = std::process::Command::new("rustfmt")
                .arg(&canister_file)
                .output()?;
        }
    }

    Ok(())
}

fn generate_mod_rs(setup: &IcpTestSetup, bindings_path: &Path) -> Result<(), Error> {
    let mut mod_file: PathBuf = bindings_path.to_path_buf();
    mod_file.push("mod.rs");

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

    // are we using EVM template?
    if let Some(evm_setup) = &setup.evm_setup {
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

        let mod_template = ModRsIcpEvmTemplate {
            canisters: &canisters,
            contracts: &contracts,
        };

        let mod_content = mod_template.render()?;

        fs::write(mod_file, mod_content)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file"));
    } else {
        let mod_template = ModRsIcpTemplate {
            canisters: &canisters,
        };

        let mod_content = mod_template.render()?;

        fs::write(mod_file, mod_content)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file"));
    }

    Ok(())
}
