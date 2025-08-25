# Changelog

## [v0.3.4]
- Do not overwrite tests.rs if the tests folder exists

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
- fix more generator errors
- improved documentation
- add derive standard traits to the generator
- support a list of default EVM RPC URLs for automatic forwarding
- fix some issues after switching to the latest alloy


## [v0.1.14]
- set ic-cdk to v0.17.2
- add improvements from v0.2.1
- fix more generator errors
- better test coverage
- better evm support


## [v0.2.1]
- fix: wrong todo! insertions
- add github CI script
- better wasm handling (first select those found in target, then other locations)
- better candid handling (as first priority select the one referenced by the dfx.json, only then try the .dfx location)
- update dependencies
- cleaner generated code


## [v0.2.0]
- switch to alloy 1.0.17
- switch to ic-cdk 0.18.5
- Fix missing evm documentation

