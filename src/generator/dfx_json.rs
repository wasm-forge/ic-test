use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::ic_test_json::{CanisterSetup, IcTestSetup};

use super::arguments::IcTestArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct DfxJson {
    pub canisters: Option<HashMap<String, DfxCanister>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DfxCanister {
    pub candid: Option<String>,

    pub init_arg_file: Option<String>,

    pub init_arg: Option<String>,

    pub package: Option<String>,

    pub wasm: Option<String>,

    #[serde(rename = "type")]
    pub canister_type: Option<String>,

    pub dependencies: Option<Vec<String>>,

    pub specified_id: Option<String>,
}

pub fn get_gen_candid_file(canister_name: &str, canister: &DfxCanister) -> Option<String> {
    // 1. try finding the local file
    if let Some(candid) = &canister.candid {
        let cached_did_path = Path::new(&candid);

        if cached_did_path.exists() && cached_did_path.is_file() {
            return Some(candid.clone());
        }
    }

    // 2. try using dfx cached .did file
    // TODO: which .did file is the correct one?
    let cached_did_string = format!(".dfx/local/canisters/{canister_name}/constructor.did");
    let cached_did_path = Path::new(&cached_did_string);

    if cached_did_path.exists() && cached_did_path.is_file() {
        return Some(cached_did_string);
    }

    None
}

pub fn parse_dfx_json(args: &IcTestArgs, setup: &mut IcTestSetup) -> anyhow::Result<()> {
    let json_string = fs::read_to_string(&args.dfx_json)?;

    //    let json: serde_json::Value =
    //        serde_json::from_str(&json_string).expect("JSON was not well-formatted");

    let json = from_str::<DfxJson>(&json_string)?;

    if let Some(canisters) = &json.canisters {
        for (canister_name, canister) in canisters {
            let gen_candid_file = get_gen_candid_file(canister_name, canister);

            if let Some(candid) = &canister.candid {
                let canister_setup = CanisterSetup {
                    canister_name: canister_name.clone(),
                    candid: Some(candid.clone()),
                    gen_candid_file,
                    wasm: canister.wasm.clone(),
                    specified_id: None,
                };

                let _ = setup
                    .canisters
                    .insert(canister_name.clone(), canister_setup);
            }
        }
    }

    Ok(())
}
