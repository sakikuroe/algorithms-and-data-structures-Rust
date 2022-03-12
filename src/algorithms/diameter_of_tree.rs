use std::collections::VecDeque;

use crate::data_structures::graph::Graph;

impl Graph<usize> {
    pub fn is_tree(&self) -> bool {
        let mut visited = vec![false; self.size()];
        let mut que = {
            let mut res = VecDeque::new();
            res.push_back((0, None));
            res
        };
        visited[0] = true;

        while let Some((node, prev)) = que.pop_front() {
            for e in &self.edges[node] {
                if !visited[e.dst] {
                    visited[e.dst] = true;
                    que.push_back((e.dst, Some(e.src)));
                } else {
                    match prev {
                        Some(node) => {
                            if node != e.dst {
                                return false;
                            }
                        }
                        None => (),
                    }
                }
            }
        }
        return visited.into_iter().fold(true, |x, y| x && y);
    }

    pub fn get_diameter(&self) -> Result<usize, &str> {
        if !self.is_tree() {
            return Err("This graph is not a tree.");
        }
        fn arg_max(v: &Vec<usize>) -> usize {
            v.into_iter()
                .enumerate()
                .fold(
                    (0, 0),
                    |(i, max), (j, &x)| if max < x { (j, x) } else { (i, max) },
                )
                .0
        }
        let new_src = arg_max(&self.dijkstra(0));
        Ok(self
            .dijkstra(new_src)
            .into_iter()
            .fold(0, |x, y| std::cmp::max(x, y)))
    }
}
