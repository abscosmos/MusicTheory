use crate::interval::Interval;
use crate::scales::BaseMode;

// const tysize, variable mode, ref; this is 16 bytes :(
pub struct SizedScaleRef<'a, const N: usize, M: BaseMode<N>> {
    mode: M,
    ivls: &'a [Interval; N],
}

// const tysize, variable mode, owned
pub struct SizedScaleOwned<const N: usize, M: BaseMode<N>> {
    mode: M,
    ivls: [Interval; N],
}

impl<const N: usize, M: BaseMode<N>> SizedScaleOwned<N, M> {
    pub fn as_ref_sized_scale(&self) -> SizedScaleRef<N, M> {
        SizedScaleRef {
            mode: self.mode,
            ivls: &self.ivls,
        }
    }
}