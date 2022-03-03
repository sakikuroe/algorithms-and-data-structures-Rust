use super::algebraical::Algebraical;

pub trait Invertibility: Algebraical {
    fn inverse(a: Self::S) -> Self::S;
}
