pub fn mex(v: &Vec<usize>) -> usize {
    let mut f = vec![false; v.len()];
    for &x in v {
        if x < v.len() {
            f[x] = true;
        }
    }
    for i in 0..v.len() {
        if !f[i] {
            return i;
        }
    }
    v.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(mex(&vec![2, 5, 6, 2, 3]), 0);
        assert_eq!(mex(&vec![2, 0, 6, 4, 3]), 1);
        assert_eq!(mex(&vec![1, 2, 3, 5, 0]), 4);
    }
}