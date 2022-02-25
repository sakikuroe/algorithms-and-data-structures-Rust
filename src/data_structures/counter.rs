use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug)]
pub struct Counter<T> {
    pub counter: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Clone + Copy + Eq + Hash + Ord,
{
    pub fn new() -> Self {
        let counter = HashMap::new();
        Counter { counter }
    }

    pub fn add(&mut self, key: T) {
        *self.counter.entry(key).or_insert(0) += 1;
    }

    pub fn keys(&self) -> Vec<T> {
        self.counter.clone().into_keys().collect()
    }

    pub fn values(&self) -> Vec<usize> {
        self.counter.clone().into_values().collect()
    }

    pub fn items(&self) -> Vec<(T, usize)> {
        let mut res = self.counter.clone().into_iter().collect::<Vec<_>>();
        res.sort();
        res
    }

    pub fn get(&self, key: T) -> usize {
        match self.counter.get(&key) {
            Some(&cnt) => cnt,
            None => 0,
        }
    }

    pub fn get_common(&self) -> Vec<(T, usize)> {
        let mut res = self.counter.clone().into_iter().collect::<Vec<_>>();
        res.sort_by(|&(t_x, cnt_x), &(t_y, cnt_y)| {
            if cnt_x == cnt_y {
                (t_x).cmp(&t_y)
            } else {
                (cnt_y).cmp(&cnt_x)
            }
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_usize() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut counter = Counter::new();
        for x in v {
            counter.add(x);
        }
        assert_eq!(
            vec![(1, 2), (3, 2), (5, 2), (2, 1), (4, 1), (6, 1), (9, 1)],
            counter.get_common()
        );
    }

    #[test]
    fn case_str() {
        let v = vec!["abc", "def", "ghi", "def", "def"];
        let mut counter = Counter::new();
        for x in v {
            counter.add(x);
        }

        assert_eq!(vec![("abc", 1), ("def", 3), ("ghi", 1)], counter.items());

        assert_eq!(
            vec![("def", 3), ("abc", 1), ("ghi", 1)],
            counter.get_common()
        );

        assert_eq!(counter.get("abc"), 1);
        assert_eq!(counter.get("def"), 3);
        assert_eq!(counter.get("ghi"), 1);
        assert_eq!(counter.get("jkl"), 0);
    }
}
