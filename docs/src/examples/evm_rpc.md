# EVM RPC tutorial

In this examples we'll explore testing a canister integration with the Ethereum smart contracts.

For a quick start, clone the ic-test examples repository:
```bash
git clone https://github.com/wasm-forge/ic-test-examples.git
```

In the cloned examples folder, enter the project `eth-balance`.

The `eth-balance-backend` is a basic implementation of a canister that connects to the EVM-RPC canister and requests for a current Eth balance on any address.

