# EVM RPC Integration

In this examples we'll explore testing a canister that uses integration with the Ethereum smart contracts and will see how one can create canister, EVM or the hybrid tests.

For a quick start, clone the [`ic-test-examples`](https://github.com/wasm-forge/ic-test-examples) repository and enter the `eth-balance` project:
```bash
git clone https://github.com/wasm-forge/ic-test-examples.git
cd ic-test-examples/eth-balance
```

In the cloned examples repository enter the project `eth-balance`. It is a basic implementation of a canister that connects to an EVM-RPC service and requests for a current Eth balance on any Ethereum address. You can try deploy it and see that the canister actually works ( and start `dfx` if it not running already):

```bash
dfx start --background --clean
dfx deploy
```

