use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::arguments::IcTestArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct DfxJson {
    pub canisters: Option<HashMap<String, JsonCanister>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCanister {
    pub candid: Option<String>,

    pub init_arg_file: Option<String>,

    pub package: Option<String>,

    pub wasm: Option<String>,

    #[serde(rename = "type")]
    pub canister_type: Option<String>,

    pub dependencies: Option<Vec<String>>,

    pub specified_id: Option<String>,
}

pub struct CanisterSetup {
    pub canister_name: String,
    pub candid: Option<String>,
    pub wasm: Option<String>,
    pub specified_id: Option<String>,
}

pub fn parse_dfx_json(args: &IcTestArgs) -> anyhow::Result<Vec<CanisterSetup>> {
    let json_string = fs::read_to_string(&args.dfx_json)?;

    //    let json: serde_json::Value =
    //        serde_json::from_str(&json_string).expect("JSON was not well-formatted");

    let json = from_str::<DfxJson>(&json_string)?;

    let mut res = Vec::new();

    if let Some(canisters) = json.canisters {
        for (canister_name, canister) in canisters {
            res.push(CanisterSetup {
                canister_name,
                candid: canister.candid,
                wasm: canister.wasm,
                specified_id: canister.specified_id,
            })
        }
    }

    Ok(res)
}
