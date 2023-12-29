use alloy_primitives::{address, Address};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[tokio::main]
async fn main() {
    // let url = "wss://api.aori.io";
    let url = "wss://dev.api.aori.io";
    let (mut ws, _) = tokio_tungstenite::connect_async(url).await.unwrap();

    let weth = address!("82aF49447D8a07e3bd95BD0d56f35241523fBab1");
    let usdc = address!("af88d065e77c8cC2239327C5EDb3A432268e5831");

    let snapshot_request_ask = make_request("SELL", 42161, weth, usdc);
    let snapshot_request_bid = make_request("BUY", 42161, weth, usdc);

    let response_bid = send_and_receive(&snapshot_request_bid, &mut ws).await;
    let response_ask = send_and_receive(&snapshot_request_ask, &mut ws).await;

    assert_ne!(response_bid["result"], response_ask["result"]);
}

pub async fn send_and_receive(
    payload: &Value,
    ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Value {
    let payload = serde_json::to_string_pretty(&payload).unwrap();

    println!("Sending payload: {}", payload);

    ws.send(tungstenite::Message::Text(payload)).await.unwrap();

    match ws.next().await.unwrap() {
        Ok(tungstenite::Message::Text(value)) => {
            let value = serde_json::from_str::<Value>(&value).unwrap();
            println!(
                "Received response: {}",
                serde_json::to_string_pretty(&value).unwrap()
            );
            value
        }
        response => panic!("Unexpected response: {:?}", response),
    }
}

pub fn make_request(
    side: &str,
    chain_id: u64,
    base_address: Address,
    quote_address: Address,
) -> Value {
    static ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    json!({
        "id": ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        "jsonrpc": "2.0",
        "method": "aori_viewOrderbook",
        "params": [
            {
                "chainId": chain_id,
                "query": {
                    "base": base_address.to_string(),
                    "quote": quote_address.to_string(),
                },
                "side": side,
                "limit": 100,
            }
        ]
    })
}
