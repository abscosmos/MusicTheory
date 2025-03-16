use std::array;
use std::ops::Add;
use strum_macros::FromRepr;
use crate::interval::Interval;
use super::{S, T};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, FromRepr)]
pub enum HeptatoniaPrimaMode {
    Ionian = 1,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl HeptatoniaPrimaMode {
    pub const MAJOR: Self = Self::Ionian;
    pub const NATURAL_MINOR: Self = Self::Aeolian;

    const INTERVALS: [Interval; 7] = [T, T, S, T, T, T, S];

    // TODO: do we like this name?
    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; 7] {
        let mode = self.number() as usize;

        let mut curr = root;

        array::from_fn(|i| {
            let ret = curr.clone();

            curr = curr.clone() + Self::INTERVALS[(i + mode - 1) % Self::INTERVALS.len()];

            ret
        })
    }

    pub fn intervals(&self) -> [Interval; 7] {
        self.build_from(Interval::PERFECT_UNISON)
    }

    pub fn number(&self) -> u8 {
        *self as _
    }

    pub fn from_number(number: u8) -> Option<Self> {
        Self::from_repr(number)
    }
}

pub use HeptatoniaPrimaMode as DiatonicMode;

// TODO: should this be I -> VII with consts for the names?
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, FromRepr)]
pub enum HeptatoniaSecundaMode {
    MelodicMinor = 1,
    DorianFlat2,
    LydianAugmented,
    LydianDominant,
    MixolydianFlat6,
    HalfDiminished,
    Altered,
}

// TODO: abstract into trait?
impl HeptatoniaSecundaMode {
    const INTERVALS: [Interval; 7] = [T, S, T, T, T, T, S];

    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; 7] {
        let mode = self.number() as usize;

        let mut curr = root;

        array::from_fn(|i| {
            let ret = curr.clone();

            curr = curr.clone() + Self::INTERVALS[(i + mode - 1) % Self::INTERVALS.len()];

            ret
        })
    }

    pub fn intervals(&self) -> [Interval; 7] {
        self.build_from(Interval::PERFECT_UNISON)
    }

    pub fn number(&self) -> u8 {
        *self as _
    }

    pub fn from_number(number: u8) -> Option<Self> {
        Self::from_repr(number)
    }
}

// TODO: MelodicAscendingMinorMode?
pub use HeptatoniaSecundaMode as MelodicMinorMode;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, FromRepr)]
pub enum HeptatoniaTertiaMode {
    I = 1,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

impl HeptatoniaTertiaMode {
    const INTERVALS: [Interval; 7] = [S, T, T, T, T, T, S];

    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; 7] {
        let mode = self.number() as usize;

        let mut curr = root;

        array::from_fn(|i| {
            let ret = curr.clone();

            curr = curr.clone() + Self::INTERVALS[(i + mode - 1) % Self::INTERVALS.len()];

            ret
        })
    }

    pub fn intervals(&self) -> [Interval; 7] {
        self.build_from(Interval::PERFECT_UNISON)
    }

    pub fn number(&self) -> u8 {
        *self as _
    }

    pub fn from_number(number: u8) -> Option<Self> {
        Self::from_repr(number)
    }
}

pub use HeptatoniaTertiaMode as NeapolitanMajorMode;