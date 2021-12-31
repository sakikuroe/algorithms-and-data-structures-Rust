use super::mint::*;

const BASE: usize = (1 << 19) - 1;
pub fn rolling_hash(s: &Vec<char>, t: &Vec<char>) -> Vec<usize> {
    type Mint = ModInt<{ (1 << 31) - 1 }>;
    let mut res = vec![];

    let sl = s.len();
    let tl = t.len();
    if sl < tl {
        return vec![];
    }

    let mut p = Mint::new(1);
    for _ in 0..tl {
        p *= Mint::new(BASE);
    }

    let mut sh = Mint::new(0);
    let mut th = Mint::new(0);

    for i in 0..tl {
        sh = sh * Mint::new(BASE) + Mint::new(s[i] as usize);
        th = th * Mint::new(BASE) + Mint::new(t[i] as usize);
    }

    for i in 0..=sl - tl {
        if th.value() == sh.value() {
            res.push(i as usize);
        }
        if i + tl < sl {
            sh = sh * Mint::new(BASE) + Mint::new(s[i + tl] as usize)
                - Mint::new(s[i] as usize) * p.clone();
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s: Vec<char> = "aiuiuaoiuuuiu".chars().collect();
        let t: Vec<char> = "iu".chars().collect();

        // s: aiuiuaoiuuuiu
        // t: .iu.......... -> index: 1

        // s: aiuiuaoiuuuiu
        // t: ...iu........ -> index: 3

        // s: aiuiuaoiuuuiu
        // t: .......iu.... -> index: 7

        // s: aiuiuaoiuuuiu
        // t: ...........iu -> index: 11

        assert_eq!(rolling_hash(&s, &t), vec![1, 3, 7, 11]);
    }

    #[test]
    fn case2() {
        let s: Vec<char> = "oooooooooo".chars().collect();
        let t: Vec<char> = "oooo".chars().collect();

        assert_eq!(rolling_hash(&s, &t), vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn case3() {
        let s: Vec<char> = "oooooooooo".chars().collect();
        let t: Vec<char> = "aa".chars().collect();

        assert_eq!(rolling_hash(&s, &t), vec![]);
    }
}
