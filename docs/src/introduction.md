# Introduction

**ic-test** is a command-line tool that helps to set up and manage Rust canister tests on the Internet Computer (IC).
The tool makes it easier to create a test project and includes the basic files and setup needed for both IC canisters and optionally EVM (Ethereum Virtual Machine) smart contracts.

The tool reads the `dfx.json` (must exist) and the `foundry.toml` (may exist) files in order to build the test environment automatically. It uses `pocket-ic` and `alloy` (foundry) to run tests.
The generated code and helpers provide:

- A simple way to start a test project.
- A single, easy-to-use interface for testing IC Canisters and EVM smart contracts.
- Type checking and auto-completion support.
- Easy functions for deploying and calling canisters or contracts.


## Overview

**ic-test** will:

- Read `dfx.json` to get canister details.  
- Read `foundry.toml` to get contract details.  
- Generate Rust types from Candid (`.did`) files.  
- Generate contract interfaces from Solidity (`.sol`) files.  
- Provide API to work with `.wasm` canisters and `.json` contract files in tests.


