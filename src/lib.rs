#![doc = include_str!("../README.md")]

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

/// Helper structure combining test environments
pub struct IcpTest {
    /// Internet Computer environment for canister interaction.
    pub icp: Icp,

    /// EVM testing environment, only available when the `evm` feature is enabled.
    #[cfg(feature = "evm")]
    pub evm: Evm,
}

impl IcpTest {
    /// Create a new `IcpTest` instance.
    ///
    /// Initializes the IC environment and, if the `evm` feature is enabled,
    /// also spawns a background task to handle EVM outcalls via Pocket-IC.
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

    /// Advance both the IC and EVM environments.
    ///
    /// - For IC, triggers a single tick cycle (e.g., canister heartbeat and timer).
    /// - For EVM (if enabled), mines a new block.
    pub async fn tick(&self) {
        self.icp.tick().await;
        #[cfg(feature = "evm")]
        self.evm.mine_block().await;
    }
}

/// Utility function to convert between types via Candid encoding/decoding.
pub fn convert<F, T>(value: F) -> T
where
    F: CandidType,
    T: for<'a> Deserialize<'a> + CandidType,
{
    decode_one(&encode_one(&value).unwrap()).unwrap()
}
