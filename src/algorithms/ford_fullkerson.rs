use std::{
    cmp,
    collections::{HashMap, HashSet},
};
const INF: usize = std::usize::MAX / 3;

#[derive(Debug, Clone)]
pub struct Graph {
    pub size: usize,
    pub edges: Vec<HashMap<(usize, usize), usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            size: n,
            edges: vec![HashMap::new(); n],
        }
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, cap: usize) {
        self.edges[src].insert((src, dst), cap);
        self.edges[dst].insert((dst, src), 0);
    }

    pub fn dfs(
        &mut self,
        start: usize,
        goal: usize,
        f: usize,
        visited: &mut HashSet<usize>,
    ) -> usize {
        if start == goal {
            return f;
        }
        visited.insert(start);
        for ((src, dst), w) in self.edges[start].clone() {
            if !visited.contains(&dst) && w > 0 {
                let d = self.dfs(dst, goal, cmp::min(f, w), visited);
                if d > 0 {
                    *self.edges[src].get_mut(&(src, dst)).unwrap() -= d;
                    *self.edges[dst].get_mut(&(dst, src)).unwrap() += d;
                    return d;
                }
            }
        }

        return 0;
    }

    pub fn max_frow(&mut self, start: usize, goal: usize) -> usize {
        let mut flow = 0;
        loop {
            let f = self.dfs(start, goal, INF, &mut HashSet::new());
            if f == 0 {
                break;
            } else {
                flow += f;
            }
        }
        flow
    }
}