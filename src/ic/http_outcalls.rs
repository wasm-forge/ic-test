use pocket_ic::{
    common::rest::{
        CanisterHttpHeader, CanisterHttpMethod, CanisterHttpReply, CanisterHttpRequest,
        CanisterHttpResponse, MockCanisterHttpResponse,
    },
    nonblocking::PocketIc,
};
use std::{sync::Weak, time::Duration};
use tokio::{sync::Mutex, time::sleep};

/// This function run a loop that fetches pending HTTP outcalls from pocket-ic
/// and handles them either by forward them to anvil or by replaying responses
/// that were recorded by `record_http_responses` below.
pub async fn handle_http_outcalls(
    pocket_ic: Weak<Mutex<PocketIc>>,
    anvil: reqwest::Url,
    rpc_nodes: Vec<String>,
) {
    while let Some(pic) = pocket_ic.upgrade() {
        let requests = {
            let pic = pic.lock().await;
            pic.get_canister_http().await
        };

        sleep(Duration::from_millis(50)).await;

        for request in requests {
            let url = request.url.clone();
            if rpc_nodes.contains(&url) {
                let response = forward_http(request, anvil.to_string()).await;
                let pic = pic.lock().await;
                pic.mock_canister_http_response(response).await;
            } else {
                println!("MISSING {},", request.url);
            }
        }
    }
}

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

fn strings_to_headers(hs: Vec<(String, String)>) -> Vec<CanisterHttpHeader> {
    hs.into_iter()
        .map(|(name, value)| CanisterHttpHeader { name, value })
        .collect()
}
