
extern crate hyper;
extern crate serde;
use hyper::{Client, Uri};

#[cfg(test)]
mod tests {
    #[test]
    fn compare_json() {
        let client = Client::new();
        let ocaml_url = "http://localhost:3000/chains/main/blocks/head".parse().unwrap();
        let rust_url = "http://localhost:18732/chains/main/blocks/head".parse().unwrap();
    
        let mut res_ocaml = client.get(ocaml_url).await?;
        let mut res_rust = client.get(rust_url).await?;
    
        let body_ocaml = res_ocaml.body_mut().try_concat().await?.to_vec();
        let body_rust = res_rust.body_mut().try_concat().await?.to_vec();
    
        let string_ocaml = match str::from_utf8(&body_ocaml) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    
        let string_rust = match str::from_utf8(&body_rust) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    
        //let string_ocaml = String::from_vec(body_ocaml);
    
        let ocaml_head = serde_json::from_str(string_ocaml)?;
        let rust_head = serde_json::from_str(string_rust)?;
    
        assert_json_eq!(rust_head, ocaml_head);
    }
}