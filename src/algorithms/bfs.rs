use crate::data_structures::graph::Graph;
use std::collections::VecDeque;
const INF: usize = std::usize::MAX;

impl<T> Graph<T>
where
    T: Clone + Copy + Eq,
{
    pub fn bfs(&mut self, starts: Vec<usize>) -> Vec<usize> {
        let mut res = vec![INF; self.size()];
        let mut que = VecDeque::new();
        for start in starts {
            res[start] = 0;
            que.push_back(start);
        }

        while let Some(node) = que.pop_front() {
            for e in &self.edges[node] {
                if res[e.dst] == INF {
                    res[e.dst] = res[e.src] + 1;
                    que.push_back(e.dst);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_test() {
        let mut g = Graph::new(10);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(1, 6, 1);
        g.add_edge(2, 3, 1);
        g.add_edge(2, 4, 1);
        g.add_edge(4, 5, 1);

        assert_eq!(g.bfs(vec![0]), vec![0, 1, 2, 3, 3, 4, 2, INF, INF, INF]);
    }
}
