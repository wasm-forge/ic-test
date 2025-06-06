use std::{marker::PhantomData, sync::Arc};

use candid::{CandidType, Principal};
use ic_cdk::management_canister::CanisterSettings;
use pocket_ic::nonblocking::PocketIc;
use serde::Deserialize;

use crate::{CallBuilder, CallMode, Caller, DeployBuilder, DeployMode, Deployer};

#[derive(Clone)]
pub struct IcpUser {
    pub principal: Principal,
    pub(crate) pic: Arc<PocketIc>,
}

impl IcpUser {
    pub fn call<ResultType>(
        &self,
        canister_id: Principal,
        call_mode: CallMode,
        method: &str,
        args: Result<Vec<u8>, candid::error::Error>,
    ) -> CallBuilder<ResultType, Self>
    where
        ResultType: for<'a> Deserialize<'a> + CandidType,
    {
        CallBuilder {
            provider: self.clone(),
            canister_id,
            call_mode,
            method: method.to_string(),
            args,
            _result: PhantomData,
        }
    }

    pub fn deploy<Canister>(
        &self,
        args: Result<Vec<u8>, candid::error::Error>,
        new: fn(&Self, Principal) -> Canister,
    ) -> DeployBuilder<Canister, Self> {
        DeployBuilder {
            provider: self.clone(),
            caller: self.clone(),
            canister_id: None,
            mode: DeployMode::Create,
            settings: CanisterSettings::default(),
            cycles: u64::MAX as u128,
            wasm: vec![],
            args,
            new,
        }
    }
}

impl Caller for IcpUser {
    type Provider = IcpUser;

    fn call<ResultType>(
        &self,
        canister_id: Principal,
        call_mode: CallMode,
        method: &str,
        args: Result<Vec<u8>, candid::error::Error>,
    ) -> CallBuilder<ResultType, Self::Provider>
    where
        ResultType: for<'a> Deserialize<'a> + CandidType,
    {
        IcpUser::call(self, canister_id, call_mode, method, args)
    }
}

impl Deployer for IcpUser {
    type Caller = IcpUser;

    fn deploy<Canister>(
        &self,
        args: Result<Vec<u8>, candid::error::Error>,
        new: fn(&Self::Caller, Principal) -> Canister,
    ) -> DeployBuilder<Canister, Self::Caller> {
        IcpUser::deploy(self, args, new)
    }
}
