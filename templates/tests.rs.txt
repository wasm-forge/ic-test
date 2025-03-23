use std::path::PathBuf;

use alloy::{
    hex::FromHex,
    primitives::{utils::parse_ether, Address, Uint, U256},
};
use candid::Principal;
use ic_test::{EvmUser, IcTest, IcUser};

use crate::bindings::{
    chain_fusion::{self, ChainFusionCanister},
    evm::Coprocessor::{self, CoprocessorInstance},
    evm_rpc::{self, EvmRpcCanister},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref WORKSPACE_ROOT: PathBuf = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("Failed to get workspace root")
        .workspace_root
        .into();
}

pub fn wasm(name: &str) -> Vec<u8> {
    let mut path = WORKSPACE_ROOT.clone();
    path.push(".dfx");
    path.push("local");
    path.push("canisters");
    path.push(name);
    path.push(format!("{}.wasm", name));
    std::fs::read(path.as_path()).unwrap_or_else(|_| panic!("wasm binary not found: {:?}", path))
}

pub fn wasm_gz(name: &str) -> Vec<u8> {
    let mut path = WORKSPACE_ROOT.clone();
    path.push(".dfx");
    path.push("local");
    path.push("canisters");
    path.push(name);
    path.push(format!("{}.wasm.gz", name));
    std::fs::read(path.as_path()).unwrap_or_else(|_| panic!("wasm binary not found: {:?}", path))
}

struct Env {
    test: IcTest,
    evm_user: EvmUser,
    chain_fusion: ChainFusionCanister,
    evm_rpc: EvmRpcCanister,
    coprocessor: CoprocessorInstance<(), EvmUser>,
}

async fn setup(test: IcTest) -> Env {
    let evm_user = test.evm.test_user(0);
    let icp_user = test.ic.test_user(0);

    let coprocessor = Coprocessor::deploy(evm_user.clone()).await.unwrap();

    let evm_rpc = evm_rpc::deploy(
        &icp_user,
        evm_rpc::InstallArgs {
            logFilter: None,
            demo: None,
            manageApiKeys: None,
            overrideProvider: None,
            nodesInSubnet: None,
        },
    )
    .with_wasm(wasm_gz("evm_rpc"))
    .with_canister_id(Principal::from_text("7hfb6-caaaa-aaaar-qadga-cai").unwrap())
    .call()
    .await;

    let chain_fusion = chain_fusion::deploy(
        &icp_user,
        chain_fusion::InitArg {
            ecdsa_key_id: chain_fusion::EcdsaKeyId {
                curve: chain_fusion::EcdsaCurve::Secp256K1,
                name: "dfx_test_key".to_string(),
            },
            rpc_service: chain_fusion::RpcService::Custom(chain_fusion::RpcApi {
                url: test.evm.rpc_url().to_string(),
                headers: None,
            }),
            chain_id: test.evm.chain_id(),
            filter_addresses: vec![coprocessor.address().to_string()],
            coprocessor_evm_address: coprocessor.address().to_string(),
            filter_events: vec!["NewJob(uint256)".to_string()],
        },
    )
    .with_wasm(wasm("chain_fusion"))
    .call()
    .await;

    while chain_fusion.get_evm_address().call().await.is_none() {
        test.tick().await;
    }

    let canister_evm_address =
        Address::from_hex(chain_fusion.get_evm_address().call().await.unwrap()).unwrap();

    let receipt = coprocessor
        .updateCoprocessor(canister_evm_address)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    assert!(receipt.status());

    test.evm
        .transfer(
            &evm_user,
            canister_evm_address,
            parse_ether("0.01").unwrap(),
        )
        .await;

    Env {
        test,
        evm_user,
        chain_fusion,
        evm_rpc,
        coprocessor,
    }
}

#[tokio::test]
async fn test_coprocessor_job() {
    let Env {
        test,
        evm_user,
        chain_fusion: _,
        evm_rpc: _,
        coprocessor,
    } = setup(IcTest::new().await).await;

    let user_balance_before = test.evm.get_balance(evm_user.address).await;

    let payment = parse_ether("0.1").unwrap();

    let receipt = coprocessor
        .newJob()
        .value(payment)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    assert!(receipt.status());

    let user_balance_after = test.evm.get_balance(evm_user.address).await;

    // This is not a strict equality because of gas cost payments.
    assert!(user_balance_before - payment >= user_balance_after);

    for _ in 0..100 {
        test.ic.tick().await;
    }

    let result = coprocessor.getResult(Uint::from(0)).call().await.unwrap();
    assert_eq!(result._0, "6765");
}
