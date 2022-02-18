use std::{
    collections::{BTreeSet, HashSet},
    fmt, mem,
};

#[derive(Clone)]
pub struct UnionFind {
    number_of_nodes: usize,
    number_of_connected_components: usize,
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(number_of_nodes: usize) -> Self {
        UnionFind {
            number_of_nodes,
            number_of_connected_components: number_of_nodes,
            parent: (0..number_of_nodes).collect::<Vec<usize>>(),
            rank: vec![0; number_of_nodes],
            size: vec![1; number_of_nodes],
        }
    }

    pub fn get_number_of_connected_components(&self) -> usize {
        self.number_of_connected_components
    }

    pub fn get_number_of_nodes(&self) -> usize {
        self.number_of_nodes
    }

    pub fn is_root(&mut self, x: usize) -> bool {
        self.parent[x] == x
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.is_root(x) {
            x
        } else {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
            root
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let (mut root_x, mut root_y) = (self.find(x), self.find(y));

        if root_x != root_y {
            if self.rank[root_x] < self.rank[root_y] {
                mem::swap(&mut root_x, &mut root_y);
            }

            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            if self.rank[root_x] == self.rank[root_y] {
                self.rank[root_x] += 1;
            }
            self.number_of_connected_components -= 1;
        }
    }

    pub fn size(&mut self, x: usize) -> usize {
        if self.is_root(x) {
            self.size[x]
        } else {
            let root = self.find(x);
            self.size[root]
        }
    }

    pub fn get_value(&mut self) -> HashSet<usize> {
        HashSet::new()
    }
}

impl fmt::Display for UnionFind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uf = (*self).clone();
        let n = uf.number_of_nodes;
        let mut res: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
        for i in 0..n {
            res[uf.find(i)].insert(i);
        }
        res.sort();
        write!(
            f,
            "{:?}",
            res.into_iter()
                .filter(|x| !x.is_empty())
                .collect::<BTreeSet<BTreeSet<usize>>>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn it_works() {
        let mut union_find = UnionFind::new(6);
        assert_eq!(format!("{}", union_find), "{{0}, {1}, {2}, {3}, {4}, {5}}");
        assert_eq!(union_find.find(0), union_find.find(0));
        assert_ne!(union_find.find(0), union_find.find(1));
        assert_ne!(union_find.find(0), union_find.find(2));
        assert_ne!(union_find.find(1), union_find.find(2));

        union_find.union(0, 1);
        assert_eq!(format!("{}", union_find), "{{0, 1}, {2}, {3}, {4}, {5}}");

        union_find.union(0, 2);
        assert_eq!(format!("{}", union_find), "{{0, 1, 2}, {3}, {4}, {5}}");

        union_find.union(2, 3);
        assert_eq!(format!("{}", union_find), "{{0, 1, 2, 3}, {4}, {5}}");

        union_find.union(5, 4);
        assert_eq!(format!("{}", union_find), "{{0, 1, 2, 3}, {4, 5}}");

        assert_eq!(union_find.get_number_of_connected_components(), 2);
        assert_eq!(union_find.find(0), union_find.find(1));
        assert_eq!(union_find.find(0), union_find.find(3));
        assert_eq!(union_find.find(4), union_find.find(5));
        assert!(!union_find.is_same(0, 5));
        assert!(union_find.is_same(4, 5));

        assert_eq!(union_find.size(0), 4);
        assert_eq!(union_find.size(1), 4);
        assert_eq!(union_find.size(2), 4);
        assert_eq!(union_find.size(3), 4);
        assert_eq!(union_find.size(4), 2);
        assert_eq!(union_find.size(5), 2);
    }
}
