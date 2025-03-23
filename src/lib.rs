use std::sync::Arc;

use candid::{decode_one, encode_one, CandidType};
use ic::http_outcalls::handle_http_outcalls;
use serde::Deserialize;
use tokio::task;

mod evm;
mod ic;

pub use crate::{
    evm::{Evm, EvmUser},
    ic::caller::{CallBuilder, CallError, CallMode, Caller},
    ic::deployer::{DeployBuilder, DeployError, DeployMode, Deployer},
    ic::user::IcUser,
    ic::Ic,
};

pub struct IcTest {
    pub ic: Ic,
    pub evm: Evm,
}

impl IcTest {
    pub async fn new() -> Self {
        let result = Self {
            ic: Ic::new().await,
            evm: Evm::new(),
        };

        let pic = Arc::downgrade(&result.ic.pic);
        task::spawn(handle_http_outcalls(
            pic,
            result.evm.rpc_url(),
            vec![result.evm.rpc_url().to_string()],
        ));
        result
    }

    pub async fn tick(&self) {
        self.ic.tick().await;
        self.evm.mine_block().await;
    }
}

pub fn convert<F, T>(value: F) -> T
where
    F: CandidType,
    T: for<'a> Deserialize<'a> + CandidType,
{
    decode_one(&encode_one(&value).unwrap()).unwrap()
}
