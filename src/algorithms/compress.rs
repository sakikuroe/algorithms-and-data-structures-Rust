use std::{collections::HashMap, iter::FromIterator};

pub struct Compress {
    pub coordinate: Vec<usize>,
    pub scale: Vec<isize>,
}

impl Compress {
    pub fn new(v: &Vec<isize>, margin: bool) -> Compress {
        let vs = {
            let mut vs = vec![];
            if margin {
                for &x in v {
                    vs.push(x - 1);
                    vs.push(x);
                    vs.push(x + 1);
                }
            } else {
                for &x in v {
                    vs.push(x);
                }
            }
            vs.sort();
            vs.dedup();
            vs
        };

        let d = HashMap::<isize, usize>::from_iter(
            vs.iter().enumerate().map(|(a, &b)| (b, a)),
        );

        let coordinate = v.iter().map(|a| *d.get(a).unwrap()).collect();

        Compress {
            coordinate,
            scale: vs,
        }
    }
}

pub struct Compress2 {
    pub coordinate1: Vec<usize>,
    pub coordinate2: Vec<usize>,
    pub scale: Vec<isize>,
}

impl Compress2 {
    pub fn new(v1: &Vec<isize>, v2: &Vec<isize>, margin: bool) -> Compress2 {
        let vs = {
            let mut vs = vec![];
            if margin {
                for &x in v1 {
                    vs.push(x - 1);
                    vs.push(x);
                    vs.push(x + 1);
                }
                for &x in v2 {
                    vs.push(x - 1);
                    vs.push(x);
                    vs.push(x + 1);
                }
            } else {
                for &x in v1 {
                    vs.push(x);
                }
                for &x in v2 {
                    vs.push(x);
                }
            }
            vs.sort();
            vs.dedup();
            vs
        };

        let d = HashMap::<isize, usize>::from_iter(
            vs.iter().enumerate().map(|(a, &b)| (b, a)),
        );

        let coordinate1 = v1.iter().map(|a| *d.get(a).unwrap()).collect();

        let coordinate2 = v2.iter().map(|a| *d.get(a).unwrap()).collect();

        Compress2 {
            coordinate1,
            coordinate2,
            scale: vs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1_1() {
        let v = vec![3, 3, 1, 8, 1];
        let compress = Compress::new(&v, false);
        let cordinate = compress.coordinate;
        let scale = compress.scale;
        assert_eq!(cordinate, vec![1, 1, 0, 2, 0]);
        assert_eq!(scale, vec![1, 3, 8]);
    }

    #[test]
    fn case1_2() {
        let v = vec![3, 3, 1, 8, 1];
        let compress = Compress::new(&v, true);
        let cordinate = compress.coordinate;
        let scale = compress.scale;
        assert_eq!(cordinate, vec![3, 3, 1, 6, 1]);
        assert_eq!(scale, vec![0, 1, 2, 3, 4, 7, 8, 9]);
    }

    #[test]
    fn case2_1() {
        let v1 = vec![3, 3, 1, 7, 1];
        let v2 = vec![0, 2, 1, 9, 2];
        let compress = Compress2::new(&v1, &v2, false);
        let cordinate1 = compress.coordinate1;
        let cordinate2 = compress.coordinate2;
        let scale = compress.scale;
        assert_eq!(cordinate1, vec![3, 3, 1, 4, 1]);
        assert_eq!(cordinate2, vec![0, 2, 1, 5, 2]);
        assert_eq!(scale, vec![0, 1, 2, 3, 7, 9]);
    }

    #[test]
    fn case2_2() {
        let v1 = vec![3, 3, 1, 7, 1];
        let v2 = vec![0, 2, -1, 9, 2];
        let compress = Compress2::new(&v1, &v2, true);
        let cordinate1 = compress.coordinate1;
        let cordinate2 = compress.coordinate2;
        let scale = compress.scale;
        assert_eq!(cordinate1, vec![5, 5, 3, 8, 3]);
        assert_eq!(cordinate2, vec![2, 4, 1, 10, 4]);
        assert_eq!(scale, vec![-2, -1, 0, 1, 2, 3, 4, 6, 7, 8, 9, 10]);
    }
}
