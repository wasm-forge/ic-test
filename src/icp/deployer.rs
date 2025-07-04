//! ## Deployer Trait and DeployBuilder
//!
//! The [`Deployer`] trait provides an ergonomic, configurable interface to create or manage
//! canisters using a builder pattern. It supports creating new canisters, installing/reinstalling/upgrading
//! WASM modules, and injecting arguments and cycles.
//!
//! It’s implemented for types like [`IcpUser`] that fulfill the [`Caller`] trait,
//! and it relies on a [`Provider`] to handle low-level interaction with the IC test environment.
//!
//! The central utility, [`DeployBuilder`], enables step-by-step customization of deployments
//! and safely returns fully-constructed canister instances.

use candid::Principal;
use ic_cdk::management_canister::{CanisterInstallMode, CanisterSettings};
use thiserror::Error;

use super::{
    caller::Caller,
    provider::{Provider, RejectResponse},
};

/// Describes potential errors that can occur during the deployment process.
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

/// Represents the deployment strategy for a canister.
pub enum DeployMode {
    /// Creates and installs a new canister.
    Create,

    /// Installs a fresh WASM on an existing canister.
    Install,

    /// Reinstalls WASM (resetting all state).
    Reinstall,

    /// Upgrades a canister (preserving state).
    Upgrade,
}

/// A type capable of deploying canisters with arguments and lifecycle control.
///
/// Implementors typically use [`DeployBuilder`] to configure and execute deployment logic.
pub trait Deployer {
    type Caller: Caller;

    /// Begins a canister deployment sequence with the given candid-encoded args
    /// and a constructor function for your strongly-typed client.
    fn deploy<Canister>(
        &self,
        args: Result<Vec<u8>, candid::error::Error>,
        new: fn(&Self::Caller, Principal) -> Canister,
    ) -> DeployBuilder<Canister, Self::Caller>;
}

/// Builder struct for configuring and performing a canister deployment.
///
/// Provides an ergonomic way to:
/// - Set the deployment mode (create, install, upgrade, reinstall)
/// - Attach initial cycles
/// - Define WASM module and settings
/// - Inject candid arguments
/// - Produce a typed client interface
pub struct DeployBuilder<Canister, C: Caller> {
    /// Provider that performs actual deployment (e.g. PocketIc).
    pub provider: C::Provider,
    /// The logical caller for interactions post-deployment.
    pub caller: C,
    /// Optional canister ID for pre-existing canisters.
    pub canister_id: Option<Principal>,
    /// Deployment mode (create, install, etc.).
    pub mode: DeployMode,
    /// Canister configuration (controllers, memory allocation, compute allocation, etc.).
    pub settings: CanisterSettings,
    /// Initial cycles to add.
    pub cycles: u128,
    /// WASM module to install.
    pub wasm: Vec<u8>,
    /// Candid-encoded constructor arguments.
    pub args: Result<Vec<u8>, candid::error::Error>,
    /// Function to wrap a raw `Principal` in a user-defined canister type.
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

    /// Execute the deployment, returning either a constructed canister interface or an error.
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

    /// Execute deployment, assuming it should not fail. Panics if deployment fails.
    pub async fn call(self) -> Canister {
        self.maybe_call().await.unwrap()
    }
}
