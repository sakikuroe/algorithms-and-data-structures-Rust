use algorithms_and_data_structures_rust::graph::Graph;

fn main() {
    let mut g = Graph::new(7);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(1, 4);
    g.add_edge(2, 5);
    g.add_edge(2, 6);
    println!("{}", g);
}