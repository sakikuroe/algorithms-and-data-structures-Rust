pub struct FenwickTree<T> {
    size: usize,
    data: Vec<T>,
    element: T,
}

// [CAUTION] 1-based numbering
impl<T> FenwickTree<T>
where
    T: Clone + std::ops::Add<Output = T> + std::ops::AddAssign,
{
    pub fn new(size: usize, element: T) -> FenwickTree<T> {
        FenwickTree {
            size,
            data: vec![element.clone(); size + 1],
            element,
        }
    }

    pub fn add(&mut self, mut idx: usize, a: T) {
        while idx <= self.size {
            self.data[idx] = self.data[idx].clone() + a.clone();
            idx += (!idx + 1) & idx;
        }
    }

    // return data[1] + ... + data[idx];
    pub fn sum(&self, mut idx: usize) -> T {
        let mut res = self.element.clone();
        while idx > 0 {
            res += self.data[idx].clone();
            idx -= (!idx + 1) & idx;
        }
        res
    }

    pub fn lower_bound(&mut self, key: T) -> usize
    where
        T: Ord,
    {
        let mut sum = self.element.clone();
        let mut ng = 0;
        let mut len = 1;

        //Get most significant bit of self.size
        while self.size & !len >= len {
            len <<= 1;
        }

        while len > 0 {
            let idx = ng + len;
            if idx <= self.size && sum.clone() + self.data[idx].clone() < key {
                sum += self.data[idx].clone();
                ng += len;
            }
            len >>= 1;
        }
        ng + 1
    }
}
