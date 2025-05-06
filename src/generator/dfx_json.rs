use std::{collections::HashMap, fs, path::Path};

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
    common::{find_candid, find_wasm},
    ic_test_json::{CanisterSetup, IcpTestSetup},
};

use anyhow::anyhow;

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
    canister_name: &str,
    canister: &DfxCanister,
    setup: &mut IcpTestSetup,
) -> Result<(), anyhow::Error> {
    // skip frontend canisters
    let generate_bindings = !canister_name.ends_with("frontend");

    let candid = find_candid(canister_name, canister).map(|x| x.to_string_lossy().to_string());

    let res = find_wasm(canister_name, canister, setup);

    let wasm = match res {
        Ok(wasm) => wasm,
        Err(_) => {
            // try to rerun dfx build later on
            setup.rerun_dfx_build = true;
            return Ok(());
        }
    };

    let mut canister_setup = CanisterSetup {
        name: canister_name.to_string(),

        init_args_rust: "".to_string(),
        var_name: canister_name.to_case(Case::Snake),
        service_name: format!("{}Canister", canister_name).to_case(Case::Pascal),
        candid_path: candid,
        wasm,
        specified_id: None,
        init_args_path: None,
        generate_bindings,
    };

    canister_setup.specified_id = canister.specified_id.clone();

    // if canister exists already,
    let old_canister = setup.icp_setup.canisters.get(canister_name);

    if let Some(old_canister) = old_canister {
        // reuse old init value if not provided
        if canister_setup.init_args_path.is_none() {
            canister_setup.init_args_path = old_canister.init_args_path.clone();
        }
    }

    let _ = setup
        .icp_setup
        .canisters
        .insert(canister_name.to_string(), canister_setup);

    Ok(())
}

// gather canister information from dfx.json
pub fn add_canisters(setup: &mut IcpTestSetup) -> anyhow::Result<()> {
    let dfx_json_path = Path::new(&setup.icp_setup.dfx_json);

    if !(dfx_json_path.exists() || dfx_json_path.is_file()) {
        return Err(anyhow!("'dfx.json' not found! Make sure you are starting the ic-test at the root of your canister project."));
    }

    if setup.icp_setup.skip_dfx_json {
        return Ok(());
    }

    let dfx_json_path = Path::new(&setup.icp_setup.dfx_json);

    let json_string = fs::read_to_string(dfx_json_path)?;

    let json = from_str::<DfxJson>(&json_string)?;

    if let Some(canisters) = &json.canisters {
        for (canister_name, canister) in canisters {
            add_canister(canister_name, canister, setup)?;
        }

        if setup.ui {
            // list all the canisters and suggest which ones to generate
            let items: Vec<_> = setup
                .icp_setup
                .canisters
                .iter()
                .map(|(name, _canister)| name)
                .collect();

            let defaults: Vec<_> = setup
                .icp_setup
                .canisters
                .iter()
                .map(|(_name, canister)| canister.generate_bindings)
                .collect();

            let selection = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Which canister bindings do you want to generate?")
                .items(&items)
                .defaults(&defaults)
                .interact()
                .unwrap();

            // configure generator selection
            setup.icp_setup.canisters.iter_mut().enumerate().for_each(
                |(idx, (_name, canister))| {
                    canister.generate_bindings = selection.contains(&idx);
                },
            );
        }
    }

    Ok(())
}
