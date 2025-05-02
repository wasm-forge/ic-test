#[cfg(feature = "evm")]
use icp::http_outcalls::handle_http_outcalls;
#[cfg(feature = "evm")]
use std::sync::Arc;
#[cfg(feature = "evm")]
use tokio::task;

use candid::{decode_one, encode_one, CandidType};

use serde::Deserialize;

mod icp;

#[cfg(feature = "evm")]
mod evm;
#[cfg(feature = "evm")]
pub use crate::evm::{Evm, EvmUser};

pub use crate::{
    icp::caller::{CallBuilder, CallError, CallMode, Caller},
    icp::deployer::{DeployBuilder, DeployError, DeployMode, Deployer},
    icp::user::IcpUser,
    icp::Icp,
};

pub struct IcpTest {
    pub icp: Icp,
    #[cfg(feature = "evm")]
    pub evm: Evm,
}

impl IcpTest {
    pub async fn new() -> Self {
        let result = Self {
            icp: Icp::new().await,
            #[cfg(feature = "evm")]
            evm: Evm::new(),
        };

        #[cfg(feature = "evm")]
        let pic = Arc::downgrade(&result.icp.pic);

        #[cfg(feature = "evm")]
        task::spawn(handle_http_outcalls(
            pic,
            result.evm.rpc_url(),
            vec![result.evm.rpc_url().to_string()],
        ));
        result
    }

    pub async fn tick(&self) {
        self.icp.tick().await;
        #[cfg(feature = "evm")]
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
