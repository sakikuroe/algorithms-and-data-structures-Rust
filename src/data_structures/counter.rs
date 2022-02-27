use std::{collections::HashMap, hash::Hash, ops};

#[derive(Clone, Debug)]
pub struct Counter<T> {
    pub counter: HashMap<T, isize>,
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

    pub fn sub(&mut self, key: T) {
        *self.counter.entry(key).or_insert(0) -= 1;
    }

    pub fn keys(&self) -> Vec<T> {
        self.counter.clone().into_keys().collect()
    }

    pub fn values(&self) -> Vec<isize> {
        self.counter.clone().into_values().collect()
    }

    pub fn items(&self) -> Vec<(T, isize)> {
        let mut res = self.counter.clone().into_iter().collect::<Vec<_>>();
        res.sort();
        res
    }

    pub fn get(&self, key: T) -> isize {
        match self.counter.get(&key) {
            Some(&cnt) => cnt,
            None => 0,
        }
    }

    pub fn get_common(&self) -> Vec<(T, isize)> {
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

impl<T> ops::Add for Counter<T>
where
    T: Clone + Copy + Eq + Hash + Ord,
{
    type Output = Counter<T>;
    fn add(self, other: Self) -> Self {
        let mut res = self.clone();
        for (key, cnt) in other.items() {
            *res.counter.entry(key).or_insert(0) += cnt;
        }
        res
    }
}

impl<T> ops::Sub for Counter<T>
where
    T: Clone + Copy + Eq + Hash + Ord,
{
    type Output = Counter<T>;
    fn sub(self, other: Self) -> Self {
        let mut res = self.clone();
        for (key, cnt) in other.items() {
            *res.counter.entry(key).or_insert(0) -= cnt;
        }
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

    #[test]
    fn op_works() {
        let mut counter1 = Counter::new();
        for x in vec![3, 1, 4, 1, 5, 9, 2] {
            counter1.add(x);
        }
        let mut counter2 = Counter::new();
        for x in vec![6, 5, 3, 5, 8, 9, 7, 9] {
            counter2.add(x);
        }

        let mut counter3 = Counter::new();
        for x in vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9] {
            counter3.add(x);
        }

        assert_eq!((counter1 + counter2).get_common(), counter3.get_common());
    }
}
