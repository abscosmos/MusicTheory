use std::ops::Add;
use crate::interval::Interval;
use crate::scales;

// var ty, var mode
pub struct DynamicScale {
    ivls: Box<[Interval]>,
}

impl DynamicScale {
    pub fn new(ivls: impl Into<Box<[Interval]>>) -> Option<Self> {
        let ivls = ivls.into();

        let sums_to_octave = ivls.iter().copied().reduce(Add::add) == Some(Interval::PERFECT_OCTAVE);

        sums_to_octave.then_some(Self { ivls })
    }
}

pub trait DynScale {
    fn size(&self) -> usize;
    
    fn relative_intervals(&self) -> &[Interval];

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> Box<[T]>;
}

impl DynScale for DynamicScale {
    fn size(&self) -> usize {
        self.ivls.len()
    }

    fn relative_intervals(&self) -> &[Interval] {
        &self.ivls
    }

    fn build_from<T: Add<Interval, Output=T> + Clone>(&self, root: T) -> Box<[T]> {
        scales::boxed_build_from(self.relative_intervals(), root)
    }
}