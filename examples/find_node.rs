use petgraph::Graph;

fn main() {
    println!("hello graph!");
    let mut g = Graph::<&str, i32>::new();
    g.add_node("apple");
    g.add_node("book");
    g.add_node("carrot");
    let index = g.node_indices().find(|i| g.node_weight(*i) == Some(&"book")).unwrap();
    println!("book index: {:?}", index);
}
