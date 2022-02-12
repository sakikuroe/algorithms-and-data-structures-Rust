use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};

#[derive(Eq, PartialEq)]
pub struct Node {
    pub vertex: usize,
    pub priory: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priory.cmp(&other.priory).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
    pub edges: Vec<HashMap<usize, Edge>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            size,
            edges: vec![HashMap::new(); size],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, weight: usize) {
        self.edges[src].insert(dst, Edge::new(src, dst, weight));
    }

    pub fn contains_edge(&self, src: usize, dst: usize) -> bool {
        self.edges[src].contains_key(&dst)
    }

    pub fn connected(&self, src: usize, dst: usize) -> bool {
        self.edges[src].contains_key(&dst) || self.edges[dst].contains_key(&src)
    }

    pub fn gen_complement(&self) -> Self {
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

    pub fn edges(&self, src: usize) -> HashSet<Edge> {
        let mut res = HashSet::new();
        for &edge in self.edges[src].values() {
            res.insert(edge);
        }
        res
    }

    pub fn add_undirected_edge(&mut self, src: usize, dst: usize, weight: usize) {
        self.add_edge(src, dst, weight);
        self.add_edge(dst, src, weight);
    }

    pub fn get_all_edges(&self) -> HashSet<Edge> {
        let mut res = HashSet::new();
        for e in &self.edges {
            res = res
                .union(&e.values().into_iter().map(|x| *x).collect::<HashSet<_>>())
                .into_iter()
                .map(|x| *x)
                .collect::<HashSet<Edge>>();
        }
        res
    }

    pub fn gen_undirected_graph(&self) -> Graph {
        let mut g = Graph::new(self.size);
        for edges in &self.edges {
            for (_, &e) in edges {
                g.add_undirected_edge(e.src, e.dst, e.weight);
            }
        }
        g
    }

    pub fn get_connected_components(&self) -> usize {
        let mut visited = vec![false; self.size];
        let g = self.gen_undirected_graph();
        let mut res = 0;
        for i in 0..g.size {
            if !visited[i] {
                let mut que = VecDeque::new();
                que.push_back(i);

                while let Some(node) = que.pop_front() {
                    visited[node] = true;
                    for &e in g.edges[node].values() {
                        if !visited[e.dst] {
                            que.push_back(e.dst);
                        }
                    }
                }
                res += 1;
            }
        }
        res
    }

    pub fn is_tree(&self) -> bool {
        if self.get_connected_components() != 1 {
            return false;
        }
        let g = self.gen_undirected_graph();
        if g.get_all_edges().len() == 2 * (g.size - 1) {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_bipartite(&self) -> Option<(usize, usize)> {
        let mut que = VecDeque::new();
        if self.size == 0 {
            return Some((0, 0));
        }

        let mut color = vec![None; self.size];
        que.push_back((0, 0));

        let (mut b, mut w) = (0, 0);
        while let Some((v, p)) = que.pop_front() {
            if p == 0 {
                w += 1;
            } else {
                b += 1;
            }
            color[v] = Some(p ^ 1);
            for (&dst, _e) in &self.edges[v] {
                if color[dst] == Some(p ^ 1) {
                    return None;
                }
                if color[dst].is_none() {
                    que.push_back((dst, p ^ 1));
                }
            }
        }

        if b + w == self.size {
            return Some((b, w));
        } else {
            return None;
        }
    }
}

impl fmt::Display for Graph {
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
    fn value() {
        let mut g = Graph::new(7);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(1, 3, 1);
        g.add_edge(1, 4, 1);
        g.add_edge(2, 5, 1);
        g.add_edge(2, 6, 1);
        assert_eq!(g.is_tree(), true);
        assert_eq!(g.get_diameter(), Some(4));
    }
}
