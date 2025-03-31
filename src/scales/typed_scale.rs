use std::ops::Add;
use crate::interval::Interval;
use crate::scales;
use crate::scales::{ScaleDefinition, ScaleMode};
use crate::scales::exact_scale::ExactScale;
use crate::scales::sized_scale::SizedScale;

// const type, variable mode
pub struct TypedScale<const N: usize, S: ScaleDefinition<N>> {
    mode: S::Mode
}

impl<const N: usize, S: ScaleDefinition<N>> TypedScale<N, S> {
    pub fn new(mode: S::Mode) -> Self {
        Self { mode }
    }
    
    // TODO: does it make sense to keep this method?
    pub fn make_exact<E: ExactScale<N, Scale=S>>() -> E {
        E::default()
    }
}

impl<const N: usize, S: ScaleDefinition<N>> SizedScale<N> for TypedScale<N, S> {
    fn relative_intervals(&self) -> [Interval; N] {
        let mut ivls = S::INTERVALS;
        
        ivls.rotate_left((self.mode.as_num() - 1) as _);
        
        ivls
    }

    fn build_from<T: Add<Interval, Output=T> + Clone>(&self, root: T) -> [T; N] {
        scales::build_from(self.relative_intervals(), root)
    }
}