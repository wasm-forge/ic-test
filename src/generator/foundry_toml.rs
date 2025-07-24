use std::{fs, path::PathBuf};

use convert_case::{Case, Casing};
use log::{debug, info};
use toml_edit::DocumentMut;

use crate::ic_test_json::{ContractSetup, IcpTestSetup};

fn get_toml_src(doc: &DocumentMut) -> Option<String> {
    doc.get("profile")?
        .get("default")?
        .get("src")?
        .as_str()
        .map(|x| x.to_owned())
}

fn get_toml_out(doc: &DocumentMut) -> Option<String> {
    doc.get("profile")?
        .get("default")?
        .get("out")?
        .as_str()
        .map(|x| x.to_owned())
}

pub fn add_contract(
    contract_name: &str,
    sol_json: &Option<String>,
    setup: &mut IcpTestSetup,
) -> anyhow::Result<()> {
    info!("Adding contract {contract_name}");

    if let Some(evm_setup) = &mut setup.evm_setup {
        let mut json = PathBuf::new();

        if let Some(sol_json) = sol_json {
            // json is provided explicitly
            json.push(sol_json);
        } else {
            // try to find the implementation json
            json.push(evm_setup.get_foundry_out());
            json.push(format!("{contract_name}.sol/{contract_name}.json"));
        };

        if json.exists() && json.is_file() {
            let var_name = contract_name.to_case(Case::Snake);

            evm_setup.contracts.insert(
                contract_name.to_string(),
                ContractSetup {
                    name: contract_name.to_string(),
                    var_name,
                    sol_json: json.to_string_lossy().to_string(),
                },
            );
        }
    } else {
        // TODO: ERROR?
    }

    Ok(())
}

// gather contract information from foundry.toml
pub fn add_contracts(setup: &mut IcpTestSetup) -> anyhow::Result<()> {
    if let Some(evm_setup) = &mut setup.evm_setup {
        if !evm_setup.skip_foundry_toml {
            use toml_edit::DocumentMut;

            let foundry_toml = &evm_setup.get_foundry_toml();
            debug!("Foundry path: {foundry_toml:?}");

            let toml = fs::read_to_string(foundry_toml)?;

            // paths are relative to the toml path
            let doc = toml
                .parse::<DocumentMut>()
                .expect("Failed to parse foundry.toml");

            let src = get_toml_src(&doc);
            if let Some(path) = src {
                evm_setup.foundry_src = path;
            }

            let out = get_toml_out(&doc);
            if let Some(path) = out {
                evm_setup.foundry_out = path;
            }
        }

        // add all contracts from "src" to the setup
        let path = &evm_setup.get_foundry_src();
        debug!("Check foundry src path: {path:?}");
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let file_path = entry.path();

                if let Some(contract_name) = file_path
                    .extension()
                    .filter(|ext| ext.to_str() == Some("sol"))
                    .and_then(|_| file_path.file_stem())
                    .and_then(|stem| stem.to_str())
                {
                    add_contract(contract_name, &None, setup)?;
                }
            }
        }
    }

    Ok(())
}
