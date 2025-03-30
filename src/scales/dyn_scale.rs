use std::ops::Add;
use crate::interval::Interval;
use crate::scales;

// var ty, var mode
pub struct DynamicScale {
    mode: u8,
    ivls: Box<[Interval]>
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
        scales::boxed_build_from(self.relative_intervals(), root, self.mode)
    }
}