# ic-test

**ic-test** is a command-line tool that helps you organize the test workflow for cross-chain projects on the Internet Computer (IC). It reads your `dfx.json` and `foundry.toml` files to automatically create testing base and uses existing frameworks (`pocket-ic` and `foundry`) to execute tests.

## Overview

ic-test will:
- Read `dfx.json` to gather canister information.  
- Read `foundry.toml` to gather contract information.  
- Generate Rust type definitions from Candid (`.did`) files.  
- Generate a contract interface from `.sol` definitions.  
- Create access API to use existing `.wasm` canisters and `.json` contracts for testing.


## Requirements
- [Rust](https://www.rust-lang.org/)
- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/build/install/) for local IC canister builds.
- [Foundry](https://book.getfoundry.sh/getting-started/installation) if your tests involve EVM contracts.


## Installation

You can install the tool via Cargo:

```bash
cargo install ic-test
```

## Tool usage

Use **ic-test** by running one of its commands:

```bash
ic-test <COMMAND> [OPTIONS]
```

### Create a new test project

```bash
ic-test new tests
```

- Creates a new test project in the `tests` folder.
- Looks for your canisters and contracts, then generates the necessary API bindings and a sample test file.
- Also creates an `ic-test.json` file to store generator configuration for future runs.
- Fails if the `tests` folder already exists.


### Update/regenerate an existing test project

```bash
ic-test update
```

Reruns the generator based on the configuration in `ic-test.json`.


### Manually add canister or contract

For a given Solidity contract name if will try to find its json implementation. Example:

```bash
ic-test add contract MyContract
```


## License

This project is licensed under the MIT License. Please see the LICENSE file in this repository for more details.
