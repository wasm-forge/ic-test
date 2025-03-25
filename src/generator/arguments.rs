use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum AddCommand {
    Canister {
        // Canister name
        name: String,

        // Path to .wasm or .wasm.gz file
        #[arg(long)]
        wasm: Option<String>,
    },
    Contract {
        // Contract name
        name: String,

        // Path to the solidity .json file
        #[arg(long)]
        sol_json: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum Command {
    // Initialize a new test project
    New {
        #[arg(default_value_t = String::from("tests"))]
        test_folder: String,
    },
    // Update the existing test project
    Update {},
    // Add a canister or a contract
    Add {
        /// Choose what you want to add
        #[command(subcommand)]
        command: AddCommand,
    },
}

#[derive(Parser, Debug)]
#[command(version, about=format!("IC Test framework V{}", env!("CARGO_PKG_VERSION")), long_about = None)]
pub struct IcTestArgs {
    // Choose which action you want to perform
    #[command(subcommand)]
    pub command: Command,
    // Path to ic-test.json file
    #[arg(long, default_value_t = String::from("ic-test.json"))]
    pub ic_test_json: String,

    // Do not use dfx.json to gather information on the available canisters
    #[arg(long)]
    pub skip_dfx_json: Option<bool>,

    // Do not use forge.toml to gather information on the available contracts
    #[arg(long)]
    pub skip_forge_toml: Option<bool>,
}
