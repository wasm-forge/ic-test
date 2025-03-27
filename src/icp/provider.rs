use ic_cdk::api::management_canister::main::{CanisterId, CanisterInstallMode, CanisterSettings};
pub use pocket_ic::{ErrorCode, RejectCode, RejectResponse};

use crate::convert;

use super::IcpUser;

#[allow(async_fn_in_trait)]
pub trait Provider {
    async fn create_canister(
        &self,
        settings: CanisterSettings,
        specified_id: Option<CanisterId>,
    ) -> Result<CanisterId, RejectResponse>;

    async fn add_cycles(
        &self,
        canister_id: CanisterId,
        cycles: u128,
    ) -> Result<u128, RejectResponse>;

    async fn install_code(
        &self,
        mode: CanisterInstallMode,
        canister_id: CanisterId,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
    ) -> Result<(), RejectResponse>;

    async fn query_call(
        &self,
        canister_id: CanisterId,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, RejectResponse>;

    async fn update_call(
        &self,
        canister_id: CanisterId,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, RejectResponse>;
}

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
