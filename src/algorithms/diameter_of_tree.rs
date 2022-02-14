use crate::data_structures::graph::Graph;

impl Graph<usize> {
    pub fn get_diameter(&self) -> Option<usize> {
        if !self.is_tree() {
            return None;
        }
        fn arg_max(v: &Vec<usize>) -> usize {
            v.into_iter()
                .enumerate()
                .fold((0, 0), |(i, max), (j, &x)| if max < x { (j, x) } else { (i, max) })
                .0
        }
        let new_src = arg_max(&self.gen_undirected_graph().dijkstra(0));
        Some(
            self.gen_undirected_graph()
                .dijkstra(new_src)
                .into_iter()
                .fold(0, |x, y| std::cmp::max(x, y)),
        )
    }
}
