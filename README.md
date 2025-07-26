# ic-test

**ic-test** is a command-line tool that helps to set up and manage Rust canister tests on the Internet Computer (IC) using.
It makes it easier to create a test project and includes the basic files and setup needed for both IC canisters and optionally EVM (Ethereum Virtual Machine) smart contracts.

The tool reads the `dfx.json` (must exist) and the `foundry.toml` (may exist) files in order to build the test environment automatically. It uses `pocket-ic` and `alloy` (foundry) to run tests.
The generated code and helpers provide:

- A simple way to start a test project.
- A single, easy-to-use interface for testing both IC and EVM parts.  
- Type checking and auto-completion support.
- Easy functions for deploying and calling canisters or contracts.

For more information, see the [ic-test Book](https://wasm-forge.github.io/ic-test/).

## Quick Overview

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
- Generates the `ic-test.json` configuration file.
- Fails if the `tests` folder already exists, the user would need to choose a different name.


### Update/regenerate an existing test project

```bash
ic-test update
```

Regenerates bindings using the configuration in `ic-test.json`.


## Examples

For other examples, see https://github.com/wasm-forge/ic-test-examples.

## Licence 

Licensed under <a href="LICENSE">MIT license</a>.