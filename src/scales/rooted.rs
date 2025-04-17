use std::ops::Add;
use crate::interval::Interval;
use crate::pitch::Pitch;
use crate::scales::dyn_scale::{DynScale, DynamicScale};
use crate::scales::numeral::Numeral;
use crate::scales::sized_scale::SizedScale;

pub struct RootedDynamicScale<R: Clone + Add<Interval, Output = R> + Into<Pitch>> {
    pub root: R,
    pub scale: DynamicScale,
}

#[derive(Debug)]
pub struct RootedSizedScale<R: Clone + Add<Interval, Output = R> + Into<Pitch>, const N: usize, S: SizedScale<N>> {
    pub root: R,
    pub scale: S,
}

impl<R: Clone + Add<Interval, Output = R> + Into<Pitch>> RootedDynamicScale<R> {
    pub fn derive_from_degree(degree: R, at: u8, scale: DynamicScale) -> Option<Self> {
        if !scale.valid_degree(at) {
            return None;
        }
        
        let root = root_from_degree_inner(scale.relative_intervals(), degree, at);

        Some(Self { root, scale })
    }
}

// TODO: is Into<Pitch> the best way to do this?
impl<R: Clone + Add<Interval, Output = R> + Into<Pitch>, const N: usize, S: SizedScale<N>> RootedSizedScale<R, N, S> {
    pub fn to_dyn(&self) -> RootedDynamicScale<R> {
        RootedDynamicScale {
            root: self.root.clone(),
            scale: self.scale.to_dyn(),
        }
    }
    
    pub fn derive_from_degree(degree: R, at: impl Numeral<N>, scale: S) -> Self {
        let root = root_from_degree_inner(&scale.relative_intervals(), degree, at.as_num());
        
        Self { root, scale }
    }
}

fn root_from_degree_inner<R: Clone + Add<Interval, Output = R>>(relative_intervals: &[Interval], degree: R, at: u8) -> R {
    let sum = relative_intervals
        .get(0..at as usize - 1)
        .expect("must be a valid range")
        .iter()
        .copied()
        .sum::<Interval>();

    degree + (-sum)
}