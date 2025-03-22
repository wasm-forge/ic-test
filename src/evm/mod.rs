use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use alloy::{
    network::{Ethereum, EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::TransactionRequest,
    signers::{
        k256::{elliptic_curve::SecretKey, Secp256k1},
        local::PrivateKeySigner,
    },
};
use alloy_node_bindings::{Anvil, AnvilInstance};
use reqwest::Url;
use serde_json::json;

pub struct Evm {
    pub rpc_url: Url,
    anvil: AnvilInstance,
    // This mutex is intentionally a sync mutex and not a tokio mutex.
    users: std::sync::Mutex<BTreeMap<Address, EvmUser>>,
}

impl Evm {
    pub fn new() -> Self {
        let anvil = Anvil::new().try_spawn().unwrap();
        let anvil_url: Url = anvil.endpoint().parse().unwrap();
        Self {
            rpc_url: anvil_url,
            anvil,
            users: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn test_user_count(&self) -> usize {
        self.anvil.addresses().len()
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

#[derive(Clone)]
pub struct EvmUser {
    pub address: Address,
    pub key: SecretKey<Secp256k1>,
    provider: Arc<dyn Provider>,
}

impl Provider for EvmUser {
    fn root(&self) -> &RootProvider<Ethereum> {
        self.provider.root()
    }
}
