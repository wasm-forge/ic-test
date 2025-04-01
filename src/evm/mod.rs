use std::{
    borrow::Cow,
    collections::BTreeMap,
    io::Read,
    sync::{Arc, Mutex},
};

use alloy::{
    network::{Ethereum, EthereumWallet, Network, TransactionBuilder},
    primitives::{Address, BlockNumber, Bytes, B256, U128, U256, U64},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        EthCall, EthCallMany, Identity, PendingTransactionBuilder, Provider, ProviderBuilder,
        ProviderCall, RootProvider, RpcWithBlock, SendableTx,
    },
    rpc::{
        client::NoParams,
        types::{
            erc4337::TransactionConditional,
            simulate::{SimulatePayload, SimulatedBlock},
            AccessListResult, Bundle, EthCallResponse, SyncStatus, TransactionRequest,
        },
    },
    signers::{
        k256::{elliptic_curve::SecretKey, Secp256k1},
        local::PrivateKeySigner,
    },
    transports::TransportResult,
};
use alloy_node_bindings::{Anvil, AnvilInstance};
use reqwest::Url;
use serde_json::{json, value::RawValue};

pub struct Evm {
    rpc_url: Url,
    anvil: AnvilInstance,
    // This mutex is intentionally a sync mutex and not a tokio mutex.
    users: std::sync::Mutex<BTreeMap<Address, EvmUser>>,
}

impl Evm {
    pub fn new() -> Self {
        Evm::default()
    }

    pub fn rpc_url(&self) -> Url {
        self.rpc_url.clone()
    }

    pub fn chain_id(&self) -> u64 {
        self.anvil.chain_id()
    }

    pub fn test_user_count(&self) -> usize {
        self.anvil.addresses().len()
    }

    pub fn key(&self, index: usize) -> SecretKey<Secp256k1> {
        self.anvil.keys()[index].clone()
    }

    pub fn test_user(&self, index: usize) -> EvmUser {
        if index >= self.test_user_count() {
            panic!(
                "Reached maximum number of test users: {}",
                self.test_user_count()
            );
        }
        self.user_from(
            self.anvil.addresses()[index],
            self.anvil.keys()[index].clone(),
        )
    }

    pub fn user_from(&self, address: Address, key: SecretKey<Secp256k1>) -> EvmUser {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get(&address) {
            return user.clone();
        }
        let signer: PrivateKeySigner = key.clone().into();
        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(signer))
            .on_http(self.rpc_url.clone());
        let user = EvmUser {
            address,
            key,
            provider: Arc::new(provider),
        };
        users.insert(user.address, user.clone());
        user
    }

    pub fn default_user(&self) -> EvmUser {
        self.test_user(0)
    }

    pub async fn transfer(&self, user: &EvmUser, to: Address, amount: U256) {
        let tx = TransactionRequest::default().with_to(to).with_value(amount);
        user.provider
            .send_transaction(tx)
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
    }

    pub async fn get_balance(&self, addr: Address) -> U256 {
        self.default_user()
            .provider
            .get_balance(addr)
            .await
            .unwrap()
    }

    pub async fn mine_block(&self) {
        let response: serde_json::Value = self
            .default_user()
            .provider
            .client()
            .request("evm_mine", json!({}))
            .await
            .unwrap();
        assert_eq!(response, "0x0");
    }
}

impl Default for Evm {
    fn default() -> Self {
        let mut anvil = Anvil::new().keep_stdout().try_spawn().unwrap();
        let anvil_stdout = anvil.child_mut().stdout.take();

        tokio::spawn(async {
            let mut buf = [0_u8; 4096];
            let mut mv = anvil_stdout;
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                match mv.as_mut().unwrap().read(&mut buf) {
                    Ok(len) => {
                        eprintln!(
                            "{}",
                            String::from_utf8(buf[0..len].to_vec()).unwrap_or_default()
                        );
                    }
                    Err(_) => return,
                }
            }
        });

        let anvil_url: Url = anvil.endpoint().parse().unwrap();
        Self {
            rpc_url: anvil_url,
            anvil,
            users: Mutex::new(BTreeMap::new()),
        }
    }
}

pub type EvmProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
    Ethereum,
>;

#[derive(Clone)]
pub struct EvmUser {
    pub address: Address,
    pub key: SecretKey<Secp256k1>,
    pub provider: Arc<EvmProvider>,
}

#[async_trait::async_trait]
impl Provider<Ethereum> for EvmUser {
    fn root(&self) -> &RootProvider {
        self.provider.root()
    }

    fn get_accounts(&self) -> ProviderCall<NoParams, Vec<Address>> {
        self.provider.get_accounts()
    }

    fn get_blob_base_fee(&self) -> ProviderCall<NoParams, U128, u128> {
        self.provider.get_blob_base_fee()
    }

    fn get_block_number(&self) -> ProviderCall<NoParams, U64, BlockNumber> {
        self.provider.get_block_number()
    }

    fn call<'req>(
        &self,
        tx: <Ethereum as Network>::TransactionRequest,
    ) -> EthCall<Ethereum, Bytes> {
        self.provider.call(tx)
    }

    fn call_many<'req>(
        &self,
        bundles: &'req Vec<Bundle>,
    ) -> EthCallMany<'req, Ethereum, Vec<Vec<EthCallResponse>>> {
        self.provider.call_many(bundles)
    }

    fn simulate<'req>(
        &self,
        payload: &'req SimulatePayload,
    ) -> RpcWithBlock<
        &'req SimulatePayload,
        Vec<SimulatedBlock<<Ethereum as Network>::BlockResponse>>,
    > {
        self.provider.simulate(payload)
    }

    fn get_chain_id(&self) -> ProviderCall<NoParams, U64, u64> {
        self.provider.get_chain_id()
    }

    fn create_access_list<'a>(
        &self,
        request: &'a <Ethereum as Network>::TransactionRequest,
    ) -> RpcWithBlock<&'a <Ethereum as Network>::TransactionRequest, AccessListResult> {
        self.provider.create_access_list(request)
    }

    async fn send_raw_transaction(
        &self,
        encoded_tx: &[u8],
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        self.provider.send_raw_transaction(encoded_tx).await
    }

    async fn send_raw_transaction_conditional(
        &self,
        encoded_tx: &[u8],
        conditional: TransactionConditional,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        self.provider
            .send_raw_transaction_conditional(encoded_tx, conditional)
            .await
    }

    async fn send_transaction_internal(
        &self,
        tx: SendableTx<Ethereum>,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        self.provider.send_transaction_internal(tx).await
    }

    fn syncing(&self) -> ProviderCall<NoParams, SyncStatus> {
        self.provider.syncing()
    }

    fn get_client_version(&self) -> ProviderCall<NoParams, String> {
        self.provider.get_client_version()
    }

    fn get_sha3(&self, data: &[u8]) -> ProviderCall<(String,), B256> {
        self.provider.get_sha3(data)
    }

    fn get_net_version(&self) -> ProviderCall<NoParams, U64, u64> {
        self.provider.get_net_version()
    }

    async fn raw_request_dyn(
        &self,
        method: Cow<'static, str>,
        params: &RawValue,
    ) -> TransportResult<Box<RawValue>> {
        self.provider.raw_request_dyn(method, params).await
    }

    fn transaction_request(&self) -> <Ethereum as Network>::TransactionRequest {
        self.provider.transaction_request()
    }
}
