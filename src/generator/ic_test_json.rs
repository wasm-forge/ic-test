use std::{fs, path::Path};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use super::arguments::IcTestArgs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IcTestSetup {
    // Tests folder: the workspace project for generating tests and bindings
    pub test_folder: String,

    // Path to dfx.json file (default: "dfx.json")
    pub dfx_json: String,

    // Do not use dfx.json to collect information on the existing canisters
    pub skip_dfx_json: bool,

    // Do not use forge.toml to collect information on the existing contracts
    pub skip_forge_toml: bool,

    // Canister setups
    pub canisters: IndexMap<String, CanisterSetup>,

    // ETH contract setup
    pub contracts: IndexMap<String, ContractSetup>,
}

impl Default for IcTestSetup {
    fn default() -> Self {
        Self {
            test_folder: "tests".to_string(),
            dfx_json: "dfx.json".to_string(),
            skip_dfx_json: false,
            skip_forge_toml: false,
            canisters: IndexMap::new(),
            contracts: IndexMap::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanisterSetup {
    pub name: String,
    pub candid: Option<String>,
    pub gen_candid_file: Option<String>, // the candid file used in generator
    pub wasm: String,
    pub specified_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractSetup {
    pub name: String,
    pub sol_json: String,
}

pub fn init_test_config(args: &IcTestArgs) -> anyhow::Result<IcTestSetup> {
    let path = Path::new(&args.ic_test_json);

    let mut setup = if !path.exists() {
        // init with default values
        IcTestSetup::default()
    } else {
        // try opening config from the ic-test.json
        let json_string = fs::read_to_string(&args.ic_test_json)?;

        from_str::<IcTestSetup>(&json_string)?
    };

    if let Some(skip) = args.skip_dfx_json {
        setup.skip_dfx_json = skip;
    }

    if let Some(skip) = args.skip_forge_toml {
        setup.skip_forge_toml = skip;
    }

    if let Some(test_folder) = &args.test_folder {
        setup.test_folder = test_folder.clone();
    }

    Ok(setup)
}

pub fn store_test_config(args: &IcTestArgs, setup: &IcTestSetup) -> anyhow::Result<()> {
    let to_store = serde_json::to_string_pretty(&setup)?;

    fs::write(&args.ic_test_json, to_store)?;

    Ok(())
}
