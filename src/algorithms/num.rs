use std::{collections::HashMap, mem};

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn is_prime(n: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    for i in (2..).take_while(|&x| x * x <= n) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

pub fn gen_divisors(n: usize) -> Vec<usize> {
    let mut res = vec![];

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i * i != n {
                res.push(n / i);
            }
        }
        i += 1;
    }

    res.sort();
    res
}

pub fn integer_factorization(mut n: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();

    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            *res.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }

    if n != 0 && n != 1 {
        *res.entry(n).or_insert(0) += 1;
    }

    res
}

mod tests {
    pub use super::*;

    #[test]
    fn test_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        let map = {
            let mut res = HashMap::new();
            res.insert(2, 3);
            res.insert(5, 2);
            res
        };
        assert_eq!(map, integer_factorization(200));
    }

    #[test]
    fn test_lcm_and_gcd() {
        assert_eq!(lcm(3, 4), 12);
        assert_eq!(lcm(10, 20), 20);
        assert_eq!(lcm(100, 60), 300);
        assert_eq!(lcm(399, 4), 399 * 4);

        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(20, 30), 10);
        assert_eq!(gcd(4352, 432), 16);
        assert_eq!(gcd(101101, 2002), 1001);
    }

    #[test]
    fn test_divisors() {
        assert!(!is_prime(10));
    }
}
