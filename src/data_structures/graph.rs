use std::{collections::HashSet, fmt};

pub trait IntoEdge {
    fn into_edge(self) -> Edge;
}

impl IntoEdge for (usize, usize) {
    fn into_edge(self) -> Edge {
        Edge {
            src: self.0,
            dst: self.1,
            weight: 1,
        }
    }
}
impl IntoEdge for (usize, usize, usize) {
    fn into_edge(self) -> Edge {
        Edge {
            src: self.0,
            dst: self.1,
            weight: self.2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
    pub weight: usize,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub size: usize,
    pub edges: Vec<HashSet<Edge>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            size: n,
            edges: vec![HashSet::new(); n],
        }
    }
    pub fn add_edge(&mut self, src: usize, dst: usize) {
        self.edges[src].insert((src, dst).into_edge());
    }
    pub fn add_weighted_edge(&mut self, src: usize, dst: usize, weight: usize) {
        self.edges[src].insert((src, dst, weight).into_edge());
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let g = (*self).clone();
        writeln!(f, "Size : {:?}", g.size)?;
        writeln!(f, "Edges: {{",)?;
        for i in 0..g.size {
            let mut edges = g.edges[i].clone().into_iter().collect::<Vec<Edge>>();
            edges.sort();
            for &e in &edges {
                writeln!(f, "({:?} -> {:?}, weight: {:?})", e.src, e.dst, e.weight)?;
            }
        }
        write!(f, "}}",)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value() {
        let mut g = Graph::new(7);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(1, 4);
        g.add_edge(2, 5);
        g.add_edge(2, 6);
    }
}
