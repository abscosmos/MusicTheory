use std::ops::Add;
use crate::interval::Interval;
use crate::scales;
use crate::scales::{ScaleDefinition, ScaleMode};
use crate::scales::typed_scale::TypedScale;

// var ty, var mode
// TODO: Box<dyn Metadata>?
#[derive(Debug)]
pub struct DynamicScale {
    ivls: Box<[Interval]>,
}

impl DynamicScale {
    pub fn new(ivls: impl Into<Box<[Interval]>>) -> Option<Self> {
        let ivls = ivls.into();

        let sums_to_octave = ivls.iter().copied().sum::<Interval>() == Interval::PERFECT_OCTAVE;

        sums_to_octave.then_some(Self { ivls })
    }
    
    pub fn try_into_typed<S: ScaleDefinition<N>, const N: usize>(&self) -> Option<TypedScale<S, N>> {
        let mode_num = find_rotation(&S::INTERVALS, self.relative_intervals())? + 1;

        let mode = S::Mode::from_num(mode_num as _)
            .expect("must be in range, since in [1, S::INTERVALS.len()]");
        
        Some(TypedScale::new(mode))
    }

    pub(crate) fn valid_degree(&self, degree: u8) -> bool {
        (1..=self.size()).contains(&(degree as _))
    } 
}

fn find_rotation<T: PartialEq>(a: &[T], b: &[T]) -> Option<usize> {
    if a.len() != b.len() {
        None
    } else if a.is_empty() {
        Some(0)
    } else {
        let n = a.len();
        
        (0..n).find(|start| (0..n).all(|i| a[(start + i) % n] == b[i]))
    }
}

// TODO: should DynScale be implemented for TypedScale & ExactScale or should this trait be deleted?
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