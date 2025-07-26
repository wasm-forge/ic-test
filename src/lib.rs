//! # ic-test
//!
//! **ic-test** is a command-line tool that helps to set up and manage Rust canister tests on the Internet Computer (IC) using.
//! It makes it easier to create a test project and includes the basic files and setup needed for both IC canisters and optionally EVM (Ethereum Virtual Machine) smart contracts.
//!
//! The tool reads the `dfx.json` (must exist) and the `foundry.toml` (may exist) files in order to build the test environment automatically. It uses `pocket-ic` and `alloy` (Foundry) to run tests.
//! The generated code and helpers provide:
//!
//! - A simple way to start a test project.
//! - A single, easy-to-use interface for testing both IC and EVM parts.  
//! - Type checking and auto-completion support.
//! - Easy functions for deploying and calling canisters or contracts.
//!
//! For more information, see the [ic-test Book](https://wasm-forge.github.io/ic-test/).
//!
//! ## Overview
//!
//! **ic-test** will:
//!
//! - Read `dfx.json` to get canister details.  
//! - Read `foundry.toml` to get contract details.  
//! - Generate Rust types from Candid (`.did`) files.  
//! - Generate contract interfaces from Solidity (`.sol`) files.  
//! - Provide API to work with `.wasm` canisters and `.json` contract files in tests.
//!
//! ## Requirements
//!
//! - [Rust](https://www.rust-lang.org/tools/install)
//! - [DFX](https://internetcomputer.org/docs/building-apps/getting-started/install#installing-dfx-via-dfxvm) – to build and locally deploy canisters.
//! - [Foundry](https://book.getfoundry.sh/getting-started/installation) – optional, if you want to test EVM contract's interaction with canisters.
//!
//! ## Installation
//!
//! ```bash
//! cargo install ic-test
//! ```
//!
//! ## Tool usage
//!
//! ```bash
//! ic-test <COMMAND> [OPTIONS]
//! ```
//!
//! Without arguments it starts in interactive mode to create a new test project. If an `ic-test.json` config file exists already, the "update" mode will regenerate the existing test project bindings.
//!
//! ### Create a new test project
//!
//! ```bash
//! ic-test new tests
//! ```
//!
//! - Creates a new test project in the `tests` folder.
//! - Looks for canisters and contracts, generates API bindings and a sample test.
//! - Generates an `ic-test.json` configuration file.
//! - Fails if the `tests` folder already exists, the user would need to choose a different name.
//!
//! ### Update/regenerate an existing test project
//!
//! ```bash
//! ic-test update
//! ```
//!
//! Regenerates bindings using the configuration in `ic-test.json`.
//!
//! ### Examples
//!
//! For other examples, see <https://github.com/wasm-forge/ic-test-examples>.

#[cfg(feature = "evm")]
use icp::http_outcalls::handle_http_outcalls;
#[cfg(feature = "evm")]
use std::sync::Arc;
#[cfg(feature = "evm")]
use tokio::task;

use candid::{decode_one, encode_one, CandidType};

use serde::Deserialize;

mod icp;

#[cfg(feature = "evm")]
mod evm;
#[cfg(feature = "evm")]
pub use crate::evm::{Evm, EvmUser};

pub use crate::{
    icp::caller::{CallBuilder, CallError, CallMode, Caller},
    icp::deployer::{DeployBuilder, DeployError, DeployMode, Deployer},
    icp::user::IcpUser,
    icp::Icp,
};

/// Helper structure combining test environments
pub struct IcpTest {
    /// Internet Computer environment for canister interactions.
    pub icp: Icp,

    /// EVM testing environment for the EVM start contract interactions.
    #[cfg(feature = "evm")]
    pub evm: Evm,
}

impl IcpTest {
    /// Create a new `IcpTest` instance.
    ///
    /// Initializes the IC environment and, if the `evm` feature is enabled,
    /// also spawns a background task to handle EVM outcalls via Pocket-IC.
    pub async fn new() -> Self {
        let result = Self {
            icp: Icp::new().await,
            #[cfg(feature = "evm")]
            evm: Evm::new(),
        };

        #[cfg(feature = "evm")]
        {
            let pic = Arc::downgrade(&result.icp.pic);

            let urls: Vec<String> = vec![
                "127.0.0.1:8545",
                "localhost:8545",
                "eth.llamarpc.com",
                "sepolia.infura.io/v3/b9794ad1ddf84dfb8c34d6bb5dca2001",
                "rpc.holesky.ethpandaops.io",
                "rpc.hoodi.ethpandaops.io",
                "mainnet.optimism.io",
                "sepolia.optimism.io",
                "arb1.arbitrum.io/rpc",
                "sepolia-rollup.arbitrum.io/rpc",
                "nova.arbitrum.io/rpc",
                "polygon-rpc.com",
                "rpc-amoy.polygon.technology",
                "api.avax.network/ext/bc/C/rpc",
                "api.avax-test.network/ext/bc/C/rpc",
                "bsc-dataseed1.binance.org",
                "rpc.ankr.com/bsc_testnet_chapel",
                "rpc.gnosischain.com",
                "rpc.api.moonbeam.network",
                "rpc.api.moonriver.moonbeam.network",
                "rpc.testnet.moonbeam.network",
                "sepolia.base.org",
                "mainnet.base.org",
                "sepolia.blast.io",
                "rpc.blast.io",
                "rpc.ankr.com/fantom",
                "rpc.ankr.com/fantom_testnet",
                "rpc.frax.com",
                "rpc.testnet.frax.com",
                "bartio.rpc.berachain.com",
                "flare-api.flare.network/ext/C/rpc",
                "coston2-api.flare.network/ext/C/rpc",
                "mode.drpc.org",
                "sepolia.mode.network",
                "zora.drpc.org",
                "sepolia.rpc.zora.energy",
                "racemainnet.io",
                "metall2.drpc.org",
                "testnet.rpc.metall2.com",
                "rpc.zero.thebinaryholdings.com",
                "rpc.orderly.network",
                "testnet-rpc.orderly.org",
            ]
            .into_iter()
            .map(String::from)
            .collect();

            task::spawn(handle_http_outcalls(pic, result.evm.rpc_url(), urls));
        }

        result
    }

    /// Advance both the IC and EVM environments.
    ///
    /// - For IC, triggers a single tick cycle (e.g., canister heartbeat and timer).
    /// - For EVM (if enabled), mines a new block.
    pub async fn tick(&self) {
        self.icp.tick().await;
        #[cfg(feature = "evm")]
        self.evm.mine_block().await;
    }
}

/// Utility function to convert between types via Candid encoding/decoding.
pub fn convert<F, T>(value: F) -> T
where
    F: CandidType,
    T: for<'a> Deserialize<'a> + CandidType,
{
    decode_one(&encode_one(&value).unwrap()).unwrap()
}
