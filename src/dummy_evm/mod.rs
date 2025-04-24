pub struct Evm {}
impl Evm {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn rpc_url(&self) -> reqwest::Url {
        // dummy Url
        reqwest::Url::parse("http://localhost").expect("Error creating dummy url!")
    }

    pub(crate) async fn mine_block(&self) {
        // do nothing, dummy implementation
    }
}

#[derive(Clone)]
pub struct EvmUser {}
