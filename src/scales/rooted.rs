use std::ops::Add;
use crate::pitch::AccidentalSign;
use crate::interval::Interval;
use crate::pitch::Pitch;
use crate::scales::dyn_scale::{DynScale, DynamicScale};
use crate::scales::numeral::Numeral;
use crate::scales::sized_scale::SizedScale;

#[derive(Debug, Clone)]
pub struct RootedDynamicScale<R: Clone + Add<Interval, Output = R> + Into<Pitch> + PartialOrd> {
    pub root: R,
    pub scale: DynamicScale,
}

#[derive(Debug, Clone)]
pub struct RootedSizedScale<R: Clone + Add<Interval, Output = R> + Into<Pitch>, const N: usize, S: SizedScale<N> + Clone> {
    pub root: R,
    pub scale: S,
}

impl<R: Clone + Add<Interval, Output = R> + Into<Pitch> + Ord> RootedDynamicScale<R> {
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

    pub fn build(&self, min: R, max: R) -> Vec<R> {
        build_inner(self.root.clone(), self.scale.relative_intervals(), min, max)
    }

    pub fn next_in_scale_after(&self, after: R) -> R {
        next_in_scale_after_inner(&self.build_default(), after)
    }

    pub fn get(&self, degree: u8) -> Option<R> {
        self.scale
            .valid_degree(degree)
            .then(|| get_inner(self.scale.relative_intervals(), self.root.clone(), degree))
    }

    pub fn transpose(&self, interval: Interval) -> Self {
        Self {
            root: self.root.clone() + interval,
            scale: self.scale.clone(),
        }
    }
}

// TODO: is Into<Pitch> the best way to do this?
impl<R: Clone + Add<Interval, Output = R> + Into<Pitch> + Ord, const N: usize, S: SizedScale<N> + Clone> RootedSizedScale<R, N, S> {
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
        build_inner(self.root.clone(), &self.scale.relative_intervals(), min, max)
    }

    // TODO: better name to convey that passing a note that's in the scale will return the same note
    pub fn next_in_scale_after(&self, after: R) -> R {
        next_in_scale_after_inner(&self.build_default(), after)
    }
    
    pub fn get(&self, degree: impl Numeral<N>) -> R {
        get_inner(&self.scale.relative_intervals(), self.root.clone(), degree.as_num())
    }
    
    pub fn transpose(&self, interval: Interval) -> Self {
        Self {
            root: self.root.clone() + interval,
            scale: self.scale.clone(),
        }
    }
}

fn build_inner<R: Clone + Add<Interval, Output = R> + Ord>(root: R, rel_ivls: &[Interval], min: R, max: R) -> Vec<R> {
    let mut generator = rel_ivls.iter().cycle();

    let mut built = Vec::new();

    let mut curr = move_into_octave_before_target(root.clone(), min.clone());

    while curr < min {
        curr = curr + *generator.next().expect("must have next, since cycling");
    }

    while curr <= max { // TODO: does this cmp work for pitches?
        built.push(curr.clone());

        curr = curr + *generator.next().expect("must have next, since cycling");
    }

    built
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

fn next_in_scale_after_inner<R: Clone + Add<Interval, Output = R> + Ord>(default_scale: &[R], after: R) -> R {
    match default_scale.binary_search(&after) {
        Ok(idx) => default_scale[idx].clone(),
        Err(insert) => default_scale.get(insert)
            .cloned()
            .unwrap_or(
                default_scale.first()
                    .expect("scale should have at least one element")
                    .clone()
                    + Interval::PERFECT_OCTAVE
            )
    }
}

#[inline]
fn get_inner<R: Add<Interval, Output = R>>(rel_ivls: &[Interval], start: R, degree: u8) -> R {
    rel_ivls[..(degree - 1) as _]
        .iter()
        .copied()
        .fold(start, Add::add)
}

// this function is necessary since R's concrete type may have octave or not
fn has_octave_information<R: Add<Interval, Output = R> + Clone + Eq>(val: R) -> bool {
    // if val is the same after transposing up an octave, then it doesn't have octave information
    val.clone() + Interval::PERFECT_OCTAVE != val
}

// this function is necessary since can't access R's concrete type's octave information, if it even has it
fn move_into_octave_before_target<R: Add<Interval, Output = R> + Clone + Eq + Ord>(mut val: R, target: R) -> R {
    if has_octave_information(val.clone()) {
        let start = target.clone() + (-Interval::PERFECT_OCTAVE);

        while val < start {
            val = val + Interval::PERFECT_OCTAVE;
        }

        while val >= target {
            val = val + (-Interval::PERFECT_OCTAVE);
        }
    }

    val
}