use super::algebraical::Algebraical;

pub trait Magma: Algebraical {
    // binary operation
    fn op(a: Self::S, b: Self::S) -> Self::S;
}
