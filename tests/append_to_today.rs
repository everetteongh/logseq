use logseq::{block::Block, graph::Graph};
use std::fs;

#[test]
fn append_to_today() {
    let graph = Graph::builder()
        .dir("assets/example_graph".into())
        .build()
        .unwrap();

    let mut today = graph.today().unwrap();

    let block = Block {
        markdown: "Hello from Rust code!".to_string(),
        ..Default::default()
    };

    today.document.blocks.insert(block.properties.id, block);

    today.save().unwrap();

    let today_content = fs::read_to_string(today.path).unwrap();
    assert!(today_content.contains("Hello from Rust code!"));
}
