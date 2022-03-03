use super::algebraical::Algebraical;

pub trait Identity: Algebraical {
    fn id() -> Self::S;
}
