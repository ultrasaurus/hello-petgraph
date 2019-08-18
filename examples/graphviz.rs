use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;

fn main() {
    println!("hello graph!");
    let mut graph = Graph::<_, i32>::new();
    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");
    graph.extend_with_edges(&[
        (0, 1), (0, 2), (0, 3),
        (1, 2), (1, 3),
        (2, 3),
    ]);

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut f = File::create("example.dot").unwrap();
    let output = format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    f.write_all(&output.as_bytes()).expect("could not write file");
}
