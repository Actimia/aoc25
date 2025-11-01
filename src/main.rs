mod util;

use crate::util::graph::Graph;

fn main() {
    println!("Hello, world!");

    let mut g: Graph<(), ()> = Graph::new();
    let n1 = g.add_node(());
    let n2 = g.add_node(());
    g.add_edge(n1, n2, ());
}
