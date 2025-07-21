//! ## HTTP Outcall Handler for PocketIC
//!
//! This module supports forwarding HTTP outcalls made by canisters during tests
//! in [`pocket-ic`] environments. It polls the in-memory HTTP queue of pending
//! canister HTTP requests and responds to them either by:
//!
//! - Forwarding them to a real HTTP endpoint (e.g. an Ethereum JSON-RPC node via Anvil),
//! - Or ignoring/unmocking them if not explicitly allowed.
//!
//! ### Usage
//! This is intended to run in the background of your test setup. It ensures canisters
//! under test can successfully resolve HTTP outcalls when using `canister_http`.

use log::error;
use pocket_ic::{
    common::rest::{
        CanisterHttpHeader, CanisterHttpMethod, CanisterHttpReply, CanisterHttpRequest,
        CanisterHttpResponse, MockCanisterHttpResponse,
    },
    nonblocking::PocketIc,
};
use std::{sync::Weak, time::Duration};
use tokio::time::sleep;

/// Spawn a polling loop to handle HTTP outcalls in [`PocketIc`] by forwarding
/// matching requests to the given Anvil node, and mocking their response.
///
/// # Parameters
///
/// - `pocket_ic`: A weak reference to a running PocketIC instance.
/// - `anvil`: The URL to which HTTP requests should be forwarded.
/// - `rpc_nodes`: A list of allowed URLs that the handler is authorized to forward.
///
/// # Behavior
///
/// - Every 50ms, checks for pending HTTP requests from the canister runtime.
/// - Forwards matching requests to the Anvil URL and mocks the response.
/// - Logs missing/mismatched URLs to stderr.
///
/// # Notes
///
/// - Only requests with exact URL matches in `rpc_nodes` are forwarded.
/// - Unsupported URLs are logged and not answered (which may cause the test to block).
pub async fn handle_http_outcalls(
    pocket_ic: Weak<PocketIc>,
    anvil: reqwest::Url,
    rpc_nodes: Vec<String>,
) {
    while let Some(pic) = pocket_ic.upgrade() {
        let requests = { pic.get_canister_http().await };

        sleep(Duration::from_millis(50)).await;

        for request in requests {
            let url = request.url.clone();

            if rpc_nodes.contains(&url) {
                let response = forward_http(request, anvil.to_string()).await;
                pic.mock_canister_http_response(response).await;
            } else {
                error!("MISSING {},", request.url);
            }
        }
    }
}

/// Forward an HTTP request to the configured Anvil endpoint and wrap the result
/// in a mock HTTP response that PocketIC can return to the canister.
async fn forward_http(request: CanisterHttpRequest, url: String) -> MockCanisterHttpResponse {
    let client = reqwest::Client::new();

    let method = match request.http_method {
        CanisterHttpMethod::GET => reqwest::Method::GET,
        CanisterHttpMethod::POST => reqwest::Method::POST,
        CanisterHttpMethod::HEAD => reqwest::Method::HEAD,
    };

    let mut forward = client.request(method, url);
    for header in &request.headers {
        forward = forward.header(&header.name, &header.value);
    }
    forward = forward.body(request.body.clone());

    let outcome = forward.send().await;
    let Ok(response) = outcome else {
        return MockCanisterHttpResponse {
            subnet_id: request.subnet_id,
            request_id: request.request_id,
            response: CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
                status: 101,
                headers: vec![],
                body: vec![],
            }),
            additional_responses: vec![],
        };
    };

    let headers = strings_to_headers(
        response
            .headers()
            .iter()
            .map(|(n, v)| (n.to_string(), v.to_str().unwrap().to_string()))
            .collect(),
    );

    let status = response.status().as_u16();
    let bytes = response.bytes().await.unwrap();

    MockCanisterHttpResponse {
        subnet_id: request.subnet_id,
        request_id: request.request_id,
        response: CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
            status,
            headers,
            body: bytes.to_vec(),
        }),
        additional_responses: vec![],
    }
}

/// Convert a simple `(name, value)` tuple list into PocketIC's HTTP header format.
fn strings_to_headers(hs: Vec<(String, String)>) -> Vec<CanisterHttpHeader> {
    hs.into_iter()
        .map(|(name, value)| CanisterHttpHeader { name, value })
        .collect()
}
