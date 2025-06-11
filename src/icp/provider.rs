//! ## Provider Trait for Canister Management
//!
//! The [`Provider`] trait defines an asynchronous interface for managing and interacting with
//! Internet Computer canisters in a test environment. It abstracts over actions such as creating
//! canisters, adding cycles, installing code, and performing method calls (query and update).
//!
//! This trait is implemented by [`IcpUser`] using [`PocketIc`], making it possible to simulate full
//! canister lifecycles during testing. These interactions are wrapped in type-safe, ergonomic
//! methods that surface consistent error reporting via `RejectResponse`.

use ic_cdk::management_canister::{CanisterId, CanisterInstallMode, CanisterSettings};
pub use pocket_ic::{ErrorCode, RejectCode, RejectResponse};

use crate::convert;

use super::IcpUser;

/// Trait defining low-level operations for IC canister management and inter-canister communication.
///
/// This trait is used internally by `ic-test` to provide a backend-agnostic interface for canister
/// control, making it pluggable across test environments (e.g., PocketIc, remote agents).
#[allow(async_fn_in_trait)]
pub trait Provider {
    /// Creates a new canister with the given settings and (optionally) a specified ID.
    async fn create_canister(
        &self,
        settings: CanisterSettings,
        specified_id: Option<CanisterId>,
    ) -> Result<CanisterId, RejectResponse>;

    /// Adds the specified number of cycles to a canister.
    async fn add_cycles(
        &self,
        canister_id: CanisterId,
        cycles: u128,
    ) -> Result<u128, RejectResponse>;

    /// Installs, reinstalls, or upgrades the given canister code, with arguments.
    async fn install_code(
        &self,
        mode: CanisterInstallMode,
        canister_id: CanisterId,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
    ) -> Result<(), RejectResponse>;

    /// Executes a query method on a canister.
    async fn query_call(
        &self,
        canister_id: CanisterId,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, RejectResponse>;

    /// Executes an update method on a canister.
    async fn update_call(
        &self,
        canister_id: CanisterId,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, RejectResponse>;
}

/// Implementation of the [`Provider`] trait for [`IcpUser`].
///
/// Uses a shared [`PocketIc`] instance to manage and interact with canisters in a simulated test environment.
/// This enables full lifecycle management including creation, cycle handling, code installation, and method execution.
impl Provider for IcpUser {
    async fn create_canister(
        &self,
        settings: CanisterSettings,
        specified_id: Option<CanisterId>,
    ) -> Result<CanisterId, RejectResponse> {
        let settings = convert(settings);
        let id = match specified_id {
            Some(id) => {
                self.pic
                    .create_canister_with_id(Some(self.principal), Some(settings), id)
                    .await
                    .map_err(|err| RejectResponse {
                        reject_code: RejectCode::CanisterReject,
                        reject_message: err,
                        error_code: ErrorCode::UnknownManagementMessage,
                        certified: false,
                    })?;
                id
            }
            None => {
                self.pic
                    .create_canister_with_settings(Some(self.principal), Some(settings))
                    .await
            }
        };
        Ok(id)
    }

    async fn add_cycles(
        &self,
        canister_id: CanisterId,
        cycles: u128,
    ) -> Result<u128, RejectResponse> {
        Ok(self.pic.add_cycles(canister_id, cycles).await)
    }

    async fn install_code(
        &self,
        mode: CanisterInstallMode,
        canister_id: CanisterId,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
    ) -> Result<(), RejectResponse> {
        match mode {
            CanisterInstallMode::Install => {
                self.pic
                    .install_canister(canister_id, wasm_module, arg, Some(self.principal))
                    .await;
                Ok(())
            }
            CanisterInstallMode::Reinstall => {
                self.pic
                    .reinstall_canister(canister_id, wasm_module, arg, Some(self.principal))
                    .await
            }
            CanisterInstallMode::Upgrade(_upgrade_flags) => {
                self.pic
                    .upgrade_canister(canister_id, wasm_module, arg, Some(self.principal))
                    .await
            }
        }
    }

    async fn query_call(
        &self,
        canister_id: CanisterId,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, RejectResponse> {
        self.pic
            .query_call(canister_id, self.principal, method, payload)
            .await
    }

    async fn update_call(
        &self,
        canister_id: CanisterId,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, RejectResponse> {
        self.pic
            .update_call(canister_id, self.principal, method, payload)
            .await
    }
}
