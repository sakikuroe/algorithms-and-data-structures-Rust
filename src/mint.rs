use std::ops;
const MOD: usize = 1_000_000_007;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Mint {
    value: usize,
}
impl Mint {
    pub fn new(value: usize) -> Mint {
        Mint {
            value: value % MOD,
        }
    }
    #[allow(dead_code)]
    pub fn value(&self) -> usize {
        self.value
    }
    pub fn pow(&self, n: usize) -> Mint {
        let mut res = Mint::new(1);
        let mut n = n;
        let mut x = self.clone();
        while n > 0 {
            if n & 1 == 1 {
                res *= x;
            }
            x = x * x;
            n = n >> 1;
        }

        res
    }
    pub fn inverse(&self) -> Mint {
        self.pow(MOD - 2)
    }
}
impl ops::Add for Mint {
    type Output = Mint;
    fn add(self, other: Self) -> Self {
        let mut value = self.value + other.value;
        if value >= MOD {
            value -= MOD;
        }
        Mint {
            value,
        }
    }
}
impl ops::Sub for Mint {
    type Output = Mint;
    fn sub(self, other: Self) -> Self {
        if self.value >= other.value {
            Mint {
                value: self.value - other.value,
            }
        } else {
            Mint {
                value: (self.value + MOD) - other.value,
            }
        }
    }
}
impl ops::Mul for Mint {
    type Output = Mint;
    fn mul(self, other: Self) -> Self {
        Mint {
            value: self.value * other.value % MOD,
        }
    }
}
impl ops::Div for Mint {
    type Output = Mint;
    fn div(self, other: Self) -> Self {
        Mint {
            value: (self * other.inverse()).value % MOD,
        }
    }
}
impl ops::AddAssign for Mint {
    fn add_assign(&mut self, other: Self) {
        self.value = (*self + other).value;
    }
}
impl ops::SubAssign for Mint {
    fn sub_assign(&mut self, other: Self) {
        self.value = (*self - other).value;
    }
}
impl ops::MulAssign for Mint {
    fn mul_assign(&mut self, other: Self) {
        self.value = (*self * other).value;
    }
}
impl ops::DivAssign for Mint {
    fn div_assign(&mut self, other: Self) {
        self.value = (*self / other).value;
    }
}

pub fn factorial(n: usize) -> Mint {
    if n == 0 {
        Mint::new(1)
    } else {
        let mut res = Mint::new(1);
        for i in 1..=n {
            res *= Mint::new(i);
        }
        res
    }
}

pub fn permutation(n: usize, r: usize) -> Mint {
    let mut res = Mint::new(1);
    for i in 0..r {
        res *= Mint::new(n - i);
    }
    res
}

pub fn combination(n: usize, r: usize) -> Mint {
    factorial(r).inverse() * permutation(n, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value() {
        assert_eq!(Mint::new(5).value(), 5 as usize);
    }

    #[test]
    fn operation() {
        assert_eq!(Mint::new(5) + Mint::new(3), Mint::new(8));
        assert_eq!(Mint::new(5) - Mint::new(3), Mint::new(2));
        assert_eq!(Mint::new(5) * Mint::new(3), Mint::new(15));

        assert_eq!(Mint::new(5 * 400000003).value(), 1);
        assert_eq!(Mint::new(5) * Mint::new(400000003), Mint::new(1));
        assert_eq!(Mint::new(1) / Mint::new(5), Mint::new(400000003));

        let mut v = Mint::new(10);
        v += Mint::new(5);
        assert_eq!(v, Mint::new(15));
        v -= Mint::new(8);
        assert_eq!(v, Mint::new(7));
        v *= Mint::new(7);
        assert_eq!(v, Mint::new(49));
        v /= Mint::new(10);
        assert_eq!(Mint::new(10 * 300000007).value(), 49);
        assert_eq!(v, Mint::new(300000007));
    }

    #[test]
    fn it_works() {
        assert_eq!(Mint::new(5).pow(3), Mint::new(125));
        assert_eq!(factorial(5), Mint::new(120));
        assert_eq!(permutation(7, 3), Mint::new(210));
        assert_eq!(combination(7, 3), Mint::new(35));
    }
}
