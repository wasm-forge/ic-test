//! Defines the call mechanism for interacting with canisters in tests.
//!
//! This includes the [`CallBuilder`] for chaining test call setup, the [`CallError`] enum for
//! error handling, and the [`Caller`] trait for types that can initiate calls.

use std::marker::PhantomData;

use candid::{decode_one, CandidType, Principal};
use serde::Deserialize;
use thiserror::Error;

use super::provider::{Provider, RejectResponse};

/// Errors that can occur during a canister method call.
#[derive(Debug, Error)]
pub enum CallError {
    /// Error during argument encoding (Candid serialization).
    #[error("failed to candid encode call arguments: {}", .0)]
    ArgumentEncoding(candid::error::Error),

    /// The canister rejected the call, providing a rejection message and error code.
    #[error("canister rejected: {}, error_code: {}", .0.reject_message, .0.error_code)]
    Reject(RejectResponse),

    /// Error during decoding the response (Candid deserialization).
    #[error("failed to candid decode call result: {}", .0)]
    ResultDecoding(candid::error::Error),
}

/// Indicates whether the call is a query (read-only) or update (state-changing).
pub enum CallMode {
    Query,
    Update,
}

/// Trait for objects that can initiate canister calls.
///
/// Implemented by user types like [`IcpUser`], allowing polymorphic handling of principals.
pub trait Caller {
    type Provider: Provider;

    /// Initiates a call to a canister method.
    ///
    /// # Parameters
    /// - `canister_id`: The target canister's principal.
    /// - `call_mode`: Whether this is a query or update.
    /// - `method`: Method name to call.
    /// - `args`: Encoded Candid arguments or error.
    ///
    /// # Returns
    /// A configured [`CallBuilder`] to execute the call.
    fn call<ResultType>(
        &self,
        canister_id: Principal,
        call_mode: CallMode,
        method: &str,
        args: Result<Vec<u8>, candid::error::Error>,
    ) -> CallBuilder<ResultType, Self::Provider>
    where
        ResultType: for<'a> Deserialize<'a> + CandidType;
}

/// A builder for creating and executing canister method calls in tests.
///
/// Generic over:
/// - `R`: The expected result type of the method.
/// - `P`: The provider type (e.g., `IcpUser` or PocketIC).
pub struct CallBuilder<R: for<'a> Deserialize<'a> + CandidType, P: Provider> {
    /// The test environment provider.
    pub provider: P,

    /// The principal of the canister to call.
    pub canister_id: Principal,

    /// The mode of the call (query or update).
    pub call_mode: CallMode,

    /// The name of the method being called.
    pub method: String,

    /// Candid-encoded call arguments, or an encoding error.
    pub args: Result<Vec<u8>, candid::error::Error>,

    /// Phantom type to carry the result type without storing it.
    pub _result: PhantomData<R>,
}

impl<R: for<'a> Deserialize<'a> + CandidType, P: Provider> CallBuilder<R, P> {
    /// Setup caller of the bulider
    pub fn with_caller<C: Caller>(self, caller: C) -> CallBuilder<R, C::Provider> {
        caller.call::<R>(self.canister_id, self.call_mode, &self.method, self.args)
    }

    /// Switch caller mode to update
    pub fn with_update(self) -> Self {
        Self {
            call_mode: CallMode::Update,
            ..self
        }
    }

    /// Executes the call and returns a `Result` with decoded output or [`CallError`].
    ///
    /// # Errors
    /// Returns a [`CallError`] if encoding, calling, or decoding fails.
    pub async fn maybe_call(self) -> Result<R, CallError> {
        let args = self.args.map_err(CallError::ArgumentEncoding)?;

        let result = match self.call_mode {
            CallMode::Query => {
                self.provider
                    .query_call(self.canister_id, &self.method, args)
                    .await
            }
            CallMode::Update => {
                self.provider
                    .update_call(self.canister_id, &self.method, args)
                    .await
            }
        };

        let reply = result.map_err(CallError::Reject)?;

        decode_one(&reply).map_err(CallError::ResultDecoding)
    }

    /// Executes the call and unwraps the result.
    ///
    /// Panics if the call fails — suitable for tests where failure is not expected.
    ///
    /// # Panics
    /// Panics if any [`CallError`] occurs.
    pub async fn call(self) -> R {
        self.maybe_call().await.unwrap()
    }
}
