use std::hash::Hash;

use crate::data_structures::graph;

impl<T> graph::Graph<T>
where
    T: Clone + Copy + Eq + Hash,
{
    pub fn max_clique(&self) -> usize {
        self.gen_complement().max_independent()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
