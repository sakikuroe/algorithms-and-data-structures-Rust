use crate::data_structures::graph;

impl graph::Graph {
    pub fn max_clique(&self) -> usize {
        self.gen_complement().max_independent()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
