use std::ops;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ModInt<const MOD: usize> {
    value: usize,
}
impl<const MOD: usize> ModInt<MOD> {
    pub fn new(value: usize) -> ModInt<MOD> {
        ModInt {
            value: value % MOD,
        }
    }
    #[allow(dead_code)]
    pub fn value(&self) -> usize {
        self.value
    }
    pub fn pow(&self, mut n: usize) -> ModInt<MOD> {
        let mut res = ModInt::new(1);
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
    pub fn inverse(&self) -> ModInt<MOD> {
        self.pow(MOD - 2)
    }
}
impl<const MOD: usize> ops::Add for ModInt<MOD> {
    type Output = ModInt<MOD>;
    fn add(self, other: Self) -> Self {
        let mut value = self.value + other.value;
        if value >= MOD {
            value -= MOD;
        }
        ModInt {
            value,
        }
    }
}
impl<const MOD: usize> ops::Sub for ModInt<MOD> {
    type Output = ModInt<MOD>;
    fn sub(self, other: Self) -> Self {
        if self.value >= other.value {
            ModInt {
                value: self.value - other.value,
            }
        } else {
            ModInt {
                value: (self.value + MOD) - other.value,
            }
        }
    }
}
impl<const MOD: usize> ops::Mul for ModInt<MOD> {
    type Output = ModInt<MOD>;
    fn mul(self, other: Self) -> Self {
        let v = (self.value as u128 * other.value as u128) % (MOD as u128);
        ModInt {
            value: v as usize,
        }
    }
}
impl<const MOD: usize> ops::Div for ModInt<MOD> {
    type Output = ModInt<MOD>;
    fn div(self, other: Self) -> Self {
        ModInt {
            value: (self * other.inverse()).value % MOD,
        }
    }
}
impl<const MOD: usize> ops::AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, other: Self) {
        self.value = (*self + other).value;
    }
}
impl<const MOD: usize> ops::SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, other: Self) {
        self.value = (*self - other).value;
    }
}
impl<const MOD: usize> ops::MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, other: Self) {
        self.value = (*self * other).value;
    }
}
impl<const MOD: usize> ops::DivAssign for ModInt<MOD> {
    fn div_assign(&mut self, other: Self) {
        self.value = (*self / other).value;
    }
}

pub fn factorial<const MOD: usize>(n: usize) -> ModInt<MOD> {
    (1..=n).into_iter().fold(ModInt::new(1), |x, y| x * ModInt::new(y))
}

pub fn permutation<const MOD: usize>(n: usize, r: usize) -> ModInt<MOD> {
    let mut res = ModInt::new(1);
    for i in 0..r {
        res *= ModInt::new(n - i);
    }
    res
}

pub fn combination<const MOD: usize>(n: usize, r: usize) -> ModInt<MOD> {
    factorial(r).inverse() * permutation(n, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    type Mint = ModInt<1000000007>;
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
