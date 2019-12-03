#[macro_use]
extern crate assert_json_diff;

extern crate hyper;
extern crate serde;
use hyper::{Client};
use futures::stream::TryStreamExt;
use std::str;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
enum NodeType {
    Tezedge,
    Ocaml,
}

#[tokio::main]
async fn main() -> Result<()>{
    let rust_head = get_head(NodeType::Tezedge).await?;
    let ocaml_head = get_head(NodeType::Ocaml).await?;

    // TODO: make it more obvious in the output
    //              actual      expected
    assert_json_eq!(rust_head, ocaml_head);

    Ok(())
}

async fn get_head(node_type: NodeType) -> Result<serde_json::value::Value> {
    let client = Client::new();

    let url = match node_type {
        NodeType::Ocaml => "http://46.101.160.245:3000/chains/main/blocks/head",       // reference Ocaml node
        NodeType::Tezedge => "http://localhost:18732/chains/main/blocks/head",         // locally built Tezedge node
    };

    // let url = format!("http://http://46.101.160.245:{}/chains/main/blocks/head", port);

    let mut res = client.get(url.parse().unwrap()).await?;
    let body = res.body_mut().try_concat().await?.to_vec();

    let json_string = match str::from_utf8(&body) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    Ok(serde_json::from_str(json_string)?)
}