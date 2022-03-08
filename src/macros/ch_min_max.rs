#[macro_export]
macro_rules! min {
    ($a: expr) => {
        $a
    };
    ($a: expr, $b: expr) => {
        std::cmp::min($a, $b)
    };
    ($a: expr, $($b: expr),+) => {
        std::cmp::min($a, min!($($b),+))
    };
}

#[macro_export]
macro_rules! chmin {
    ($a: expr, $b: expr) => {{
        $a > $b && {
            $a = $b;
            true
        }
    }};
    ($a: expr, $($xs: expr),+) => {{
        chmin!($a, crate::min!($($xs),+))
    }};
}

#[macro_export]
macro_rules! max {
    ($a: expr) => {
        $a
    };
    ($a: expr, $($xs: expr),+) => {
        std::cmp::max($a, max!($($xs),+))
    };
}

#[macro_export]
macro_rules! chmax {
    ($a: expr, $b: expr) => {{
        $a < $b && {
            $a = $b;
            true
        }
    }};
    ($a: expr, $($xs: expr),+) => {{
        chmax!($a, crate::max!($($xs),+))
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn min_test() {
        assert_eq!(-10, min!(-10, 0));
        assert_eq!(-10, min!(0, -10));
        assert_eq!(0, min!(2, 6, 4, 1, 3, 0, 5));

        let mut temp = 10;
        assert!(chmin!(temp, 0, 3));
        assert_eq!(temp, 0);

        let mut temp = -5;
        assert!(!chmin!(temp, 0, 3));
        assert_eq!(temp, -5);

        assert_eq!(0, max!(-10, 0));
        assert_eq!(0, max!(0, -10));
        assert_eq!(6, max!(2, 6, 4, 1, 3, 0, 5));

        let mut temp = 10;
        assert!(!chmax!(temp, 0, 3));
        assert_eq!(temp, 10);

        let mut temp = -5;
        assert!(chmax!(temp, 0, 3));
        assert_eq!(temp, 3);
    }
}
