use crate::interval::Interval;
use crate::scales::BaseMode;

// const tysize, variable mode, ref; this is 16 bytes :(
pub struct RefScaleN<'a, const N: usize, M: BaseMode<N>> {
    mode: M,
    ivls: &'a [Interval; N],
}

// const tysize, variable mode, owned
pub struct OwnedScaleN<const N: usize, M: BaseMode<N>> {
    mode: M,
    ivls: [Interval; N],
}