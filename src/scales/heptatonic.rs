use std::array;
use std::ops::Add;
use strum_macros::FromRepr;
use crate::interval::Interval;
use super::{S, T};

// TODO: consider assoc constants for names of modes?

pub trait HeptatonicScaleModes: Sized {
    const RELATIVE_INTERVALS: [Interval; 7];

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; 7] {
        let mode = self.number() as usize;

        let mut curr = root;

        array::from_fn(|i| {
            let ret = curr.clone();

            curr = curr.clone() + Self::RELATIVE_INTERVALS[(i + mode - 1) % Self::RELATIVE_INTERVALS.len()];

            ret
        })
    }

    fn intervals(&self) -> [Interval; 7] {
        self.build_from(Interval::PERFECT_UNISON)
    }

    fn number(&self) -> u8;

    fn from_number(number: u8) -> Option<Self>;
}

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
}

impl HeptatonicScaleModes for HeptatoniaPrimaMode {
    const RELATIVE_INTERVALS: [Interval; 7] = [T, T, S, T, T, T, S];

    fn number(&self) -> u8 {
        *self as _
    }

    fn from_number(number: u8) -> Option<Self> {
        Self::from_repr(number)
    }
}

pub use HeptatoniaPrimaMode as DiatonicMode;

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

impl HeptatonicScaleModes for HeptatoniaSecundaMode {
    const RELATIVE_INTERVALS: [Interval; 7] = [T, S, T, T, T, T, S];

    fn number(&self) -> u8 {
        *self as _
    }

    fn from_number(number: u8) -> Option<Self> {
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

impl HeptatonicScaleModes for HeptatoniaTertiaMode {
    const RELATIVE_INTERVALS: [Interval; 7] = [S, T, T, T, T, T, S];

    fn number(&self) -> u8 {
        *self as _
    }

    fn from_number(number: u8) -> Option<Self> {
        Self::from_repr(number)
    }
}

pub use HeptatoniaTertiaMode as NeapolitanMajorMode;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, FromRepr)]
pub enum HarmonicMinorMode {
    I = 1,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

impl HeptatonicScaleModes for HarmonicMinorMode {
    const RELATIVE_INTERVALS: [Interval; 7] = [T, S, T, T, S, Interval::AUGMENTED_SECOND, S];

    fn number(&self) -> u8 {
        *self as _
    }

    fn from_number(number: u8) -> Option<Self> {
        Self::from_repr(number)
    }
}