use super::{algebraical::Algebraical, identity::Identity, magma::Magma, semigroup::SemiGroup};

pub trait Monoid: SemiGroup + Identity {}

pub struct MinMonoid;

impl Algebraical for MinMonoid {
    type S = usize;
}

impl Magma for MinMonoid {
    fn op(a: Self::S, b: Self::S) -> Self::S {
        std::cmp::min(a, b)
    }
}

impl Identity for MinMonoid {
    fn id() -> Self::S {
        std::usize::MAX
    }
}

impl SemiGroup for MinMonoid {}

impl Monoid for MinMonoid {}

pub struct MaxMonoid;

impl Algebraical for MaxMonoid {
    type S = usize;
}

impl Magma for MaxMonoid {
    fn op(a: Self::S, b: Self::S) -> Self::S {
        std::cmp::max(a, b)
    }
}

impl Identity for MaxMonoid {
    fn id() -> Self::S {
        0
    }
}

impl SemiGroup for MaxMonoid {}

impl Monoid for MaxMonoid {}

pub struct AddMonoid;

impl Algebraical for AddMonoid {
    type S = usize;
}

impl Magma for AddMonoid {
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a + b
    }
}

impl Identity for AddMonoid {
    fn id() -> Self::S {
        0
    }
}

impl SemiGroup for AddMonoid {}

impl Monoid for AddMonoid {}

struct RightMonoid;

impl Algebraical for RightMonoid {
    type S = usize;
}

impl Magma for RightMonoid {
    fn op(a: Self::S, b: Self::S) -> Self::S {
        if b == Self::id() {
            a
        } else {
            b
        }
    }
}

impl Identity for RightMonoid {
    fn id() -> Self::S {
        std::usize::MAX
    }
}

impl SemiGroup for RightMonoid {}

impl Monoid for RightMonoid {}

#[derive(Clone, Copy, Debug)]
pub struct Sum {
    value: usize,
    len: usize,
}

pub struct SumMonoid {}

impl Algebraical for SumMonoid {
    type S = Sum;
}

impl Magma for SumMonoid {
    fn op(a: Self::S, b: Self::S) -> Self::S {
        Sum {
            value: a.value + b.value,
            len: a.len + b.len,
        }
    }
}

impl Identity for SumMonoid {
    fn id() -> Self::S {
        Sum { value: 0, len: 0 }
    }
}

impl SemiGroup for SumMonoid {}

impl Monoid for SumMonoid {}
