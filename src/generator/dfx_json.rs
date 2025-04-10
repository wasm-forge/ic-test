use std::{collections::HashMap, fs};

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
    common::{find_candid, find_wasm},
    ic_test_json::{CanisterSetup, IcpTestSetup},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DfxJson {
    pub canisters: Option<HashMap<String, DfxCanister>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DfxCanister {
    pub init_arg_file: Option<String>,

    pub init_arg: Option<String>,

    pub package: Option<String>,

    pub wasm: Option<String>,

    pub candid: Option<String>,

    #[serde(rename = "type")]
    pub canister_type: Option<String>,

    pub dependencies: Option<Vec<String>>,

    pub id: Option<String>,

    pub specified_id: Option<String>,
}

pub fn add_canister(
    canister_name: &String,
    canister: &DfxCanister,
    setup: &mut IcpTestSetup,
) -> Result<(), anyhow::Error> {
    let candid = find_candid(canister_name, canister).map(|x| x.to_string_lossy().to_string());
    let wasm = find_wasm(canister_name, canister, setup)?;
    let mut canister_setup = CanisterSetup {
        name: canister_name.clone(),
        var_name: canister_name.to_case(Case::Snake),
        service_name: format!("{}Canister", canister_name).to_case(Case::Pascal),
        candid,
        wasm,
        specified_id: None,
    };
    canister_setup.specified_id = canister.specified_id.clone();
    let _ = setup
        .icp_setup
        .canisters
        .insert(canister_name.clone(), canister_setup);
    Ok(())
}

// gather canister information from dfx.json
pub fn add_canisters(setup: &mut IcpTestSetup) -> anyhow::Result<()> {
    if setup.icp_setup.skip_dfx_json {
        return Ok(());
    }

    let json_string = fs::read_to_string(&setup.icp_setup.dfx_json)?;
    let json = from_str::<DfxJson>(&json_string)?;

    if let Some(canisters) = &json.canisters {
        for (canister_name, canister) in canisters {
            add_canister(canister_name, canister, setup)?;
        }
    }

    Ok(())
}
