use crate::interval::Interval;
use crate::scales::{ScaleDefinition, ScaleMode};
use crate::scales::exact_scale::ExactScale;
use crate::scales::sized_scale::SizedScale;

// const type, variable mode
#[derive(Debug)]
pub struct TypedScale<S: ScaleDefinition<N>, const N: usize> {
    mode: S::Mode
}

impl<const N: usize, S: ScaleDefinition<N>> TypedScale<S, N> {
    pub fn new(mode: S::Mode) -> Self {
        Self { mode }
    }
    
    // TODO: does it make sense to keep this method?
    pub fn make_exact<E: ExactScale<N, Scale=S>>() -> E {
        E::default()
    }
}

impl<S: ScaleDefinition<N>, const N: usize> SizedScale<N> for TypedScale<S, N> {
    fn relative_intervals(&self) -> [Interval; N] {
        let mut ivls = S::INTERVALS;
        
        ivls.rotate_left((self.mode.as_num() - 1) as _);
        
        ivls
    }
}