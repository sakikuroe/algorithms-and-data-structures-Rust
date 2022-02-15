use crate::data_structures::mint::ModInt;

pub struct BinomicalCoeff<const MOD: usize> {
    factorial_table: Vec<ModInt<MOD>>,
    factorial_inv_table: Vec<ModInt<MOD>>,
}

impl<const MOD: usize> BinomicalCoeff<MOD> {
    pub fn new(max_size: usize) -> BinomicalCoeff<MOD> {
        let mut factorial_table = vec![];

        let mut factorial = ModInt::<MOD>::new(1);
        factorial_table.push(factorial);
        for i in 1..=max_size {
            factorial *= ModInt::<MOD>::new(i);
            factorial_table.push(factorial);
        }

        let mut factorial_inv_table = vec![];
        let mut s_inv = factorial_table[max_size].inverse();
        factorial_inv_table.push(s_inv);
        for i in (1..=max_size).rev() {
            s_inv *= ModInt::<MOD>::new(i);
            factorial_inv_table.push(s_inv);
        }
        factorial_inv_table.reverse();

        BinomicalCoeff {
            factorial_table,
            factorial_inv_table,
        }
    }

    pub fn get_value(&self, n: usize, r: usize) -> ModInt<MOD> {
        if n < r {
            return ModInt::<MOD>::new(0);
        }
        self.factorial_table[n]
            * self.factorial_inv_table[r]
            * self.factorial_inv_table[n - r]
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::mint::combination;

    use super::*;
    const MOD: usize = 1000000007;
    #[test]
    fn it_works() {
        let bc = BinomicalCoeff::<MOD>::new(10000);
        assert_eq!(bc.get_value(5, 2).value(), 10);
        assert_eq!(bc.get_value(5, 1).value(), 5);
        assert_eq!(bc.get_value(5, 0).value(), 1);

        assert_eq!(bc.get_value(10000, 400), combination(10000, 400));
    }
}
