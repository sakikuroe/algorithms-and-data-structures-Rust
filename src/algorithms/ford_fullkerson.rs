use std::{
    cmp,
    collections::HashSet,
};
use crate::data_structures::graph;
const INF: usize = std::usize::MAX / 3;

impl graph::Graph {
    fn max_frow_dfs(
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
        for &graph::Edge {
            src,
            dst,
            weight,
        } in self.edges[start].clone().values()
        {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_frow_test() {
        let mut g = graph::Graph::new(5);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 0, 0);
        g.add_edge(0, 2, 1);
        g.add_edge(2, 0, 0);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 1, 0);
        g.add_edge(1, 3, 1);
        g.add_edge(3, 1, 0);
        g.add_edge(2, 3, 2);
        g.add_edge(3, 2, 0);

        assert_eq!(g.max_frow(0, 3), 3);
    }
}
