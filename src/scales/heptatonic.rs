use std::array;
use std::ops::Add;
use crate::interval::Interval;
use super::{S, T};

const A2: Interval = Interval::AUGMENTED_SECOND;

macro_rules! define_scale {
    ($name: ident, $intervals: expr) => {
        
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
        pub enum $name {
            I = 1,
            II,
            III,
            IV,
            V,
            VI,
            VII,
        }
        
        impl $crate::scales::heptatonic::HeptatonicScaleModes for $name {
            const RELATIVE_INTERVALS: [Interval; 7] = $intervals;
        
            fn number(&self) -> u8 {
                *self as _
            }
        
            fn from_number(number: u8) -> Option<Self> {
                Self::from_repr(number)
            }
        }
    };
    
    ($name: ident, $intervals: expr, alias = $alias: ident) => {
        define_scale!($name, $intervals);
        
        pub use $name as $alias;
    };
}

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

define_scale!(HeptatoniaPrimaMode, [T, T, S, T, T, T, S], alias = DiatonicMode);

impl HeptatoniaPrimaMode {
    pub const IONIAN: Self = Self::I;
    pub const DORIAN: Self = Self::II;
    pub const PHRYGIAN: Self = Self::III;
    pub const LYDIAN: Self = Self::IV;
    pub const MIXOLYDIAN: Self = Self::V;
    pub const AEOLIAN: Self = Self::VI;
    pub const LOCRIAN: Self = Self::VII;
    
    pub const MAJOR: Self = Self::IONIAN;
    pub const NATURAL_MINOR: Self = Self::AEOLIAN;
}

// TODO: MelodicAscendingMinorMode?
define_scale!(HeptatoniaSecundaMode, [T, S, T, T, T, T, S], alias = MelodicMinorMode);

impl HeptatoniaSecundaMode {
    pub const MELODIC_MINOR: Self = Self::I;
    pub const DORIAN_FLAT2: Self = Self::II;
    pub const LYDIAN_AUGMENTED: Self = Self::III;
    pub const LYDIAN_DOMINANT: Self = Self::IV;
    pub const MIXOLYDIAN_FLAT6: Self = Self::V;
    pub const HALF_DIMINISHED: Self = Self::VI;
    pub const ALTERED: Self = Self::VII;
}

define_scale!(HeptatoniaTertiaMode, [S, T, T, T, T, T, S], alias = NeapolitanMajorMode);

define_scale!(NeapolitanMinorMode, [S, T, T, T, S, A2, S]);

define_scale!(HarmonicMinorMode, [T, S, T, T, S, A2, S]);

define_scale!(DoubleHarmonicMode, [S, A2, S, T, S, A2, S]);