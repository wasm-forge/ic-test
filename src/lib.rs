//! # ic-test
//!
//! **ic-test** is a command-line tool that helps to set up and manage Rust canister tests on the Internet Computer (IC) using.
//! It makes it easier to create a test project and includes the basic files and setup needed for both IC canisters and optionally EVM (Ethereum Virtual Machine) smart contracts.
//!
//! The tool reads the `dfx.json` (must exist) and the `foundry.toml` (may exist) files in order to build the test environment automatically. It uses `pocket-ic` and `alloy` (foun/dry) to run tests.
//! The generated code and helpers provide:
//!
//! - A simple way to start a test project.
//! - A single, easy-to-use interface for testing both IC and EVM parts.  
//! - Type checking and auto-completion support.
//! - Easy functions for deploying and calling canisters or contracts.
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
//! ## "Hello world" tutorial
//!
//! *Create a "Hello, World!" canister:*
//!
//! ```bash
//! dfx new hello-ic-test --type rust --no-frontend
//! ```
//!
//! *Compile the project:*
//!
//! ```bash
//! dfx start --clean --background
//!
//! dfx canister create --all
//!
//! dfx build
//! ```
//!
//! *Generate test bindings*
//!
//! If there are uncommitted changes, either commit them before generating or use the `--force` flag:
//!
//! ```bash
//! ic-test new tests --force
//! ```
//!
//! This creates a tests package with:
//!
//! * Canister API bindings in `tests/src/bindings`
//! * Test environment setup logic in `test_setup.rs`
//! * A test template in `tests.rs`
//!
//! ### Example test
//!
//! *Edit `tests.rs`:*
//!
//! ```rust
//! use ic_test::IcpTest;
//!
//! #[tokio::test]
//! async fn test_greet() {
//!     let test_setup::Env {
//!         icp_test,
//!         hello_ic_test_backend,
//!     } = test_setup::setup(IcpTest::new().await).await;
//!
//!     let result = hello_ic_test_backend
//!         .greet("ic-test".to_string())
//!         .call()
//!         .await;
//!
//!     assert_eq!(result, "Hello, ic-test!");
//! }
//! ```
//!
//! *Run tests:*
//!
//! ```bash
//! cargo test
//! ```
//!
//! ### Adding a counter
//!
//! *Update the canister backend:*
//!
//! ```rust
//! use std::cell::RefCell;
//!
//! #[ic_cdk::query]
//! fn greet(name: String) -> String {
//!     format!("Hello, {}!", name)
//! }
//!
//! #[derive(Clone, Default)]
//! struct CounterState {
//!     value: u64,
//!     increment: u64,
//! }
//!
//! thread_local! {
//!     static STATE: RefCell<CounterState> = RefCell::new(CounterState::default());
//! }
//!
//! #[ic_cdk::init]
//! fn init(init_value: u64, increment: u64) {
//!     STATE.with(|state| {
//!         *state.borrow_mut() = CounterState {
//!             value: init_value,
//!             increment,
//!         };
//!     });
//! }
//!
//! #[ic_cdk::update]
//! fn increment_counter() {
//!     STATE.with(|state| {
//!         let mut s = state.borrow_mut();
//!         s.value += s.increment;
//!     });
//! }
//!
//! #[ic_cdk::query]
//! fn get_counter() -> u64 {
//!     STATE.with(|state| state.borrow().value)
//! }
//! ```
//!
//! *Update Candid file `hello-ic-test-backend.did`:*
//!
//! ```candid
//! service : (nat64, nat64) -> {
//!   "greet": (text) -> (text) query;
//!   "get_counter": () -> (nat64) query;
//!   "increment_counter": () -> ();
//! }
//! ```
//!
//! *Set initialization arguments in `dfx.json`:*
//!
//! ```json
//! {
//!   "canisters": {
//!     "hello-ic-test-backend": {
//!       "candid": "src/hello-ic-test-backend/hello-ic-test-backend.did",
//!       "package": "hello-ic-test-backend",
//!       "type": "rust",
//!       "init_arg": "(50, 73)"
//!     }
//!   },
//!   "defaults": {
//!     "build": {
//!       "args": "",
//!       "packtool": ""
//!     }
//!   },
//!   "output_env_file": ".env",
//!   "version": 1
//! }
//! ```
//!
//! *Regenerate the bindings:*
//!
//! ```bash
//! dfx build
//!
//! ic-test
//! ```
//!
//! The `ic-test` will enter interactive mode and prompt user to allow overwriting the `test_setup.rs` file. Upon confirmation the the `test_setup.rs` is regenerated with the initialization parameters:
//!
//! ```rust
//!
//! # mod hello_ic_test_backend {
//! #     type Caller = ic_test::IcpUser;
//! #     type DeployMode = ic_test::DeployMode;
//! #     type Deployer = ic_test::IcpUser;
//! #     type DeployBuilder<C> = ic_test::DeployBuilder<C, Caller>;
//! #     pub fn deploy(user: &str, a: u64, b: u64) -> DeployBuilder<String> {
//! #         panic!();
//! #     }
//! # }
//!   //...
//!   # async fn dummy_fn() {
//!   let icp_user = "";
//!   let hello_ic_test_backend = hello_ic_test_backend::deploy(&icp_user, 50, 73)
//!       .call()
//!       .await;
//!   # }
//!   // ...
//!
//! ```
//!
//! ### New test
//!
//! *Add a new test in `tests.rs`:*
//!
//! ```rust
//! // ...
//! #[tokio::test]
//! async fn test_counter() {
//!     let test_setup::Env {
//!         icp_test,
//!         hello_ic_test_backend,
//!     } = test_setup::setup(IcpTest::new().await).await;
//!
//!     let result = hello_ic_test_backend.get_counter().call().await;
//!
//!     assert_eq!(result, 50u64);
//!
//!     hello_ic_test_backend.increment_counter().call().await;
//!
//!     let result = hello_ic_test_backend.get_counter().call().await;
//!
//!     assert_eq!(result, 123u64); // 50 + 73
//! }
//! ```
//!
//! ### More examples
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
    /// Internet Computer environment for canister interaction.
    pub icp: Icp,

    /// EVM testing environment, only available when the `evm` feature is enabled.
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
        let pic = Arc::downgrade(&result.icp.pic);

        #[cfg(feature = "evm")]
        task::spawn(handle_http_outcalls(
            pic,
            result.evm.rpc_url(),
            vec![result.evm.rpc_url().to_string()],
        ));
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
