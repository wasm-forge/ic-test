use std::{collections::HashMap, fs, path::Path};

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
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
    canister_name: &str,
    canister: &DfxCanister,
    setup: &mut IcpTestSetup,
) -> Result<(), anyhow::Error> {
    // skip frontend canisters
    let generate_bindings = canister.canister_type != Some("asset".to_string());

    let candid = find_candid(canister_name, canister).map(|x| x.to_string_lossy().to_string());

    let wasm = find_wasm(canister_name, canister, setup)?;

    let mut canister_setup = CanisterSetup {
        name: canister_name.to_string(),

        init_args_rust: "".to_string(),
        var_name: canister_name.to_case(Case::Snake),
        service_name: format!("{canister_name}Canister").to_case(Case::Pascal),
        candid_path: candid,
        wasm,
        specified_id: None,
        init_arg_file: canister.init_arg_file.clone(),
        init_arg: canister.init_arg.clone(),
        generate_bindings,
    };

    canister_setup.specified_id = canister.specified_id.clone();

    // if canister exists already,
    let old_canister = setup.icp_setup.canisters.get(canister_name);

    if let Some(old_canister) = old_canister {
        // reuse old init values if not provided
        if canister_setup.init_arg_file.is_none() {
            canister_setup.init_arg_file = old_canister.init_arg_file.clone();
        }

        if canister_setup.init_arg.is_none() {
            canister_setup.init_arg = old_canister.init_arg.clone();
        }
    }

    let _ = setup
        .icp_setup
        .canisters
        .insert(canister_name.to_string(), canister_setup);

    Ok(())
}

fn check_dfx_json_exists(setup: &mut IcpTestSetup) -> anyhow::Result<()> {
    // check if dfx.json is found
    let dfx_json_path = Path::new(&setup.icp_setup.dfx_json);

    if !(dfx_json_path.exists() || dfx_json_path.is_file()) {
        return Err(anyhow::anyhow!("'dfx.json' not found! Make sure you are starting the ic-test at the root of your canister project."));
    }

    Ok(())
}

// gather canister information from dfx.json
pub fn add_canisters(setup: &mut IcpTestSetup) -> anyhow::Result<()> {
    if setup.icp_setup.skip_dfx_json {
        return Ok(());
    }

    check_dfx_json_exists(setup)?;

    let dfx_json_path = Path::new(&setup.icp_setup.dfx_json);
    let json_string = fs::read_to_string(dfx_json_path)?;
    let json = from_str::<DfxJson>(&json_string)?;

    if let Some(canisters) = &json.canisters {
        if canisters.is_empty() {
            return Err(anyhow::anyhow!(
                "No canisters were found in the 'dfx.json' file!"
            ));
        }

        // list all the canisters and suggest which ones to generate
        let items: Vec<_> = setup.icp_setup.canisters.keys().collect();

        // first, select canisters to generate, only then actually try to add those (check their Wasm file etc)
        let selection = if setup.ui {
            let defaults: Vec<_> = setup
                .icp_setup
                .canisters
                .values()
                .map(|canister| canister.generate_bindings)
                .collect();

            if defaults.is_empty() {
                return Err(anyhow::anyhow!(
                    "No compatible canisters were found in the 'dfx.json' file!"
                ));
            }

            MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Which canister bindings do you want to generate?")
                .items(&items)
                .defaults(&defaults)
                .interact()
                .unwrap()
        } else {
            // by default mark all canisters as selected
            items.iter().enumerate().map(|(idx, _x)| idx).collect()
        };

        // configure generator selection
        setup
            .icp_setup
            .canisters
            .iter_mut()
            .enumerate()
            .for_each(|(idx, (_name, canister))| {
                canister.generate_bindings = selection.contains(&idx);
            });

        // add canisters that have been selected
        for (canister_name, canister) in canisters {
            if setup.icp_setup.canisters.contains_key(canister_name) {
                add_canister(canister_name, canister, setup)?;
            }
        }
    }

    Ok(())
}
