use std::{collections::HashMap, hash::Hash, mem};

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy
        + Ord
        + From<u8>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign,
{
    if a < b {
        mem::swap(&mut a, &mut b);
    }
    while b != 0.into() {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + Ord
        + From<u8>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign,
{
    a * b / gcd(a, b)
}

pub fn is_prime<T>(n: T) -> bool
where
    T: Copy
        + Ord
        + From<u8>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign,
{
    if n <= 1.into() {
        return false;
    }

    let mut i = 2.into();
    while i * i <= n {
        if n % i == 0.into() {
            return false;
        }
        i += 1.into();
    }

    return true;
}

pub fn gen_divisors<T>(n: T) -> Vec<T>
where
    T: Copy
        + Ord
        + From<u8>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign,
{
    let mut res = vec![];

    let mut i = 1.into();
    while i * i <= n {
        if n % i == 0.into() {
            res.push(i);
            if i * i != n {
                res.push(n / i);
            }
        }
        i += 1.into();
    }

    res.sort();
    res
}

pub fn integer_factorization<T>(mut n: T) -> HashMap<T, usize>
where
    T: Copy
        + Ord
        + Hash
        + From<u8>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign,
{
    let mut res = HashMap::new();

    let mut i = 2.into();
    while i * i <= n {
        while n % i == 0.into() {
            *res.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1.into();
    }

    if n != 0.into() && n != 1.into() {
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
