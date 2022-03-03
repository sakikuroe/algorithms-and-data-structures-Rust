use crate::{chmin, min, data_structures::graph};
use std::{collections::BinaryHeap, usize::MAX};
const INF: usize = MAX / 3;

impl graph::Graph<usize> {
    pub fn dijkstra(&self, start: usize) -> Vec<usize> {
        let mut res = vec![INF; self.size()];
        res[start] = 0;
        let mut que = BinaryHeap::new();
        que.push(graph::Node {
            vertex: start,
            priory: 0,
        });

        while let Some(graph::Node { vertex, priory }) = que.pop() {
            if priory > res[vertex] {
                continue;
            }
            for &e in self.edges[vertex].values() {
                if chmin!(res[e.dst], res[e.src] + e.weight) {
                    que.push(graph::Node {
                        vertex: e.dst,
                        priory: res[e.dst],
                    });
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
    fn dijkstra_test() {
        let mut g = graph::Graph::new(10);
        g.add_edge(0, 1, 5);
        g.add_edge(0, 2, 3);
        g.add_edge(0, 3, 2);
        g.add_edge(0, 5, 2);
        g.add_edge(1, 3, 1);
        g.add_edge(2, 4, 1);
        g.add_edge(3, 4, 1);
        g.add_edge(4, 5, 3);

        assert_eq!(g.dijkstra(0), vec![0, 5, 3, 2, 3, 2, INF, INF, INF, INF]);
    }
}
