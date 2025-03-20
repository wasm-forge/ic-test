use clap::{Parser, ValueEnum};

#[derive(Parser, Clone, Debug, ValueEnum)]
pub enum TestType {
    Icp,
    IcpEvm,
}

#[derive(Parser, Debug)]
#[command(version, about=format!("IC Test framework V{}", env!("CARGO_PKG_VERSION")), long_about = None)]
pub struct IcTestArgs {
    /// Test folder to create
    #[arg(long, short, default_value_t = String::from("tests"))]
    pub test_folder: String,

    /// Path to dfx.json file
    #[arg(long, default_value_t = String::from("dfx.json"))]
    pub dfx_json: String,

    /// Generate all canisters in dfx.json?
    #[arg(long, short = 'j', default_value_t = true)]
    pub use_dfx_json: bool,

    /// Generate EVM tests?
    #[arg(long, default_value_t = false)]
    pub generate_evm_tests: bool,

    /// Generate EVM contracts?
    #[arg(long, default_value_t = false)]
    pub evm_contracts: bool,

    /// Comma-separated canister names
    #[arg(long, default_value_t = String::from(""))]
    pub canister_names: String,

    /// Comma-separated test types for each canister (icp,icp-evm)
    #[arg(long, default_value_t = String::from(""))]
    pub test_types: String,
}
