use super::{invertibility::Invertibility, monoid::Monoid};

pub trait Group: Monoid + Invertibility {}
