use std::ops::Add;
use crate::accidental::AccidentalSign;
use crate::interval::Interval;
use crate::pitch::Pitch;
use crate::scales::dyn_scale::{DynScale, DynamicScale};
use crate::scales::numeral::Numeral;
use crate::scales::sized_scale::SizedScale;

pub struct RootedDynamicScale<R: Clone + Add<Interval, Output = R> + Into<Pitch> + PartialOrd> {
    pub root: R,
    pub scale: DynamicScale,
}

#[derive(Debug)]
pub struct RootedSizedScale<R: Clone + Add<Interval, Output = R> + Into<Pitch>, const N: usize, S: SizedScale<N>> {
    pub root: R,
    pub scale: S,
}

impl<R: Clone + Add<Interval, Output = R> + Into<Pitch> + PartialOrd> RootedDynamicScale<R> {
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
    
    pub fn build_default(&self) -> Box<[R]> {
        self.scale.build_from(self.root.clone())
    }
}

// TODO: is Into<Pitch> the best way to do this?
impl<R: Clone + Add<Interval, Output = R> + Into<Pitch> + Ord, const N: usize, S: SizedScale<N>> RootedSizedScale<R, N, S> {
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
    
    pub fn build_default(&self) -> [R; N] {
        self.scale.build_from(self.root.clone())
    }
    
    // TODO: should this return Box<[T]> ?
    pub fn build(&self, min: R, max: R) -> Vec<R> {
        let mut gen = self.scale.relative_intervals().into_iter().cycle();
        
        let mut built = Vec::new(); // TODO: precalc capacity
        
        let mut curr = self.root.clone() + (-Interval::PERFECT_OCTAVE) + (-Interval::PERFECT_OCTAVE) + (-Interval::PERFECT_OCTAVE);
        
        while curr < min {
            curr = curr + gen.next().expect("must have next, since cycling"); 
        }
        
        while curr <= max { // TODO: does this cmp work for pitches?
            built.push(curr.clone());

            curr = curr + gen.next().expect("must have next, since cycling");
        }
        
        built
    }

    // TODO: better name to convey that passing a note that's in the scale will return the same note
    pub fn next_in_scale_after(&self, after: R) -> R {
        let scale = self.build_default();
        
        match scale.binary_search(&after) {
            Ok(idx) => scale[idx].clone(),
            Err(insert) => scale.get(insert)
                .cloned()
                .unwrap_or(
                    scale.first()
                        .expect("scale should have at least one element")
                        .clone()
                        + Interval::PERFECT_OCTAVE
                )
        }
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