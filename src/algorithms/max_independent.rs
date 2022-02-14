use std::hash::Hash;

use crate::data_structures::graph;

impl<T> graph::Graph<T>
where
    T: Copy + Clone + Eq + Hash,
{
    pub fn max_independent(&self) -> usize {
        let n1 = (self.size() + 1) / 2;
        let n2 = self.size() / 2;

        let mut ok = vec![true; 1 << n1];
        for v in 0..n1 {
            for graph::Edge {
                src,
                dst,
                weight: _,
            } in self.edges(v)
            {
                if dst < n1 {
                    ok[(1 << src) | (1 << dst)] = false;
                }
            }
        }
        for bit in 0..(1 << n1) {
            if ok[bit] == false {
                for w in 0..n1 {
                    ok[bit | (1 << w)] = false;
                }
            }
        }

        let mut set = vec![(1 << n2) - 1; 1 << n1];
        for v in 0..n1 {
            set[1 << v] = (1 << n2) - 1;
            for graph::Edge {
                src,
                dst,
                weight: _,
            } in self.edges(v)
            {
                if src < n1 && n1 <= dst {
                    set[1 << src] &= !(1 << (dst - n1));
                }
            }
        }
        for bit in 0..(1 << n1) {
            for w in 0..n1 {
                set[bit | (1 << w)] = set[bit] & set[1 << w];
            }
        }

        let mut ok2 = vec![true; 1 << n2];
        for v in 0..n2 {
            for graph::Edge {
                src,
                dst,
                weight: _,
            } in self.edges(v + n1)
            {
                if n1 <= src {
                    ok2[(1 << (src - n1)) | (1 << (dst - n1))] = false;
                }
            }
        }
        for bit in 0..(1 << n2) {
            if !ok2[bit] {
                for w in 0..n2 {
                    ok2[bit | (1 << w)] = false;
                }
            }
        }

        let mut dp = vec![0; 1 << n2];
        for bit in 0..(1 << n2) {
            if ok2[bit] {
                dp[bit] = (bit as u64).count_ones();
            }
        }
        for bit in 0..(1 << n2) {
            for w in 0..n2 {
                dp[bit | (1 << w)] = std::cmp::max(dp[bit | (1 << w)], dp[bit]);
            }
        }
        let mut res = 0;
        for bit in 0..(1 << n1) {
            if !ok[bit] {
                continue;
            }
            res = std::cmp::max(res, (bit as u64).count_ones() + dp[set[bit]]);
        }
        res as usize
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
