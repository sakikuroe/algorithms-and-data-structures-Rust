use std::collections::HashSet;
use crate::data_structures::graph::{Graph, Edge};

#[derive(Clone, Debug)]
pub struct SccGraph<T> {
    pub values: Vec<Vec<usize>>,
    pub edges: Vec<Vec<Edge<T>>>,
}
 
impl<T> SccGraph<T>
where
    T: Clone + Copy + Eq,
{
    pub fn size(&self) -> usize {
        self.edges.len()
    }
 
    pub fn add_edge(&mut self, src: usize, dst: usize, weight: T) {
        self.edges[src].push(Edge::new(src, dst, weight));
    }
}

impl<T> Graph<T>
where
    T: Clone + Copy + Eq,
{
    pub fn scc(&self) -> SccGraph<T> {
        fn dfs<T>(v: usize, used: &mut Vec<bool>, vs: &mut Vec<usize>, g: &Graph<T>) {
            used[v] = true;
            for e in &g.edges[v] {
                if !used[e.dst] {
                    dfs(e.dst, used, vs, g);
                }
            }
            vs.push(v);
        }
 
        let mut used = vec![false; self.size()];
        let mut vs: Vec<usize> = vec![];
        for v in 0..self.size() {
            if !used[v] {
                dfs(v, &mut used, &mut vs, &self);
            }
        }
 
        fn rev_dfs<T>(
            v: usize,
            id: usize,
            used: &mut Vec<bool>,
            cmp: &mut Vec<usize>,
            g: &Graph<T>,
        ) {
            used[v] = true;
            cmp[v] = id;
            for e in &g.edges[v] {
                if !used[e.dst] {
                    rev_dfs(e.dst, id, used, cmp, g);
                }
            }
        }
 
        let rev_g = {
            let mut res = Graph::new(self.size());
            for edges in &self.edges {
                for e in edges {
                    res.add_edge(e.dst, e.src, e.weight);
                }
            }
            res
        };
 
        let mut used = vec![false; self.size()];
        let mut cmp = vec![0; self.size()];
        let mut id = 0;
        for v in vs.iter().rev() {
            if !used[*v] {
                rev_dfs(*v, id, &mut used, &mut cmp, &rev_g);
                id += 1;
            }
        }
 
        let mut res = SccGraph {
            values: vec![vec![]; id],
            edges: vec![vec![]; id],
        };
 
        let mut used = HashSet::new();
        for i in 0..self.size() {
            res.values[cmp[i]].push(i);
            for e in &self.edges[i] {
                if cmp[e.src] != cmp[e.dst] && !used.contains(&(cmp[e.src], cmp[e.dst])) {
                    res.add_edge(cmp[e.src], cmp[e.dst], e.weight);
                    used.insert((cmp[e.src], cmp[e.dst]));
                }
            }
        }
        res
    }
}