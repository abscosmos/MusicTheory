use std::ops::Add;
use crate::interval::Interval;
use crate::scales;

// var ty, var mode
pub struct DynScaleRef<'a> {
    mode: u8,
    ivls: &'a [Interval],
}

// var ty, var mode
pub struct DynScaleOwned {
    mode: u8,
    ivls: Box<[Interval]>
}

impl DynScaleOwned {
    pub fn as_scale_ref(&self) -> DynScaleRef {
        DynScaleRef { mode: self.mode, ivls: &self.ivls }
    }
}

pub trait DynScale {
    fn size(&self) -> usize;
    
    fn relative_intervals(&self) -> &[Interval];

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> Box<[T]>;
}

impl DynScale for DynScaleRef<'_> {
    fn size(&self) -> usize {
        self.ivls.len()
    }

    fn relative_intervals(&self) -> &[Interval] {
        self.ivls
    }

    fn build_from<T: Add<Interval, Output=T> + Clone>(&self, root: T) -> Box<[T]> {
        scales::boxed_build_from(self.relative_intervals(), root, self.mode)
    }
}

impl DynScale for DynScaleOwned {
    fn size(&self) -> usize {
        self.as_scale_ref().size()
    }

    fn relative_intervals(&self) -> &[Interval] {
        &self.ivls
    }

    fn build_from<T: Add<Interval, Output=T> + Clone>(&self, root: T) -> Box<[T]> {
        self.as_scale_ref().build_from(root)
    }
}