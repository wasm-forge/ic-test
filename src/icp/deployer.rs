use candid::Principal;
use ic_cdk::management_canister::{CanisterInstallMode, CanisterSettings};
use thiserror::Error;

use super::{
    caller::Caller,
    provider::{Provider, RejectResponse},
};

#[derive(Debug, Error)]
pub enum DeployError {
    #[error("failed to candid encode arguments: {}", .0)]
    ArgumentEncoding(candid::error::Error),
    #[error("canister rejected: {}, error_code: {}", .0.reject_message, .0.error_code)]
    Reject(RejectResponse),
    #[error("failed to candid decode result: {}", .0)]
    ResultDecoding(candid::error::Error),
    #[error("canister creation failed: {}", .0)]
    CreateCanister(String),
    #[error("canister id is missing")]
    UnspecifiedCanister,
}

pub enum DeployMode {
    Create,
    Install,
    Reinstall,
    Upgrade,
}

pub trait Deployer {
    type Caller: Caller;

    fn deploy<Canister>(
        &self,
        args: Result<Vec<u8>, candid::error::Error>,
        new: fn(&Self::Caller, Principal) -> Canister,
    ) -> DeployBuilder<Canister, Self::Caller>;
}

pub struct DeployBuilder<Canister, C: Caller> {
    pub provider: C::Provider,
    pub caller: C,
    pub canister_id: Option<Principal>,
    pub mode: DeployMode,
    pub settings: CanisterSettings,
    pub cycles: u128,
    pub wasm: Vec<u8>,
    pub args: Result<Vec<u8>, candid::error::Error>,
    pub new: fn(&C, Principal) -> Canister,
}

impl<Canister, C: Caller> DeployBuilder<Canister, C> {
    pub fn with_canister_id(self, canister_id: Principal) -> Self {
        Self {
            canister_id: Some(canister_id),
            ..self
        }
    }

    pub fn with_controllers(self, controllers: Vec<Principal>) -> Self {
        Self {
            settings: CanisterSettings {
                controllers: Some(controllers.clone()),
                ..self.settings
            },
            ..self
        }
    }

    pub fn with_cycles(self, cycles: u128) -> Self {
        Self { cycles, ..self }
    }

    pub fn with_settings(self, settings: CanisterSettings) -> Self {
        Self { settings, ..self }
    }

    pub fn with_wasm(self, wasm: Vec<u8>) -> Self {
        Self { wasm, ..self }
    }

    pub fn with_install(self) -> Self {
        Self {
            mode: DeployMode::Install,
            ..self
        }
    }

    pub fn with_upgrade(self) -> Self {
        Self {
            mode: DeployMode::Upgrade,
            ..self
        }
    }

    pub fn with_reinstall(self) -> Self {
        Self {
            mode: DeployMode::Reinstall,
            ..self
        }
    }

    pub async fn maybe_call(self) -> Result<Canister, DeployError> {
        let args = self.args.map_err(DeployError::ArgumentEncoding)?;

        let canister_id = if let DeployMode::Create = self.mode {
            self.provider
                .create_canister(self.settings, self.canister_id)
                .await
                .map_err(DeployError::Reject)?
        } else {
            match self.canister_id {
                Some(canister_id) => canister_id,
                None => {
                    return Err(DeployError::UnspecifiedCanister);
                }
            }
        };

        self.provider
            .add_cycles(canister_id, self.cycles)
            .await
            .map_err(DeployError::Reject)?;

        let mode = match self.mode {
            DeployMode::Create | DeployMode::Install => CanisterInstallMode::Install,
            DeployMode::Reinstall => CanisterInstallMode::Reinstall,
            DeployMode::Upgrade => CanisterInstallMode::Upgrade(None),
        };

        self.provider
            .install_code(mode, canister_id, self.wasm, args)
            .await
            .map_err(DeployError::Reject)?;

        Ok((self.new)(&self.caller, canister_id))
    }

    pub async fn call(self) -> Canister {
        self.maybe_call().await.unwrap()
    }
}
