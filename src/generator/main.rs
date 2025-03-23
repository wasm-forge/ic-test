mod arguments;
mod candid_to_rust;
mod common;
mod dependencies;
mod dfx_json;
mod ic_test_json;
mod test_structure;

use arguments::IcTestArgs;
use clap::Parser;
use dfx_json::{parse_dfx_json, parse_forge_toml};
use ic_test_json::{init_test_config, store_test_config, IcTestSetup};

fn process_arguments(args: &IcTestArgs, setup: &mut IcTestSetup) -> anyhow::Result<()> {
    if !setup.skip_dfx_json {
        parse_dfx_json(setup)?;
    }

    if !setup.skip_forge_toml {
        parse_forge_toml(setup)?;
    }

    match &args.command {
        arguments::Command::Init {} => {}
        arguments::Command::Update {} => {}
        arguments::Command::Add { command } => {
            // either add a canister or a contract to the setup
            match command {
                arguments::AddCommand::Canister { name, wasm } => {
                    println!("ADDING canister {name}");
                }
                arguments::AddCommand::Contract { name, sol_json } => {
                    println!("ADDING contract {name}");

                    /*
                    // parse provided EVM contracts, add those to setup
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
                    */
                }
            }
        }
    }

    ///////////////////////////////////
    // Generate / regenerate folders

    // generate structure
    test_structure::generate(setup, true)?;

    // generate candid
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
