use std::{fs, path::Path};

use indexmap::IndexMap;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
    arguments,
    dfx_json::add_canisters,
    foundry_toml::{add_contract, add_contracts},
};

use super::arguments::IcpTestArgs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvmSetup {
    // Path to foundry.toml file (default: "foundry.toml")
    pub foundry_toml: String,

    // Do not use foundry.toml to collect information on the existing contracts
    pub skip_foundry_toml: bool,

    // Path to found "foundry src" containing contract .sol files
    pub foundry_src: String,

    // Path to found "foundry out" containing contract Solidity json files
    pub foundry_out: String,

    // ETH contracts setup
    pub contracts: IndexMap<String, ContractSetup>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IcpSetup {
    // Path to dfx.json file (default: "dfx.json")
    pub dfx_json: String,

    // Do not use dfx.json to collect information on the existing canisters
    pub skip_dfx_json: bool,

    // Canister setups
    pub canisters: IndexMap<String, CanisterSetup>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IcpTestSetup {
    // Tests folder: the workspace project for generating tests and bindings
    pub test_folder: String,

    #[serde(skip)]
    pub forced: bool,

    #[serde(skip)]
    pub regenerate_cargo: bool,

    #[serde(skip)]
    pub is_complete: bool,

    #[serde(skip)]
    pub ui: bool,

    #[serde(skip)]
    pub tests_rs_regenerated: bool,

    #[serde(skip)]
    pub rerun_dfx_build: bool,

    // ICP settings
    pub icp_setup: IcpSetup,

    // EVM settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_setup: Option<EvmSetup>,
}

impl Default for IcpSetup {
    fn default() -> Self {
        Self {
            dfx_json: "dfx.json".to_string(),
            skip_dfx_json: false,
            canisters: IndexMap::new(),
        }
    }
}

impl Default for EvmSetup {
    fn default() -> Self {
        Self {
            foundry_toml: crate::common::FOUNDRY_TOML.to_string(),
            foundry_src: "src".to_string(),
            foundry_out: "out".to_string(),
            skip_foundry_toml: false,
            contracts: IndexMap::new(),
        }
    }
}

impl Default for IcpTestSetup {
    fn default() -> Self {
        Self {
            test_folder: "tests".to_string(),
            forced: false,
            regenerate_cargo: false,
            icp_setup: IcpSetup::default(),
            evm_setup: None,
            is_complete: false,
            rerun_dfx_build: false,
            tests_rs_regenerated: false,
            ui: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanisterSetup {
    pub name: String,
    pub var_name: String,
    pub service_name: String,

    #[serde(skip)]
    pub init_args_rust: String,
    pub candid_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_args_path: Option<String>,

    #[serde(default)]
    pub generate_bindings: bool,

    pub wasm: String,
    pub specified_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractSetup {
    pub name: String,
    pub var_name: String,
    pub sol_json: String,
}

pub fn init_test_config(args: &IcpTestArgs) -> anyhow::Result<IcpTestSetup> {
    let path = Path::new(&args.ic_test_json);

    let mut setup = if !path.exists() {
        // init with default values
        let mut setup = IcpTestSetup::default();

        // we need to decide if we want to work with EVM, by default it depends on whether we can find the foundry.toml file
        let foundry_toml = Path::new(crate::common::FOUNDRY_TOML);

        if foundry_toml.exists() && foundry_toml.is_file() {
            setup.evm_setup = Some(EvmSetup::default());
        }

        setup
    } else {
        // try opening config from the ic-test.json
        let json_string = fs::read_to_string(&args.ic_test_json)?;

        from_str::<IcpTestSetup>(&json_string)?
    };

    setup.ui = args.ui == Some(true);

    if let Some(skip) = args.skip_dfx_json {
        setup.icp_setup.skip_dfx_json = skip;
    }

    if let Some(evm_setup) = &mut setup.evm_setup {
        if let Some(skip) = args.skip_foundry_toml {
            evm_setup.skip_foundry_toml = skip;
        }
    }

    // Do setup initializations
    add_canisters(&mut setup)?;

    add_contracts(&mut setup)?;

    match &args.command {
        arguments::Command::New { test_folder } => {
            setup.test_folder = test_folder.clone();
        }

        arguments::Command::Update { force, command } => {
            setup.forced = *force;

            if let Some(command) = command {
                match command {
                    arguments::UpdateCommand::Canister {
                        name,
                        wasm,
                        init_args_path,
                    } => {
                        let canister = setup.icp_setup.canisters.get_mut(name);

                        if let Some(canister) = canister {
                            if let Some(init_args_path) = init_args_path {
                                if init_args_path.is_empty() {
                                    canister.init_args_path = None;
                                } else {
                                    canister.init_args_path = Some(init_args_path.clone());
                                }
                            }

                            if let Some(wasm) = wasm {
                                canister.wasm = wasm.clone();
                            }
                        } else {
                            return Err(anyhow::anyhow!(format!("Canister '{}' not found!", name)));
                        }
                    }
                    arguments::UpdateCommand::Contract {
                        name: _,
                        sol_json: _,
                    } => {}
                }
            }
        }

        arguments::Command::Add { command } => {
            // either add a canister or a contract to the setup
            match command {
                arguments::AddCommand::Canister {
                    name,
                    wasm: _,
                    init_args_path: _,
                } => {
                    info!("Adding canister {name}");

                    todo!("Adding a canister is currently not supported. To add one manually, modify the ic-test.json file directly.");
                }
                arguments::AddCommand::Contract { name, sol_json } => {
                    info!("Adding contract {name}");
                    add_contract(name, sol_json, &mut setup)?;
                }
            }
        }
    }

    Ok(setup)
}

pub fn store_test_config(args: &IcpTestArgs, setup: &IcpTestSetup) -> anyhow::Result<()> {
    let to_store = serde_json::to_string_pretty(&setup)?;

    fs::write(&args.ic_test_json, to_store)?;

    Ok(())
}
