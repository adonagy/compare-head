#[macro_use]
extern crate assert_json_diff;

extern crate reqwest;
extern crate serde;
use std::error::Error;
use std::io::Read;
use std::str;

#[derive(Debug)]
pub enum NodeType {
    Tezedge,
    Ocaml,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {}

pub fn get_head(node_type: NodeType) -> Result<serde_json::value::Value> {
    let url = match node_type {
        NodeType::Ocaml => "http://46.101.160.245:3000/chains/main/blocks/head", // reference Ocaml node
        NodeType::Tezedge => "http://localhost:18732/chains/main/blocks/head", // locally built Tezedge node
    };

    // let url = format!("http://http://46.101.160.245:{}/chains/main/blocks/head", port);

    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    // let json_string = match str::from_utf8() {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };

    Ok(serde_json::from_str(&body)?)
}
