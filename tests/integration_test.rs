#[macro_use]
extern crate assert_json_diff;

extern crate compare-head;

#[test]
fn test_heads() -> Result<()>{
    let rust_head = get_head(compare-head::NodeType::Tezedge).await?;
    let ocaml_head = get_head(compare-head::NodeType::Ocaml).await?;

    // TODO: make it more obvious in the output
    //              actual      expected
    assert_json_eq!(rust_head, ocaml_head);

    Ok(())
}
