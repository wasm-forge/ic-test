use std::{collections::HashMap, fs, path::Path};

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
    common::find_wasm,
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

    #[serde(rename = "type")]
    pub canister_type: Option<String>,

    pub dependencies: Option<Vec<String>>,

    pub specified_id: Option<String>,
}

pub fn find_candid(canister_name: &str, _canister: &DfxCanister) -> Option<String> {
    // 1. try finding the local file
    // if let Some(candid) = &canister.candid {
    //     let cached_did_path = Path::new(&candid);

    //     if cached_did_path.exists() && cached_did_path.is_file() {
    //         return Some(candid.clone());
    //     }
    // }

    // 2. try using dfx cached .did file
    // TODO: which .did file is the correct one?
    let cached_did_string = format!(".dfx/local/canisters/{canister_name}/constructor.did");
    let cached_did_path = Path::new(&cached_did_string);

    if cached_did_path.exists() && cached_did_path.is_file() {
        return Some(cached_did_string);
    }

    None
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
            // prepare canister
            let candid = find_candid(canister_name, canister);
            println!("found candid: {:?}", candid);

            let wasm = find_wasm(canister_name, setup)?;

            let mut canister_setup = CanisterSetup {
                name: canister_name.clone(),
                service_name: format!("{}Canister", canister_name).to_case(Case::Pascal),
                candid,
                wasm,
                specified_id: None,
            };

            //if let Some(candid) = &canister.candid {
            //    canister_setup.candid = Some(candid.clone());
            //}

            canister_setup.specified_id = canister.specified_id.clone();

            // store new canister setup
            let _ = setup
                .icp_setup
                .canisters
                .insert(canister_name.clone(), canister_setup);
        }
    }

    Ok(())
}
