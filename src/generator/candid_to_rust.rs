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
    common::{expand_path, get_path_relative_to_test_dir, HOME_VAR},
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
        if !canister.generate_bindings {
            continue;
        }

        if let Some(candid_path) = &canister.candid_path {
            // if candid_path begins with $HOME, exchange it with the actual home folder
            let mut candid_path = candid_path.clone();

            if candid_path.starts_with(HOME_VAR) {
                let home = dirs::home_dir().expect("Cound not find the home directory!");
                candid_path = candid_path.replace(HOME_VAR, home.to_string_lossy().as_ref());
            }

            let canister_file = bindings_path.join(format!("{}.rs", &canister.var_name));

            // try parsing candid file
            let mut config = code_generator::Config::new();

            config.set_target(code_generator::Target::Builder);

            config.set_service_name(canister.service_name.clone());

            config.set_type_attributes(
                "#[derive(CandidType, Clone, Debug, Deserialize, PartialEq)]".to_string(),
            );

            if let Some(specified_id) = canister.specified_id.clone() {
                config.set_canister_id(Principal::from_text(&specified_id).unwrap());
            }

            let wasm_path_str = canister.wasm.clone().unwrap_or_else(|| panic!("Cannot generate canister {canister_name} because its Wasm path is not provided"));

            let wasm_path = get_path_relative_to_test_dir(Path::new(&wasm_path_str), &test_folder)?;

            config.set_canister_wasm_path(wasm_path.to_string_lossy().to_string());

            let (env, actor) =
                candid_parser::typing::pretty_check_file(&expand_path(Path::new(&candid_path)))
                    .unwrap();

            let values = if let Some(values) = canister.init_arg.clone() {
                Some(candid_parser::parse_idl_args(&values)?)
            } else if let Some(path) = canister.init_arg_file.clone() {
                Some(candid_parser::parse_idl_args(&std::fs::read_to_string(
                    path,
                )?)?)
            } else {
                None
            };

            // generate from candid_value
            canister.init_args_rust = candid_value_to_rust::generate_init_values(
                &canister.var_name,
                &env,
                &actor,
                values.as_ref(),
            );

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

pub fn get_generatable_canisters(setup: &IcpTestSetup) -> Vec<CanisterSetup> {
    let canisters: Vec<CanisterSetup> = setup
        .icp_setup
        .canisters
        .iter()
        .filter(|x| x.1.generate_bindings && x.1.wasm.is_some())
        .map(|(canister_name, x)| {
            let mut x = x.clone();

            let wasm = x.wasm.clone().unwrap_or_else(|| {
                panic!("Trying to generate canister {canister_name} with no Wasm defined!")
            });

            let path = Path::new(&wasm);
            let relative = get_path_relative_to_test_dir(path, &setup.test_folder).unwrap();
            x.wasm = Some(relative.to_string_lossy().to_string());
            x
        })
        .collect();
    canisters
}

fn generate_mod_rs(setup: &IcpTestSetup, bindings_path: &Path) -> Result<(), Error> {
    let mut mod_file: PathBuf = bindings_path.to_path_buf();
    mod_file.push("mod.rs");

    let canisters = get_generatable_canisters(setup);

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
