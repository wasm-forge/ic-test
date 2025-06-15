# ic-test

**ic-test** is a command-line tool that helps to set up and manage canister tests on the Internet Computer (IC) using Rust.  
It makes it easier to create test projects and includes the basic files and setup needed for both IC canisters and optionally EVM (Ethereum Virtual Machine) smart contracts.

The tool reads the `dfx.json` (must exist) and the `foundry.toml` (may exist) files in order to build the test environment automatically. It uses `pocket-ic` and `alloy` (foundry) to run tests.
The generated code and helpers provide:

- A simple way to start a test project.
- A single, easy-to-use interface for testing both IC and EVM parts.  
- Type checking and auto-completion support.
- Easy functions for deploying and calling canisters or contracts.


## Overview

**ic-test** will:

- Read `dfx.json` to get canister details.  
- Read `foundry.toml` to get contract details.  
- Generate Rust types from Candid (`.did`) files.  
- Generate contract interfaces from Solidity (`.sol`) files.  
- Provide API to work with `.wasm` canisters and `.json` contract files in tests.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [DFX](https://internetcomputer.org/docs/building-apps/getting-started/install#installing-dfx-via-dfxvm) – to build and locally deploy canisters.
- [Foundry](https://book.getfoundry.sh/getting-started/installation) – optional, if you want to test EVM contract's interaction with canisters.

## Installation

```bash
cargo install ic-test
```

## Tool usage

```bash
ic-test <COMMAND> [OPTIONS]
```

Without arguments it starts in interactive mode to create a new test project. If an `ic-test.json` config file exists already, the "update" mode will regenerate the existing test project bindings.

### Create a new test project

```bash
ic-test new tests
```

- Creates a new test project in the `tests` folder.
- Looks for canisters and contracts, generates API bindings and a sample test.
- Generates an `ic-test.json` configuration file.
- Fails if the `tests` folder already exists, the user would need to choose a different name.


### Update/regenerate an existing test project

```bash
ic-test update
```

Regenerates bindings using the configuration in `ic-test.json`.


## "Hello world" tutorial

*Create a "Hello, World!" canister:*

```bash
dfx new hello-ic-test --type rust --no-frontend
```

*Compile the project:*

```bash
dfx start --clean --background

dfx canister create --all

dfx build
```

*Generate test bindings*

If there are uncommitted changes, either commit them before generating or use the `--force` flag:

```bash
ic-test new tests --force
```

This creates a tests package with:

* Canister API bindings in `tests/src/bindings`
* Test environment setup logic in `test_setup.rs`
* A test template in `tests.rs`


### Example test

*Edit `tests.rs`:*

```rust
use ic_test::IcpTest;

use crate::test_setup;

#[tokio::test]
async fn test_greet() {
    let test_setup::Env {
        icp_test,
        hello_ic_test_backend,
    } = test_setup::setup(IcpTest::new().await).await;

    let result = hello_ic_test_backend
        .greet("ic-test".to_string())
        .call()
        .await;

    assert_eq!(result, "Hello, ic-test!");
}
```

*Run tests:*

```bash
cargo test
```

### Adding a counter

*Update the canister backend:*

```rust
//...

#[derive(Clone, Default)]
struct CounterState {
    value: u64,
    increment: u64,
}

thread_local! {
    static STATE: RefCell<CounterState> = RefCell::new(CounterState::default());
}

#[ic_cdk::init]
fn init(init_value: u64, increment: u64) {
    STATE.with(|state| {
        *state.borrow_mut() = CounterState {
            value: init_value,
            increment,
        };
    });
}

#[ic_cdk::update]
fn increment_counter() {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.value += s.increment;
    });
}

#[ic_cdk::query]
fn get_counter() -> u64 {
    STATE.with(|state| state.borrow().value)
}
```

*Update Candid file `hello-ic-test-backend.did`:*

```candid
service : (nat64, nat64) -> {
  "greet": (text) -> (text) query;
  "get_counter": () -> (nat64) query;
  "increment_counter": () -> ();
}
```

*Set initialization arguments in `dfx.json`:*

```json
{
  "canisters": {
    "hello-ic-test-backend": {
      "candid": "src/hello-ic-test-backend/hello-ic-test-backend.did",
      "package": "hello-ic-test-backend",
      "type": "rust",
      "init_arg": "(50, 73)"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "output_env_file": ".env",
  "version": 1
}
```

*Regenerate the bindings:*

```bash
dfx build

ic-test
```

The `ic-test` will enter interactive mode and prompt user to allow overwriting the `test_setup.rs` file. Upon confirmation the the `test_setup.rs` is regenerated with the initialization parameters:

```rust
//...

let hello_ic_test_backend = hello_ic_test_backend::deploy(&icp_user, 50, 73)
    .call()
    .await;

//...
```

### New test

*Add a new test in `tests.rs`:*

```rust
#[tokio::test]
async fn test_counter() {
    let test_setup::Env {
        icp_test,
        hello_ic_test_backend,
    } = test_setup::setup(IcpTest::new().await).await;

    let result = hello_ic_test_backend.get_counter().call().await;

    assert_eq!(result, 50u64);

    hello_ic_test_backend.increment_counter().call().await;

    let result = hello_ic_test_backend.get_counter().call().await;

    assert_eq!(result, 123u64); // 50 + 73
}
```

### Example of testing an EVM contract

For a more advanced example involving an EVM contract, check out the [Co-processor example](https://github.com/wasm-forge/icp-evm-coprocessor-starter).

```bash
git clone --branch testing https://github.com/letmejustputthishere/icp-evm-coprocessor-starter

dfx build

forge build

cargo test
```
