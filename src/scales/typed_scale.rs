use std::ops::Add;
use crate::interval::Interval;
use crate::scales::{build_from, ScaleLike};
use crate::scales::exact_scale::ExactScale;

// const type, variable mode
pub struct TypedScale<const N: usize, S: ScaleLike<N>> {
    mode: S::Mode
}

impl<const N: usize, S: ScaleLike<N>> TypedScale<N, S> {
    pub fn new(mode: S::Mode) -> Self {
        Self { mode }
    }

    pub fn relative_intervals(&self) -> [Interval; N] {
        S::INTERVALS
    }

    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; N] {
        build_from(self.relative_intervals(), root, &self.mode)
    }
    
    // TODO: does it make sense to keep this method?
    pub fn make_exact<E: ExactScale<N, S>>() -> E {
        E::default()
    }
}