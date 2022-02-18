use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    hash::Hash,
};

#[derive(Eq, PartialEq)]
pub struct Node<T> {
    pub vertex: usize,
    pub priory: T,
}

impl<T> Ord for Node<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.priory.cmp(&other.priory).reverse()
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
        Edge {
            src,
            dst,
            weight,
        }
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

    pub fn get_all_edges(&self) -> HashSet<Edge<T>> {
        let mut res = HashSet::new();
        for e in &self.edges {
            res = res
                .union(&e.values().into_iter().map(|x| *x).collect::<HashSet<_>>())
                .into_iter()
                .map(|x| *x)
                .collect::<HashSet<Edge<T>>>();
        }
        res
    }

    pub fn gen_undirected_graph(&self) -> Graph<T> {
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
        let mut visited = {
            let mut res = vec![false; self.size()];
            res[0] = true;
            res
        };
        let mut que = {
            let mut res = VecDeque::new();
            res.push_back((0, None));
            res
        };

        while let Some((node, prev)) = que.pop_front() {
            for e in self.edges(node) {
                if !visited[e.dst] {
                    visited[e.dst] = true;
                    que.push_back((e.dst, Some(e.src)));
                } else {
                    match prev {
                        Some(node) => {
                            if e.dst != node {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
            }
        }

        return visited.into_iter().fold(true, |x, y| x && y);
    }

    pub fn is_partially_tree(&self, i: usize) -> bool {
        let mut visited = {
            let mut res = vec![false; self.size()];
            res[i] = true;
            res
        };
        let mut que = {
            let mut res = VecDeque::new();
            res.push_back((i, None));
            res
        };

        while let Some((node, prev)) = que.pop_front() {
            for e in self.edges(node) {
                if !visited[e.dst] {
                    visited[e.dst] = true;
                    que.push_back((e.dst, Some(e.src)));
                } else {
                    match prev {
                        Some(node) => {
                            if e.dst != node {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
            }
        }

        return true;
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
        assert_eq!(g.gen_undirected_graph().is_tree(), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(0), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(1), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(2), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(3), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(4), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(5), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(6), true);
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
        assert_eq!(g.gen_undirected_graph().is_tree(), false);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(0), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(1), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(2), true);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(3), false);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(4), false);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(5), false);
        assert_eq!(g.gen_undirected_graph().is_partially_tree(6), false);
    }
}
