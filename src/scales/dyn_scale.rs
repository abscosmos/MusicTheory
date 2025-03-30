use crate::interval::Interval;

// var ty, var mode
pub struct RefDynScale<'a> {
    mode: u8,
    ivls: &'a [Interval],
}

// var ty, var mode
pub struct OwnedDynScale {
    mode: u8,
    ivls: Box<[Interval]>
}