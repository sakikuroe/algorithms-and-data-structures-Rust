use std::{collections::HashMap, hash::Hash};

pub struct Counter<T> {
    counter: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Clone + Copy + Eq + Hash + Ord,
{
    pub fn new(v: &Vec<T>) -> Self {
        let mut counter = HashMap::new();
        for &x in v {
            *counter.entry(x).or_insert(0) += 1;
        }
        Counter {
            counter,
        }
    }

    pub fn add(&mut self, key: T) {
        *self.counter.entry(key).or_insert(0) += 1;
    }

    pub fn values(&mut self) -> Vec<usize> {
        self.counter.clone().into_values().collect()
    }

    pub fn get_common(&self) -> Vec<(T, usize)> {
        let mut res = self.counter.clone().into_iter().collect::<Vec<_>>();
        res.sort_by(
            |&(t_x, cnt_x), &(t_y, cnt_y)| {
                if cnt_x == cnt_y {
                    (t_x).cmp(&t_y)
                } else {
                    (cnt_y).cmp(&cnt_x)
                }
            },
        );
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_usize() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let counter = Counter::new(&v);
        assert_eq!(
            vec![(1, 2), (3, 2), (5, 2), (2, 1), (4, 1), (6, 1), (9, 1)],
            counter.get_common()
        );
    }

    #[test]
    fn case_str() {
        let v = vec!["abc", "def", "ghi", "abc", "abc"];
        let counter = Counter::new(&v);
        assert_eq!(vec![("abc", 3), ("def", 1), ("ghi", 1)], counter.get_common());
    }
}
