use std::collections::BinaryHeap;
#[derive(Clone, Copy, Debug)]
struct Edge {
    src: usize,
    dst: usize,
    weight: usize,
}
#[derive(Debug)]
struct Graph {
    edges: Vec<Vec<Edge>>,
    size: usize,
}
impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            edges: vec![vec![]; size],
            size,
        }
    }
    fn add_edge(&mut self, e: Edge) {
        self.edges[e.src].push(Edge {
            src: e.src,
            dst: e.dst,
            weight: e.weight,
        });
    }
}
