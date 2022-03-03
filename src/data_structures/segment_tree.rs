use crate::algebraic_structures::monoid::Monoid;

pub struct SegmentTree<M>
where
    M: Monoid,
{
    size: usize,
    data: Vec<M::S>,
}

impl<M> SegmentTree<M>
where
    M: Monoid,
    M::S: Clone + Copy,
{
    pub fn new(size: usize) -> Self {
        let size = size.next_power_of_two();
        SegmentTree::<M> {
            size,
            data: vec![M::id(); 2 * size - 1],
        }
    }

    pub fn update(&mut self, mut idx: usize, x: M::S) {
        idx += self.size - 1;
        self.data[idx] = x;
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.data[idx] = M::op(self.data[2 * idx + 1], self.data[2 * idx + 2]);
        }
    }

    pub fn reduce(&self, mut l: usize, mut r: usize) -> M::S {
        l += self.size - 1;
        r += self.size - 1;
        let mut x1 = M::id();
        let mut x2 = M::id();
        while l < r {
            if l % 2 == 0 {
                x1 = M::op(x1, self.data[l]);
            }
            if r % 2 == 0 {
                x2 = M::op(self.data[r - 1], x2);
            }
            l = l / 2;
            r = (r - 1) / 2;
        }
        M::op(x1, x2)
    }
}
