use std::ops;
const MOD: usize = 1_000_000_007;
#[derive(Clone, Debug, PartialEq, Eq)]
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
        if n == 0 {
            Mint::new(1)
        } else if n % 2 == 0 {
            Mint::new((self.clone() * self.clone()).pow(n / 2).value % MOD)
        } else {
            Mint::new((self.clone() * self.clone().pow(n - 1)).value % MOD)
        }
    }
    pub fn inverse(&self) -> Mint {
        self.pow(MOD - 2)
    }
    pub fn factorial(&self) -> Mint {
        if self.value == 0 {
            Mint::new(1)
        } else {
            let mut res = Mint::new(1);
            for i in 1..=self.value {
                res *= Mint::new(i);
            }
            res
        }
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
        self.value = (self.clone() + other).value;
    }
}
impl ops::SubAssign for Mint {
    fn sub_assign(&mut self, other: Self) {
        self.value = (self.clone() - other).value;
    }
}
impl ops::MulAssign for Mint {
    fn mul_assign(&mut self, other: Self) {
        self.value = (self.clone() * other).value;
    }
}
impl ops::DivAssign for Mint {
    fn div_assign(&mut self, other: Self) {
        self.value = (self.clone() / other).value;
    }
}
#[allow(dead_code)]
pub fn permutation(n: Mint, r: Mint) -> Mint {
    let mut res = Mint::new(1);
    for i in 0..r.value {
        res *= Mint::new(n.value - i);
    }
    res
}
#[allow(dead_code)]
pub fn combination(n: Mint, r: Mint) -> Mint {
    r.factorial().inverse() * permutation(n, r)
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
        assert_eq!(Mint::new(5).factorial(), Mint::new(120));
        assert_eq!(combination(Mint::new(7), Mint::new(3)), Mint::new(35));
        assert_eq!(combination(Mint::new(7), Mint::new(3)).value(), 35);
    }
}
