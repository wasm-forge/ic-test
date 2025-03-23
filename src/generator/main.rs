mod arguments;
mod candid_to_rust;
mod common;
mod dependencies;
mod dfx_json;
mod foundry_toml;
mod ic_test_json;
mod test_structure;

use arguments::IcTestArgs;
use clap::Parser;
use dfx_json::add_canisters;
use foundry_toml::{add_contract, add_contracts};
use ic_test_json::{init_test_config, store_test_config, IcTestSetup};

fn process_arguments(args: &IcTestArgs, setup: &mut IcTestSetup) -> anyhow::Result<()> {
    add_canisters(setup)?;

    add_contracts(setup)?;

    match &args.command {
        arguments::Command::Init {} => {}
        arguments::Command::Update {} => {}
        arguments::Command::Add { command } => {
            // either add a canister or a contract to the setup
            match command {
                arguments::AddCommand::Canister { name, wasm: _ } => {
                    println!("ADDING canister {name}");
                    // TODO
                }
                arguments::AddCommand::Contract { name, sol_json } => {
                    println!("ADDING contract {name}");
                    add_contract(name, sol_json, setup)?;
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
