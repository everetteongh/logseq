use logseq::{
    block::{Block, view::TaskStatus},
    graph::Graph,
};
use std::fs;

#[test]
fn task_handling() {
    let graph = Graph::builder()
        .dir("assets/example_graph".into())
        .build()
        .unwrap();

    let mut today = graph.today().unwrap();

    let block = Block {
        markdown: "TODO This is a task.".to_string(),
        ..Default::default()
    };

    today.document.blocks.insert(block.properties.id, block);

    today.save().unwrap();

    let mut today_content = fs::read_to_string(&today.path).unwrap();
    assert!(today_content.contains("TODO"));

    for ref mut block in today.document.blocks_mut() {
        if let Some(ref mut task) = block.task_mut() {
            task.status(&TaskStatus::Done);
        }
    }

    today.save().unwrap();

    today_content = fs::read_to_string(&today.path).unwrap();
    assert_eq!(today_content.contains("TODO"), false);
}
