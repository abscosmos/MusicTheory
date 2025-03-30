use std::array;
use std::ops::Add;
use crate::interval::Interval;
use super::{S, T, TS, TT, A2};

// const type, variable mode
pub struct ScaleCtVm<const N: usize, S: ScaleLike<N>> {
    mode: S::Mode
}

impl<const N: usize, S: ScaleLike<N>> ScaleCtVm<N, S> {
    pub fn new(mode: S::Mode) -> Self {
        Self { mode }
    }
    
    pub fn relative_intervals(&self) -> [Interval; N] {
        S::INTERVALS
    }
    
    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; N] {
        build_from(self.relative_intervals(), root, &self.mode)
    }
}

// const type, const mode
pub struct MajorScale;

impl MajorScale {
    pub fn base(&self) -> ScaleCtVm<7, DiatonicScaleDef> {
        ScaleCtVm::new(DiatonicMode::I)
    }
    
    pub fn relative_intervals(&self) -> [Interval; 7] {
        self.base().relative_intervals()
    }

    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; 7] {
        self.base().build_from(root)
    }
}

// const tysize, variable mode, ref; this is 16 bytes :(
pub struct RefScaleN<'a, const N: usize, M: BaseMode<N>> {
    mode: M,
    ivls: &'a [Interval; N],
}

// const tysize, variable mode, owned
pub struct OwnedScaleN<const N: usize, M: BaseMode<N>> {
    mode: M,
    ivls: [Interval; N],
}

// var ty, var mode
pub struct RefDynScale<'a> {
    mode: u8,
    ivls: &'a [Interval],
}

// var ty, var mode
pub struct OwnedDynScale {
    mode: u8,
    ivls: Box<[Interval]>
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, strum_macros::FromRepr)]
pub enum BaseMode7 {
    I = 1,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

impl BaseMode<7> for BaseMode7 {
    fn as_num(&self) -> u8 {
        *self as _
    }

    fn from_num(num: u8) -> Option<Self> where Self: Sized {
        Self::from_repr(num)
    }
}

pub trait BaseMode<const N: usize> {
    fn as_num(&self) -> u8;

    fn from_num(num: u8) -> Option<Self> where Self: Sized;
}


pub trait ScaleLike<const N: usize> {
    type Mode: ScaleMode<N>;
    const INTERVALS: [Interval; N];
}

pub struct DiatonicScaleDef;

impl ScaleLike<7> for DiatonicScaleDef {
    type Mode = DiatonicMode;
    const INTERVALS: [Interval; 7] = [T, T, S, T, T, T, S];
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, strum_macros::FromRepr)]
pub enum DiatonicMode {
    I = 1,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

// TODO: assoc const for size
impl ScaleMode<7> for DiatonicMode {
    fn as_num(&self) -> u8 {
        *self as _
    }

    fn from_num(num: u8) -> Option<Self> where Self: Sized {
        Self::from_repr(num)
    }
}

pub trait ScaleIntervals {
    
}

fn build_from<T: Add<Interval, Output = T> + Clone, const N: usize, M: ScaleMode<N>>(rel_ivls: [Interval; N], root: T, mode: &M) -> [T; N] {
    let mode = mode.as_num();
    
    assert!(mode >= 1 && mode as usize <= N, "TODO: make degree enum; mode should be in range");
    
    let mut curr = root;

    array::from_fn(|i| {
        let ret = curr.clone();

        curr = curr.clone() + rel_ivls[(i + mode as usize - 1) % N];

        ret
    })
}


pub trait ScaleMode<const N: usize> {
    fn as_num(&self) -> u8;
    
    fn from_num(num: u8) -> Option<Self> where Self: Sized;
}