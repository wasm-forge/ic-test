//! Represents a user (principal) in the PocketIC test environment, capable of deploying and
//! interacting with Internet Computer (IC) canisters.
//!
//! This abstraction enables writing tests from the perspective of a particular identity.
//! It implements both the [`Caller`] and [`Deployer`] traits for use in IC test scenarios.

use std::{marker::PhantomData, sync::Arc};

use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::CanisterSettings;
use pocket_ic::nonblocking::PocketIc;
use serde::Deserialize;

use crate::{CallBuilder, CallMode, Caller, DeployBuilder, DeployMode, Deployer};

/// A simulated Internet Computer user for use in tests.
///
/// Provides a simplified API for calling methods and deploying canisters.
/// Wraps a `Principal` identity and a reference to the [`PocketIc`] test environment.
#[derive(Clone)]
pub struct IcpUser {
    /// The user's principal identity used for calls and deployments.
    pub principal: Principal,

    /// Shared reference to the underlying PocketIC instance.
    pub(crate) pic: Arc<PocketIc>,
}

impl IcpUser {
    /// Prepare a canister method call builder for the user.
    ///
    /// # Parameters
    /// - `canister_id`: The principal ID of the target canister.
    /// - `call_mode`: Whether the call is a query or an update.
    /// - `method`: The name of the method to invoke.
    /// - `args`: Encoded Candid arguments or an error.
    ///
    /// # Returns
    /// A [`CallBuilder`] for the specified method, result type, and context.    
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

    /// Prepare a new canister deployment as this user.
    ///
    /// # Parameters
    /// - `args`: Encoded Candid arguments for the canister constructor, or an error.
    /// - `new`: A function to instantiate the canister wrapper after deployment.
    ///
    /// # Returns
    /// A [`DeployBuilder`] for the specified canister.
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

    /// Dispatch a call via this user, required by the [`Caller`] trait.
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

    /// Dispatch a deployment via this user, required by the [`Deployer`] trait.
    fn deploy<Canister>(
        &self,
        args: Result<Vec<u8>, candid::error::Error>,
        new: fn(&Self::Caller, Principal) -> Canister,
    ) -> DeployBuilder<Canister, Self::Caller> {
        IcpUser::deploy(self, args, new)
    }
}
