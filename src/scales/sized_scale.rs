use std::ops::Add;
use crate::interval::Interval;
use crate::scales::{build_from, BaseMode};

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

pub trait SizedScale<const N: usize> {
    fn relative_intervals(&self) -> [Interval; N];

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; N];
}

impl<const N: usize, M: BaseMode<N>> SizedScale<N> for SizedScaleRef<'_, N, M> {
    fn relative_intervals(&self) -> [Interval; N] {
        *self.ivls
    }

    fn build_from<T: Add<Interval, Output=T> + Clone>(&self, root: T) -> [T; N] {
        build_from(self.relative_intervals(), root, self.mode.as_num())
    }
}

impl<const N: usize, M: BaseMode<N>> SizedScale<N> for SizedScaleOwned<N, M> {
    fn relative_intervals(&self) -> [Interval; N] {
        self.as_ref_sized_scale().relative_intervals()
    }

    fn build_from<T: Add<Interval, Output=T> + Clone>(&self, root: T) -> [T; N] {
        self.as_ref_sized_scale().build_from(root)
    }
}