pub trait BinarySearch<T> {
    fn lower_bound(&self, key: T) -> usize;
    fn upper_bound(&self, key: T) -> usize;
}

impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    fn lower_bound(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key <= self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    fn upper_bound(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key < self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::binary_search::BinarySearch;

    #[test]
    fn it_works() {
        assert_eq!(vec![0, 0, 0, 1, 1, 3].lower_bound(-1), 0);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].lower_bound(0), 0);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].lower_bound(1), 3);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].lower_bound(2), 5);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].lower_bound(3), 5);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].lower_bound(10), 6);

        assert_eq!(vec![0, 0, 0, 1, 1, 3].upper_bound(0), 3);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].upper_bound(1), 5);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].upper_bound(2), 5);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].upper_bound(3), 6);
        assert_eq!(vec![0, 0, 0, 1, 1, 3].upper_bound(10), 6);
    }
}