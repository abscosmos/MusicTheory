use std::ops::Add;
use crate::interval::Interval;
use crate::scales::{DiatonicMode, DiatonicScaleDef, ScaleDefinition};
use crate::scales::sized_scale::SizedScale;
use crate::scales::typed_scale::TypedScale;

// TODO: N should be an assoc constant once that's stable
pub trait ExactScale<const N: usize>: Default {
    type Scale: ScaleDefinition<N>;
    
    fn as_typed(&self) -> TypedScale<N, Self::Scale>;
}

// const type, const mode
#[derive(Default)]
pub struct MajorScale;

impl ExactScale<7> for MajorScale {
    type Scale = DiatonicScaleDef;

    fn as_typed(&self) -> TypedScale<7, DiatonicScaleDef> {
        TypedScale::new(DiatonicMode::I)
    }
}

impl<const N: usize, S: ScaleDefinition<N>, E: ExactScale<N, Scale= S>> SizedScale<N> for E {
    fn relative_intervals(&self) -> [Interval; N] {
        self.as_typed().relative_intervals()
    }

    fn build_from<T: Add<Interval, Output=T> + Clone>(&self, root: T) -> [T; N] {
        self.as_typed().build_from(root)
    }
}