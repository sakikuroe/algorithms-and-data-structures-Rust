use std::{collections::HashMap, hash::Hash, iter::FromIterator};

pub struct Compress<T> {
    pub coordinate: Vec<usize>,
    pub scale: Vec<T>,
}

impl<T> Compress<T>
where
    T: Clone + Copy + Ord + Hash,
{
    pub fn new(v: &Vec<T>) -> Compress<T> {
        let vs = {
            let mut vs = v.clone();
            vs.sort();
            vs.dedup();
            vs
        };

        let coordinate = {
            let d = HashMap::<T, usize>::from_iter(
                vs.iter().enumerate().map(|(a, &b)| (b, a)),
            );

            v.iter().map(|a| *d.get(a).unwrap()).collect()
        };

        Compress {
            coordinate,
            scale: vs,
        }
    }
}
