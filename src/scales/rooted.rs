use std::ops::Add;
use crate::accidental::AccidentalSign;
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

    pub fn get_scale_degree(&self, target: R) -> Option<u8> {
        let (degree, acc) = self.get_scale_degree_and_accidental(target)?;

        (acc == AccidentalSign::NATURAL).then_some(degree)
    }

    pub fn get_scale_degree_and_accidental(&self, target: R) -> Option<(u8, AccidentalSign)> { // TODO: support calling these fucntions from key
        let scale = self.scale.build_from(self.root.clone().into());

        get_scale_degree_and_accidental_inner(target.into(), &scale)
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
    
    pub fn get_scale_degree(&self, target: R) -> Option<u8> { // TODO: ideally this would give back a numeral type
        let (degree, acc) = self.get_scale_degree_and_accidental(target)?;

        (acc == AccidentalSign::NATURAL).then_some(degree)
    }

    pub fn get_scale_degree_and_accidental(&self, target: R) -> Option<(u8, AccidentalSign)> {
        let scale = self.scale.build_from(self.root.clone().into());
        
        get_scale_degree_and_accidental_inner(target.into(), &scale)
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

fn get_scale_degree_and_accidental_inner(target: Pitch, scale: &[Pitch]) -> Option<(u8, AccidentalSign)> {
    let (degree_0, found) = scale
        .iter()
        .enumerate()
        .find(|(_, p)| p.letter() == target.letter())?;

    let acc = target.accidental().offset - found.accidental().offset;

    Some((degree_0 as u8 + 1, AccidentalSign { offset: acc }))
}