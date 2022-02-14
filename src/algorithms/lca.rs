use crate::data_structures::graph::{Edge, Graph};
use std::{collections::VecDeque, mem};

const INF: usize = std::usize::MAX / 3;

pub struct LCA {
    pub height: usize,
    pub parent: Vec<Vec<Option<usize>>>,
    pub depth: Vec<usize>,
}

impl LCA {
    pub fn new<T>(n: usize, root: usize, graph: &Graph<T>) -> LCA {
        let height = {
            let mut cnt = 1;
            while (1 << cnt) < n {
                cnt += 1;
            }
            cnt
        };
        let parent = vec![vec![None; n]; height];
        let depth = vec![INF; n];
        let mut res = LCA {
            height,
            parent,
            depth,
        };
        res.bfs(&graph, &vec![root]);
        for k in 1..height {
            for i in 0..n {
                if let Some(j) = res.parent[k - 1][i] {
                    res.parent[k][i] = res.parent[k - 1][j];
                }
            }
        }
        res
    }

    fn bfs<T>(&mut self, graph: &Graph<T>, start: &Vec<usize>) {
        let mut que = VecDeque::new();
        for &start in start {
            que.push_back(start);
            self.depth[start] = 0;
        }
        while let Some(current) = que.pop_front() {
            for &Edge {
                src,
                dst,
                weight: _,
            } in graph.edges[current].values()
            {
                if self.depth[dst] > self.depth[src] + 1 {
                    self.depth[dst] = self.depth[src] + 1;
                    self.parent[0][dst] = Some(current);
                    que.push_back(dst);
                }
            }
        }
    }

    pub fn value(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            mem::swap(&mut u, &mut v);
        }

        let diff = self.depth[v] - self.depth[u];
        for k in 0..self.height {
            if diff & (1 << k) != 0 {
                v = self.parent[k][v].unwrap();
            }
        }
        if u == v {
            u
        } else {
            for k in (0..self.height - 1).rev() {
                if self.parent[k][u] != self.parent[k][v] {
                    u = self.parent[k][u].unwrap();
                    v = self.parent[k][v].unwrap();
                }
            }
            self.parent[0][u].unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, LCA};

    #[test]
    fn case1() {
        let n = 7;
        let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)];
        let graph = {
            let mut res = Graph::new(n);
            for &(a, b) in &edges {
                res.add_edge(a, b, 1);
                res.add_edge(b, a, 1);
            }
            res
        };

        //    (root)
        //      0
        //    /   \
        //   1     2
        //  / \   / \
        // 3   4 5   6

        let lca = LCA::new(n, 0, &graph);

        assert_eq!(lca.value(1, 2), 0);
        assert_eq!(lca.value(3, 4), 1);
        assert_eq!(lca.value(5, 2), 2);
        assert_eq!(lca.value(6, 1), 0);
        assert_eq!(lca.value(2, 0), 0);
    }

    #[test]
    fn case2() {
        let n = 8;
        let edges = vec![(0, 1), (0, 2), (2, 3), (2, 4), (4, 5), (4, 6), (4, 7)];
        let graph = {
            let mut res = Graph::new(n);
            for &(a, b) in &edges {
                res.add_edge(a, b, 1);
                res.add_edge(b, a, 1);
            }
            res
        };

        //    (root)
        //      0
        //    /   \
        //   1     2
        //        / \
        //       3   4
        //          /|\
        //         5 6 7

        let lca = LCA::new(n, 0, &graph);

        assert_eq!(lca.value(0, 3), 0);
        assert_eq!(lca.value(3, 5), 2);
        assert_eq!(lca.value(5, 6), 4);
        assert_eq!(lca.value(7, 3), 2);
        assert_eq!(lca.value(2, 7), 2);
    }
}
