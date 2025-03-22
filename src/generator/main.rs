mod arguments;
mod candid_to_rust;
mod dependencies;
mod dfx_json;
mod ic_test_json;

use std::path::Path;

use arguments::IcTestArgs;
use clap::Parser;
use dfx_json::parse_dfx_json;
use ic_test_json::{init_test_config, store_test_config, ContractSetup, IcTestSetup};

fn process_arguments(args: &IcTestArgs, setup: &mut IcTestSetup) -> anyhow::Result<()> {
    // TODO: always parse dfx.json?
    parse_dfx_json(args, setup)?;

    // parse added EVM contracts
    for contract_json_path in &args.add_sol_json {
        let path = Path::new(contract_json_path);

        if let Some(stem) = path.file_stem() {
            let name = stem.to_string_lossy().into_owned();

            setup.contracts.insert(
                name.clone(),
                ContractSetup {
                    name,
                    sol_json: contract_json_path.clone(),
                },
            );
        }
    }

    // TODO: always try to generate?
    candid_to_rust::generate(setup)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = IcTestArgs::try_parse()?;

    let mut setup = init_test_config(&args)?;

    process_arguments(&args, &mut setup)?;

    store_test_config(&args, &setup)?;

    Ok(())
}
