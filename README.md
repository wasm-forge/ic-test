# ic-test

**ic-test** is a command-line tool that helps you organize the test workflow for cross-chain projects on the Internet Computer (IC). It uses existing frameworks (`pocket-ic` and `foundry`) to read your `dfx.json` and `foundry.toml` files, then creates a test setup that works in both environments.

## Overview

ic-test will:
- Read `dfx.json` to gather canister information.  
- Read `foundry.toml` to gather contract information.  
- Generate Rust type definitions from Candid (`.did`) files.  
- Generate a contract interface from `.sol` definitions.  
- Create access API to use existing `.wasm` canisters and `.json` contracts for testing.


## Requirements
- [Rust](https://www.rust-lang.org/) (1.83+ recommended)  
- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/build/install/) for local IC canister builds.
- [Foundry](https://book.getfoundry.sh/getting-started/installation) if your tests involve EVM contracts.


## Installation

Because **ic-test** is still under early development, you need to build it yourself. You will need both the `ic-test` source code and a modified version of `cdk-rs`:

```bash
git clone https://github.com/wasm-forge/ic-test.git
git clone https://github.com/wasm-forge/cdk-rs.git
```

Compile **ic-test**, then add it to your `PATH` or create a symbolic link so you can run it from your project folder.

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
