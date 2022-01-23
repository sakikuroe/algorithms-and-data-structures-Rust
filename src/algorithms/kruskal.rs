use crate::data_structures::{graph, union_find};
use std::collections::HashSet;

impl graph::Graph {
    pub fn kruskal(&self) -> HashSet<graph::Edge> {
        let mut res = HashSet::new();
        let mut uf = union_find::UnionFind::new(self.size());
        let mut all_edges = self.get_all_edges().into_iter().collect::<Vec<_>>();
        all_edges.sort();
        for e in all_edges {
            if !uf.is_same(e.src, e.dst) {
                uf.union(e.src, e.dst);
                res.insert(e);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kruskal_test() {
        let mut g = graph::Graph::new(5);
        g.add_edge(0, 1, 3);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 1);
        g.add_edge(1, 4, 5);
        g.add_edge(2, 3, 1);
        g.add_edge(3, 4, 2);
        let ans = {
            let mut res = HashSet::new();
            res.insert(graph::Edge::new(0, 1, 3));
            res.insert(graph::Edge::new(1, 2, 1));
            res.insert(graph::Edge::new(2, 3, 1));
            res.insert(graph::Edge::new(3, 4, 2));
            res
        };
        assert_eq!(ans, g.kruskal());
    }
}
