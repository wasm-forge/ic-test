use std::marker::PhantomData;

use candid::{decode_one, CandidType, Principal};
use serde::Deserialize;
use thiserror::Error;

use super::provider::{Provider, RejectResponse};

#[derive(Debug, Error)]
pub enum CallError {
    #[error("failed to candid encode call arguments: {}", .0)]
    ArgumentEncoding(candid::error::Error),
    #[error("canister rejected: {}, error_code: {}", .0.reject_message, .0.error_code)]
    Reject(RejectResponse),
    #[error("failed to candid decode call result: {}", .0)]
    ResultDecoding(candid::error::Error),
}

pub enum CallMode {
    Query,
    Update,
}

pub trait Caller {
    type Provider: Provider;

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

pub struct CallBuilder<R: for<'a> Deserialize<'a> + CandidType, P: Provider> {
    pub provider: P,
    pub canister_id: Principal,
    pub call_mode: CallMode,
    pub method: String,
    pub args: Result<Vec<u8>, candid::error::Error>,
    pub _result: PhantomData<R>,
}

impl<R: for<'a> Deserialize<'a> + CandidType, P: Provider> CallBuilder<R, P> {
    pub fn with_caller<C: Caller>(self, caller: C) -> CallBuilder<R, C::Provider> {
        caller.call::<R>(self.canister_id, self.call_mode, &self.method, self.args)
    }

    pub fn with_update(self) -> Self {
        Self {
            call_mode: CallMode::Update,
            ..self
        }
    }

    pub async fn call(self) -> Result<R, CallError> {
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
}
