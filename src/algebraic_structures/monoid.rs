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
