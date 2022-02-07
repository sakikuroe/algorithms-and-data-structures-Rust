use std::{
    cmp,
    collections::HashSet,
};
use crate::data_structures::graph;
const INF: usize = std::usize::MAX / 3;

impl graph::Graph {
    pub fn max_frow_dfs(
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
        for &graph::Edge{src, dst, weight} in self.clone().edges[start].values() {
            if !visited.contains(&dst) && weight > 0 {
                let d = self.max_frow_dfs(dst, goal, cmp::min(f, weight), visited);
                if d > 0 {
                    self.edges[src].get_mut(&dst).unwrap().weight -= d;
                    self.edges[dst].get_mut(&src).unwrap().weight += d;
                    return d;
                }
            }
        }

        return 0;
    }

    pub fn max_frow(&mut self, start: usize, goal: usize) -> usize {
        let mut flow = 0;
        loop {
            let f = self.max_frow_dfs(start, goal, INF, &mut HashSet::new());
            if f == 0 {
                break;
            } else {
                flow += f;
            }
        }
        flow
    }
}
