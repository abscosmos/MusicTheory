use std::ops::Add;
use crate::interval::Interval;
use crate::scales;
use crate::scales::dyn_scale::DynamicScale;

pub trait SizedScale<const N: usize> {
    fn relative_intervals(&self) -> [Interval; N];

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; N] {
        scales::build_from(self.relative_intervals(), root)
    }
    
    fn to_dyn(&self) -> DynamicScale {
        DynamicScale::new(self.relative_intervals()).expect("must add up to a P8")
    }
}