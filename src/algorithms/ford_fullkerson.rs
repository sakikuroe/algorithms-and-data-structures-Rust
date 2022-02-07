const INF: usize = std::usize::MAX / 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct FrowEdge {
    src: usize,
    dst: usize,
    cap: usize,
    rev: usize,
}

impl FrowEdge {
    fn new(src: usize, dst: usize, cap: usize, rev: usize) -> FrowEdge {
        FrowEdge {
            src,
            dst,
            cap,
            rev,
        }
    }
}

#[derive(Debug, Clone)]
struct MaxFlowGraph {
    size: usize,
    edges: Vec<Vec<FrowEdge>>,
}

impl MaxFlowGraph {
    fn new(n: usize) -> Self {
        MaxFlowGraph {
            size: n,
            edges: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dst: usize, cap: usize) {
        let src_len = self.edges[dst].len();
        self.edges[src].push(FrowEdge::new(src, dst, cap, src_len));
        let dst_len = self.edges[src].len();
        self.edges[dst].push(FrowEdge::new(dst, src, 0, dst_len - 1));
    }
}

pub struct MaxFlow {
    graph: MaxFlowGraph,
}

impl MaxFlow {
    pub fn new(n: usize) -> Self {
        MaxFlow{
            graph: MaxFlowGraph::new(n),
        }
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, cap: usize) {
        self.graph.add_edge(src, dst, cap);
    }

    fn dfs(
        &mut self,
        start: usize,
        goal: usize,
        flow: usize,
        visited: &mut Vec<bool>,
    ) -> usize {
        if start == goal {
            return flow;
        }
        visited[start] = true;
        for i in 0..self.graph.edges[start].len() {
            let edge = self.graph.edges[start][i];
            if !visited[edge.dst] && edge.cap > 0 {
                let d = self.dfs(edge.dst, goal, std::cmp::min(flow, edge.cap), visited);
                if d > 0 {
                    self.graph.edges[edge.src][i].cap -= d;
                    self.graph.edges[edge.dst][edge.rev].cap += d;
                    return d;
                }
            }
        }

        return 0;
    }

    pub fn max_frow(&mut self, start: usize, goal: usize) -> usize {
        let mut flow = 0;
        loop {
            let f = self.dfs(start, goal, INF, &mut vec![false; self.graph.size]);
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
        let mut g = MaxFlow::new(6);
        g.add_edge(0, 1, 2);
        g.add_edge(0, 3, 5);
        g.add_edge(1, 2, 3);
        g.add_edge(2, 5, 4);
        g.add_edge(3, 2, 2);
        g.add_edge(3, 4, 1);
        g.add_edge(4, 5, 1);

        assert_eq!(g.max_frow(0, 3), 5);
    }
}
