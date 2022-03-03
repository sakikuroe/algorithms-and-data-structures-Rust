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
    pub fn new(n: usize) -> Self {
        let mut size = 1;
        while n > size {
            size <<= 1;
        }
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

    pub fn find_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> M::S {
        if r <= a || b <= l {
            return M::id();
        } else if a <= l && r <= b {
            return self.data[k];
        } else {
            return M::op(
                self.find_sub(a, b, k * 2 + 1, l, (l + r) / 2),
                self.find_sub(a, b, k * 2 + 2, (l + r) / 2, r),
            );
        }
    }

    pub fn find(&self, l: usize, r: usize) -> M::S {
        self.find_sub(l, r, 0, 0, self.size)
    }
}
