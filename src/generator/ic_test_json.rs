use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Ok;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
    arguments,
    common::search_file_recursively,
    dfx_json::add_canisters,
    foundry_toml::{add_contract, add_contracts},
};

use super::arguments::IcpTestArgs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvmSetup {
    // Path to foundry.toml file (default: "")
    #[serde(default)]
    pub foundry_toml_path: String,

    // Do not use foundry.toml to collect information on the existing contracts
    #[serde(default)]
    pub skip_foundry_toml: bool,

    // Path to found "foundry src" containing contract .sol files
    #[serde(default)]
    pub foundry_src: String,

    // Path to found "foundry out" containing contract Solidity json files
    #[serde(default)]
    pub foundry_out: String,

    // ETH contracts setup
    #[serde(default)]
    pub contracts: BTreeMap<String, ContractSetup>,
}

impl EvmSetup {
    pub fn get_foundry_toml(&self) -> PathBuf {
        PathBuf::new()
            .join(self.foundry_toml_path.clone())
            .join(crate::common::FOUNDRY_TOML)
    }

    pub fn get_foundry_src(&self) -> PathBuf {
        PathBuf::new()
            .join(self.foundry_toml_path.clone())
            .join(self.foundry_src.clone())
    }

    pub fn get_foundry_out(&self) -> PathBuf {
        PathBuf::new()
            .join(self.foundry_toml_path.clone())
            .join(self.foundry_out.clone())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IcpSetup {
    // Path to dfx.json file (default: "dfx.json")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dfx_json: Option<String>,

    // Do not use dfx.json to collect information on the existing canisters
    pub skip_dfx_json: bool,

    // Canister setups
    pub canisters: BTreeMap<String, CanisterSetup>,
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
    pub test_setup_rs_regenerated: bool,

    // ICP settings
    pub icp_setup: IcpSetup,

    // EVM settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_setup: Option<EvmSetup>,
}

impl Default for IcpSetup {
    fn default() -> Self {
        Self {
            dfx_json: Some("dfx.json".to_string()),
            skip_dfx_json: false,
            canisters: BTreeMap::new(),
        }
    }
}

impl Default for EvmSetup {
    fn default() -> Self {
        Self {
            foundry_toml_path: "".to_string(),
            foundry_src: "src".to_string(),
            foundry_out: "out".to_string(),
            skip_foundry_toml: false,
            contracts: BTreeMap::new(),
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
            test_setup_rs_regenerated: false,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candid_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_arg_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_arg: Option<String>,

    #[serde(default)]
    pub generate_bindings: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasm: Option<String>,
    pub specified_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractSetup {
    pub name: String,
    pub var_name: String,
    pub sol_json: String,
}

pub fn init_test_config(args: &IcpTestArgs) -> anyhow::Result<IcpTestSetup> {
    let ic_test_json_path = Path::new(&args.ic_test_json);

    let mut setup = if !ic_test_json_path.exists() {
        // init with default values
        IcpTestSetup::default()
    } else {
        // try opening config from the ic-test.json
        let json_string = fs::read_to_string(&args.ic_test_json)?;

        from_str::<IcpTestSetup>(&json_string)?
    };

    if let Some(_evm) = &mut setup.evm_setup {
        //
    } else {
        // search for foundry.toml
        debug!("Searching for foundry.toml...");
        // we need to decide if we want to work with EVM, by default it depends on whether we can find the foundry.toml file
        let foundry_toml = search_file_recursively(Path::new("."), 3, crate::common::FOUNDRY_TOML);

        // TODO: if the foundry.toml file was not found, ask user for an explicit path from user
        // TODO: if multiple files were found, allow user to select one
        // TODO: support option for the foundry.toml file path

        if let Some(path) = &foundry_toml {
            debug!("foundry.toml found: {path:?}");
        } else {
            debug!("foundry.toml not found!");
        }

        if let Some(f) = foundry_toml {
            let path = f.parent().expect("could not find foundry.toml parent");
            let evm = EvmSetup {
                foundry_toml_path: path.to_string_lossy().to_string(),
                ..EvmSetup::default()
            };

            setup.evm_setup = Some(evm);
        }
    }

    setup.ui = args.ui == Some(true);

    if let Some(dfx_json) = &args.dfx_json {
        if dfx_json.is_empty() {
            setup.icp_setup.dfx_json = None;
        } else {
            setup.icp_setup.dfx_json = Some(dfx_json.to_owned());
        }
    }

    if let Some(skip) = args.skip_dfx_json {
        setup.icp_setup.skip_dfx_json = skip;
    }

    if let Some(evm_setup) = &mut setup.evm_setup {
        if let Some(skip) = args.skip_foundry_toml {
            evm_setup.skip_foundry_toml = skip;
        }
    }

    // Do setup initializations

    debug!("automatically adding canisters...");
    add_canisters(&mut setup)?;

    debug!("automatically adding contracts...");
    add_contracts(&mut setup)?;

    debug!("processing command {:?}", &args.command);

    match &args.command {
        arguments::Command::New { test_folder, force } => {
            setup.test_folder = test_folder.clone();
            setup.forced = *force;
        }

        arguments::Command::Update {
            force,
            name,
            wasm,
            init_arg_file,
            init_arg,
            sol_json,
        } => {
            setup.forced = *force;

            if let Some(name) = name {
                let canister = setup.icp_setup.canisters.get_mut(name);

                if let Some(canister) = canister {
                    if let Some(init_arg_file) = init_arg_file {
                        if init_arg_file.is_empty() {
                            canister.init_arg_file = None;
                        } else {
                            canister.init_arg_file = Some(init_arg_file.clone());
                        }
                    }

                    if let Some(init_arg) = init_arg {
                        if init_arg.is_empty() {
                            canister.init_arg_file = None;
                        } else {
                            canister.init_arg_file = Some(init_arg.clone());
                        }
                    }

                    if let Some(wasm) = wasm {
                        canister.wasm = Some(wasm.clone());
                    }
                }

                if let Some(evm_setup) = &mut setup.evm_setup {
                    let contract = evm_setup.contracts.get_mut(name);

                    if let Some(contract) = contract {
                        if let Some(sol_json) = sol_json {
                            contract.sol_json = sol_json.clone();
                        }
                    }
                }
            }
        }

        arguments::Command::Add { command } => {
            // either add a canister or a contract to the setup
            match command {
                arguments::AddCommand::Canister {
                    name,
                    wasm: _,
                    init_arg_file: _,
                    init_arg: _,
                } => {
                    info!("Adding canister {name}");

                    todo!("Adding a canister is currently not supported. To add one manually, modify the ic-test.json file directly.");
                }
                arguments::AddCommand::Contract { name, sol_json } => {
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
