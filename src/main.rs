use alloy_primitives::{address, Address};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // let url = "wss://api.aori.io";
    let url = "wss://dev.api.aori.io";
    let weth = address!("82aF49447D8a07e3bd95BD0d56f35241523fBab1");
    let usdc = address!("af88d065e77c8cC2239327C5EDb3A432268e5831");

    let snapshot_request_bid = get_orderbook_side("SELL", 42161, weth, usdc);
    let (mut ws, _) = tokio_tungstenite::connect_async(url).await.unwrap();

    let payload = serde_json::to_string_pretty(&snapshot_request_bid).unwrap();

    println!("Sending payload: {}", payload);

    ws.send(tungstenite::Message::Text(payload)).await.unwrap();

    loop {
        match ws.next().await {
            Some(Ok(tungstenite::Message::Text(value))) => {
                let value = serde_json::from_str::<Value>(&value).unwrap();
                println!("bid: {}", value);
                break;
            }
            _ => (),
        }
    }
}

pub fn get_orderbook_side(
    side: &str,
    chain_id: u64,
    base_address: Address,
    quote_address: Address,
) -> Value {
    json!({
        "id": 1,
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
