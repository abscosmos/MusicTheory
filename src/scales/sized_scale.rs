use std::ops::Add;
use crate::interval::Interval;

pub trait SizedScale<const N: usize> {
    fn relative_intervals(&self) -> [Interval; N];

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; N];
}