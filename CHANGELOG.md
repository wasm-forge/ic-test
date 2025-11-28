# Changelog

## [v0.4.0]
- Switch to pocket-ic 10.0.0
- Switch to ic-cdk 0.19.0

## [v0.3.4]
- Fix incorrect function call generator when function names are long

## [v0.3.3]
- Do not overwrite tests.rs if the tests folder exists

## [v0.3.2]
- Use candid v0.10.16
- Introduce temporary solution for the incorrect concat


## [v0.3.1]
- Fix showing version number
- test no_args


## [v0.3.0]
- Stay with ic-cdk v0.17.2
- Fix more generator errors
- Improved documentation
- Add derive standard traits to the generator
- Support a list of default EVM RPC URLs for automatic forwarding
- Fix some issues after switching to the latest alloy


## [v0.1.14]
- Set ic-cdk to v0.17.2
- Add improvements from v0.2.1
- Fix more generator errors
- Better test coverage
- Better EVM support


## [v0.2.1]
- Fix: wrong todo! insertions
- Add github CI script
- Better wasm handling (first select those found in target, then other locations)
- Better candid handling (as first priority select the one referenced by the dfx.json, only then try the .dfx location)
- Update dependencies
- Cleaner generated code


## [v0.2.0]
- Switch to alloy 1.0.17
- Switch to ic-cdk 0.18.5
- Fix missing EVM documentation

