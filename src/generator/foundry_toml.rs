use std::{
    fs,
    path::{Path, PathBuf},
};

use toml_edit::DocumentMut;

use crate::ic_test_json::{ContractSetup, IcTestSetup};

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
    setup: &mut IcTestSetup,
) -> anyhow::Result<()> {
    if let Some(evm_setup) = &mut setup.evm_setup {
        let mut json = PathBuf::new();

        if let Some(sol_json) = sol_json {
            // json is provided explicitly
            json.push(sol_json);
        } else {
            // try to find the implementation json
            json.push(&evm_setup.foundry_out);
            json.push(format!("{contract_name}.sol/{contract_name}.json"));
        };

        if json.exists() && json.is_file() {
            evm_setup.contracts.insert(
                contract_name.to_string(),
                ContractSetup {
                    name: contract_name.to_string(),
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
pub fn add_contracts(setup: &mut IcTestSetup) -> anyhow::Result<()> {
    if let Some(evm_setup) = &mut setup.evm_setup {
        if !evm_setup.skip_foundry_toml {
            use toml_edit::DocumentMut;

            let toml = fs::read_to_string(crate::common::FOUNDRY_TOML)?;

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
        let path = Path::new(&evm_setup.foundry_src);
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
