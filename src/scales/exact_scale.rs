use std::ops::Add;
use crate::interval::Interval;
use crate::scales::{DiatonicMode, DiatonicScaleDef, ScaleLike};
use crate::scales::typed_scale::TypedScale;

// TODO: N should be an assoc constant once that's stable
pub trait ExactScale<const N: usize>: Default {
    type Scale: ScaleLike<N>;
    
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

impl MajorScale {
    pub fn relative_intervals() -> [Interval; 7] {
        DiatonicScaleDef::INTERVALS
    }

    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; 7] {
        self.as_typed().build_from(root)
    }
}