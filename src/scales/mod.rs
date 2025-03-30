use std::array;
use std::ops::Add;
use crate::interval::Interval;

mod old;
pub mod typed_scale;
pub mod exact_scale;
pub mod n_scale;
pub mod dyn_scale;

const T: Interval = Interval::MAJOR_SECOND;
const S: Interval = Interval::MINOR_SECOND;
const TS: Interval = Interval::MINOR_THIRD;
const TT: Interval = Interval::MAJOR_THIRD;
const A2: Interval = Interval::AUGMENTED_SECOND;


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
