use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    hash::Hash,
};

#[derive(PartialEq, Eq)]
pub struct Node<T> {
    pub vertex: usize,
    pub priority: T,
}

impl<T> Ord for Node<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Edge<T> {
    pub src: usize,
    pub dst: usize,
    pub weight: T,
}

impl<T> Ord for Edge<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T> PartialOrd for Edge<T>
where
    T: Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Edge<T> {
    pub fn new(src: usize, dst: usize, weight: T) -> Edge<T> {
        Edge { src, dst, weight }
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T> {
    size: usize,
    pub edges: Vec<HashMap<usize, Edge<T>>>,
}

impl<T> Graph<T>
where
    T: Clone + Copy + Eq + Hash,
{
    pub fn new(size: usize) -> Self {
        Graph {
            size,
            edges: vec![HashMap::new(); size],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, weight: T) {
        self.edges[src].insert(dst, Edge::new(src, dst, weight));
    }

    pub fn contains_edge(&self, src: usize, dst: usize) -> bool {
        self.edges[src].contains_key(&dst)
    }

    pub fn connected(&self, src: usize, dst: usize) -> bool {
        self.edges[src].contains_key(&dst) || self.edges[dst].contains_key(&src)
    }

    pub fn gen_complement(&self) -> Graph<usize> {
        let mut g = Graph::new(self.size());
        for src in 0..g.size() {
            for dst in src + 1..g.size() {
                if !self.contains_edge(src, dst) && !self.contains_edge(dst, src) {
                    g.add_edge(src, dst, 1);
                }
            }
        }
        g
    }

    pub fn edges(&self, src: usize) -> HashSet<Edge<T>> {
        let mut res = HashSet::new();
        for &edge in self.edges[src].values() {
            res.insert(edge);
        }
        res
    }

    pub fn add_undirected_edge(&mut self, src: usize, dst: usize, weight: T) {
        self.add_edge(src, dst, weight);
        self.add_edge(dst, src, weight);
    }
}

impl<T> fmt::Display for Graph<T>
where
    T: Clone + Ord + std::fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let g = (*self).clone();
        writeln!(f, "Size : {:?}", g.size)?;
        writeln!(f, "Edges: {{",)?;
        for edges in g.edges {
            let mut edges = edges.values().map(|x| *x).collect::<Vec<_>>();
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
    fn case1() {
        let mut g = Graph::new(7);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(1, 3, 1);
        g.add_edge(1, 4, 1);
        g.add_edge(2, 5, 1);
        g.add_edge(2, 6, 1);
    }

    #[test]
    fn case2() {
        let mut g = Graph::new(7);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(3, 4, 1);
        g.add_edge(4, 5, 1);
        g.add_edge(5, 6, 1);
        g.add_edge(3, 6, 1);
    }
}
