use std::{cmp::Ordering, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
    pub weight: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Edge {
    pub fn new(src: usize, dst: usize, weight: usize) -> Edge {
        Edge {
            src,
            dst,
            weight,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    size: usize,
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            size,
            edges: vec![vec![]; size],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, weight: usize) {
        self.edges[src].push(Edge::new(src, dst, weight));
    }

    pub fn add_undirected_edge(&mut self, src: usize, dst: usize, weight: usize) {
        self.add_edge(src, dst, weight);
        self.add_edge(dst, src, weight);
    }

    pub fn edges(&self, src: usize) -> Vec<Edge> {
        let mut res = vec![];
        for &edge in &self.edges[src] {
            res.push(edge);
        }
        res
    }

    pub fn get_all_edges(&self) -> Vec<Edge> {
        let mut res = vec![];
        for edge_i in &self.edges {
            for &e in edge_i {
                res.push(e);
            }
        }
        res
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let g = (*self).clone();
        writeln!(f, "Size : {:?}", g.size)?;
        writeln!(f, "Edges: {{",)?;
        for mut edges in g.edges.clone() {
            edges.sort();
            for e in edges {
                writeln!(
                    f,
                    "Edge: {{{:?} -> {:?}, weight: {:?}}},",
                    e.src, e.dst, e.weight
                )?;
            }
        }
        writeln!(f, "}}",)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut g = Graph::new(7);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(1, 3, 1);
        g.add_edge(1, 4, 1);
        g.add_edge(2, 5, 1);
        g.add_edge(2, 5, 1);
        g.add_edge(2, 6, 1);
        assert_eq!(g.get_all_edges().len(), 7);
    }
}
