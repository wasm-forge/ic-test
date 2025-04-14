use std::sync::Arc;

use candid::{decode_one, encode_one, CandidType};
use icp::http_outcalls::handle_http_outcalls;
use serde::Deserialize;
use tokio::task;

#[cfg(feature = "evm")]
mod evm;

#[cfg(not(feature = "evm"))]
mod dummy_evm;

#[cfg(not(feature = "evm"))]
use dummy_evm as evm;


mod icp;
pub use crate::evm::{Evm, EvmUser};


pub use crate::{
    icp::caller::{CallBuilder, CallError, CallMode, Caller},
    icp::deployer::{DeployBuilder, DeployError, DeployMode, Deployer},
    icp::user::IcpUser,
    icp::Icp,
};

pub struct IcpTest {
    pub icp: Icp,
    pub evm: Evm,
}

impl IcpTest {
    pub async fn new() -> Self {
        let result = Self {
            icp: Icp::new().await,
            evm: Evm::new(),
        };

        let pic = Arc::downgrade(&result.icp.pic);
        task::spawn(handle_http_outcalls(
            pic,
            result.evm.rpc_url(),
            vec![result.evm.rpc_url().to_string()],
        ));
        result
    }

    pub async fn tick(&self) {
        self.icp.tick().await;
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
