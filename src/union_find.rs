use std::{collections::BTreeSet, fmt};
#[derive(Clone)]
pub struct UnionFind {
        n: usize,
        parent: Vec<usize>,
        rank: Vec<usize>,
        size: Vec<usize>,
}
#[allow(dead_code)]
impl UnionFind {
        pub fn new(number_of_nodes: usize) -> Self {
                let n = number_of_nodes;
                let parent: Vec<usize> = (0..number_of_nodes).collect();
                let rank = vec![0; number_of_nodes];
                let size = vec![1; number_of_nodes];
                UnionFind {
                        n,
                        parent,
                        rank,
                        size,
                }
        }
        pub fn find(&mut self, x: usize) -> usize {
                if self.parent[x] == x {
                        x
                } else {
                        self.parent[x] = self.find(self.parent[x]);
                        self.parent[x]
                }
        }
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
                self.find(x) == self.find(y)
        }
        fn is_root(&mut self, x: usize) -> bool {
                self.find(x) == x
        }
        pub fn size(&mut self, x: usize) -> usize {
                if self.is_root(x) {
                        self.size[x]
                } else {
                        let root_x = self.find(x);
                        self.size(root_x)
                }
        }
        pub fn union(&mut self, x: usize, y: usize) {
                let (x, y) = (self.find(x), self.find(y));
                if x != y {
                        if self.rank[x] < self.rank[y] {
                                self.size[y] += self.size(x);
                                self.parent[x] = y;
                        } else {
                                self.size[x] += self.size(y);
                                self.parent[y] = x;
                                if self.rank[x] == self.rank[y] {
                                        self.rank[x] += 1;
                                }
                        }
                }
        }
}
impl fmt::Display for UnionFind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut uf = (*self).clone();
                let n = uf.n;
                let mut res: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
                for i in 0..n {
                        res[uf.find(i)].insert(i);
                }
                res.sort();
                write!(f, "{:?}", res.into_iter().filter(|x| !x.is_empty()).collect::<Vec<BTreeSet<usize>>>())
        }
}
#[cfg(test)]
mod tests {
        use super::UnionFind;

        #[test]
        fn it_works() {
                let mut union_find = UnionFind::new(6);
                assert_eq!(format!("{}", union_find), "[{0}, {1}, {2}, {3}, {4}, {5}]");
                assert_eq!(union_find.find(0), union_find.find(0));
                assert_ne!(union_find.find(0), union_find.find(1));
                assert_ne!(union_find.find(0), union_find.find(2));
                assert_ne!(union_find.find(1), union_find.find(2));

                union_find.union(0, 1);
                assert_eq!(format!("{}", union_find), "[{0, 1}, {2}, {3}, {4}, {5}]");

                union_find.union(0, 2);
                assert_eq!(format!("{}", union_find), "[{0, 1, 2}, {3}, {4}, {5}]");

                union_find.union(2, 3);
                assert_eq!(format!("{}", union_find), "[{0, 1, 2, 3}, {4}, {5}]");

                union_find.union(5, 4);
                assert_eq!(format!("{}", union_find), "[{0, 1, 2, 3}, {4, 5}]");

                assert_eq!(union_find.find(0), union_find.find(1));
                assert_eq!(union_find.find(0), union_find.find(3));
                assert_eq!(union_find.find(4), union_find.find(5));
                assert!(!union_find.is_same(0, 5));
                assert!(union_find.is_same(4, 5));
        }
}
