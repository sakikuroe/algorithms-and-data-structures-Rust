use std::{cmp::Ordering, collections::HashSet, fmt, hash::Hash};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
    T: Eq + PartialEq + Ord,
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

#[derive(Clone, Debug)]
pub struct Graph<T> {
    size: usize,
    pub edges: Vec<Vec<Edge<T>>>,
}

impl<T> Graph<T>
where
    T: Clone + Copy + Eq,
{
    pub fn new(size: usize) -> Self {
        Graph {
            size,
            edges: vec![vec![]; size],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, weight: T) {
        self.edges[src].push(Edge::new(src, dst, weight));
    }

    pub fn add_undirected_edge(&mut self, src: usize, dst: usize, weight: T) {
        self.add_edge(src, dst, weight);
        self.add_edge(dst, src, weight);
    }

    pub fn get_all_edges(&self) -> HashSet<Edge<T>>
    where
        T: Hash,
    {
        let mut res = HashSet::new();
        for edges in &self.edges {
            for e in edges {
                res.insert(e.clone());
            }
        }
        res
    }

    pub fn gen_complement_graph(&self) -> Graph<T>
    where
        T: Hash + From<u8>,
    {
        let all_edges = self.get_all_edges();
        let mut g = Graph::new(self.size());
        for src in 0..g.size() {
            for dst in 0..g.size() {
                if !all_edges.contains(&Edge::new(src, dst, 1.into()))
                    && !all_edges.contains(&Edge::new(dst, src, 1.into()))
                {
                    g.add_edge(src, dst, 1.into());
                }
            }
        }
        g
    }
}

impl<T> fmt::Display for Graph<T>
where
    T: Clone + Ord + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let g = (*self).clone();
        writeln!(f, "Size : {:?}", g.size)?;
        writeln!(f, "Edges: {{",)?;
        for mut edges in g.edges {
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
