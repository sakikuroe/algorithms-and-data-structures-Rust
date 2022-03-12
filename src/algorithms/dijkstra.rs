use crate::{chmin, data_structures::graph};
use std::{cmp::Ordering, collections::BinaryHeap};
const INF: usize = std::usize::MAX;

#[derive(Clone, PartialEq, Eq)]
pub struct Node<T> {
    pub id: usize,
    pub priority: T,
}

impl<T> Ord for Node<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
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

impl<T> Node<T>
where
    T: Ord,
{
    pub fn new(id: usize, priority: T) -> Self {
        Node { id, priority }
    }
}

impl graph::Graph<usize> {
    pub fn dijkstra(&self, start: usize) -> Vec<usize> {
        let mut res = vec![INF; self.size()];
        res[start] = 0;
        let mut que = BinaryHeap::new();
        que.push(Node::new(start, 0));

        while let Some(Node { id, priority }) = que.pop() {
            if priority > res[id] {
                continue;
            }
            for e in &self.edges[id] {
                if chmin!(res[e.dst], res[e.src] + e.weight) {
                    que.push(Node::new(e.dst, res[e.dst]));
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
