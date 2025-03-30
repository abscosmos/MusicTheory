use std::ops::Add;
use crate::interval::Interval;
use crate::scales::{DiatonicMode, DiatonicScaleDef, ScaleLike};
use crate::scales::typed_scale::TypedScale;

pub trait ExactScale<const N: usize, S: ScaleLike<N>>: Default {
    fn as_typed(&self) -> TypedScale<N, S>;
}

// const type, const mode
#[derive(Default)]
pub struct MajorScale;

impl ExactScale<7, DiatonicScaleDef> for MajorScale {
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