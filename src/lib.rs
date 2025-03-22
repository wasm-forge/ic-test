use candid::{decode_one, encode_one, CandidType};
use serde::Deserialize;

mod evm;
mod ic;

pub use crate::{
    evm::{Evm, EvmUser},
    ic::caller::{CallBuilder, CallError, CallMode, Caller},
    ic::deployer::{DeployBuilder, DeployError, DeployMode, Deployer},
    ic::{Ic, IcUser},
};

pub struct IcTest {
    pub ic: Ic,
    pub evm: Evm,
}

impl IcTest {
    pub async fn new() -> Self {
        Self {
            ic: Ic::new().await,
            evm: Evm::new(),
        }
    }
}

pub fn convert<F, T>(value: F) -> T
where
    F: CandidType,
    T: for<'a> Deserialize<'a> + CandidType,
{
    decode_one(&encode_one(&value).unwrap()).unwrap()
}
