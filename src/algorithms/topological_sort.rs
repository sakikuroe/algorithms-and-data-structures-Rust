use std::{collections::BinaryHeap, hash::Hash};

use crate::data_structures::graph::{Graph, Node};

impl<T> Graph<T>
where
    T: Clone + Copy + Eq + Hash,
{
    pub fn topological_sort(&self) -> Result<Vec<usize>, &str> {
        let mut indegree = {
            let mut res = vec![0; self.size()];
            for edge_list in &self.edges {
                for (_, &e) in edge_list {
                    res[e.dst] += 1;
                }
            }
            res
        };
        let res = {
            let mut sorted: Vec<usize> = vec![];
            let mut que = {
                let mut que = BinaryHeap::new();
                for i in 0..self.size() {
                    if indegree[i] == 0 {
                        que.push(Node {
                            priority: i,
                            vertex: i,
                        });
                    }
                }
                que
            };
            while let Some(Node { priority, vertex: _ }) = que.pop() {
                for (_, &e) in &self.edges[priority] {
                    indegree[e.dst] -= 1;
                    if indegree[e.dst] == 0 {
                        que.push(Node {
                            priority: e.dst,
                            vertex: e.dst,
                        })
                    }
                }
                sorted.push(priority);
            }

            if sorted.len() == self.size() {
                Ok(sorted)
            } else {
                Err("-1")
            }
        };
        res
    }
}
