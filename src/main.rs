use alloy_primitives::{address, Address};
use serde_json::{json, Value};

fn main() {
    let url = "wss://dev.api.aori.io/";
    let weth = address!("82aF49447D8a07e3bd95BD0d56f35241523fBab1");
    let usdc = address!("af88d065e77c8cC2239327C5EDb3A432268e5831");

    let snapshot_request_bid = snapshot_request_body(1, weth, usdc, 1, "BUY");
    let snapshot_request_ask = snapshot_request_body(2, weth, usdc, 1, "SELL");
    let (mut ws, _) = tungstenite::connect(url).unwrap();

    ws.send(tungstenite::Message::Text(snapshot_request_bid.to_string()))
        .unwrap();

    loop {
        match ws.read() {
            Ok(tungstenite::Message::Text(value)) => {
                let value = serde_json::from_str::<Value>(&value).unwrap();
                println!("bid: {}", value);
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => (),
        }
    }
}

pub fn snapshot_request_body(
    id: u64,
    base_address: Address,
    quote_address: Address,
    chain_id: u64,
    side: &str,
) -> Value {
    json!({
       "id": id,
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
                    "base": format!("{}", base_address),
                    "quote": format!("{}", quote_address),
                },
                "side": side,
                "limit": 100,
            }
        ]
    })
}
