use crate::data_structures::graph::Graph;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, PartialEq, Eq)]
pub struct Node<T> {
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
    pub fn new(priority: T) -> Self {
        Node { priority }
    }
}

impl<T> Graph<T>
where
    T: Clone + Copy + Eq,
{
    pub fn topological_sort(&self) -> Result<Vec<usize>, &str> {
        let mut indegree = {
            let mut res = vec![0; self.size()];
            for edges in &self.edges {
                for e in edges {
                    res[e.dst] += 1;
                }
            }
            res
        };
        let mut sorted = vec![];
        let mut que = {
            let mut que = BinaryHeap::new();
            for i in 0..self.size() {
                if indegree[i] == 0 {
                    que.push(Node::new(i));
                }
            }
            que
        };
        while let Some(Node { priority }) = que.pop() {
            sorted.push(priority);
            for e in &self.edges[priority] {
                indegree[e.dst] -= 1;
                if indegree[e.dst] == 0 {
                    que.push(Node::new(e.dst))
                }
            }
        }

        if sorted.len() == self.size() {
            Ok(sorted)
        } else {
            Err("This graph could not be topologically sorted.")
        }
    }
}
