use std::ops::Add;
use crate::interval::Interval;
use crate::scales::{DiatonicMode, DiatonicScaleDef, ScaleLike};
use crate::scales::typed_scale::TypedScale;

// const type, const mode
pub struct MajorScale;

impl MajorScale {
    pub fn base(&self) -> TypedScale<7, DiatonicScaleDef> {
        TypedScale::new(DiatonicMode::I)
    }

    pub fn relative_intervals(&self) -> [Interval; 7] {
        self.base().relative_intervals()
    }

    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; 7] {
        self.base().build_from(root)
    }
}