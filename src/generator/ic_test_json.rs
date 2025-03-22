use std::{fs, path::Path};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use super::arguments::IcTestArgs;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct IcTestSetup {
    pub canisters: IndexMap<String, CanisterSetup>,
    pub contracts: IndexMap<String, ContractSetup>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanisterSetup {
    pub canister_name: String,
    pub candid: Option<String>,
    pub gen_candid_file: Option<String>, // the candid file used in generator
    pub wasm: Option<String>,
    pub specified_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractSetup {
    pub name: String,
    pub sol_json: String,
}

pub fn init_test_config(args: &IcTestArgs) -> anyhow::Result<IcTestSetup> {
    let path = Path::new(&args.ic_test_json);

    if !path.exists() {
        // init with default values
        return Ok(IcTestSetup::default());
    }

    // try opening config from the ic-test.json
    let json_string = fs::read_to_string(&args.ic_test_json)?;

    let json = from_str::<IcTestSetup>(&json_string)?;

    Ok(json)
}

pub fn store_test_config(args: &IcTestArgs, setup: &IcTestSetup) -> anyhow::Result<()> {
    let to_store = serde_json::to_string_pretty(&setup)?;

    fs::write(&args.ic_test_json, to_store)?;

    Ok(())
}
