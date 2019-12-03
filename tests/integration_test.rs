#[macro_use]
extern crate assert_json_diff;

extern crate compare_head;

#[test]
fn test_heads() {
    let rust_head = match compare_head::get_head(compare_head::NodeType::Tezedge) {
        Ok(v) => v,
        Err(e) => panic!("Invalid json: {}", e),
    };
    let ocaml_head = match compare_head::get_head(compare_head::NodeType::Ocaml) {
        Ok(v) => v,
        Err(e) => panic!("Invalid json: {}", e),
    };

    // TODO: make it more obvious in the output
    //              actual      expected
    assert_json_eq!(rust_head, ocaml_head);
}
