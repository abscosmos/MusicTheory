use std::array;
use std::ops::Add;
use crate::interval::Interval;
use super::{S, T};

const A2: Interval = Interval::AUGMENTED_SECOND;

macro_rules! define_scale {
    (
        name = $name: ident,
        intervals = $intervals: expr
        $(, alias = $alias: ident)?
        $(, mode_aliases = [$($alias_mode: ident => $alias_mode_num: ident),* $(,)?])?
        $(,)?
    ) => {
        define_scale!(@define $name, $intervals);

        $(define_scale!(@scale_alias $name, $alias);)?

        $(define_scale!(@mode_aliases $name, $($alias_mode => $alias_mode_num),*);)?
    };
    
    (@define $name: ident, $intervals: expr) => {

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
    
    (@scale_alias $name: ident, $alias: ident) => {
        pub use $name as $alias;
    };
    
    (@mode_aliases $name: ident, $($alias_mode:ident => $alias_mode_num:ident),*) => {
        impl $name {
            $(
                pub const $alias_mode : Self = Self:: $alias_mode_num ;
            )*
        }
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

define_scale!(
    name = HeptatoniaPrimaMode,
    intervals = [T, T, S, T, T, T, S],
    alias = DiatonicMode,
    mode_aliases = [
        IONIAN => I,
        DORIAN => II,
        PHRYGIAN => III,
        LYDIAN => IV,
        MIXOLYDIAN => V,
        AEOLIAN => VI,
        LOCRIAN => VII,
        
        MAJOR => IONIAN,
        NATURAL_MINOR => AEOLIAN,
    ],
);

define_scale!(
    name = HeptatoniaSecundaMode,
    intervals = [T, S, T, T, T, T, S],
    alias = MelodicMinorMode, // TODO: MelodicAscendingMinorMode?
    mode_aliases = [
        MELODIC_MINOR => I,
        DORIAN_FLAT2 => II,
        LYDIAN_AUGMENTED => III,
        LYDIAN_DOMINANT => IV,
        MIXOLYDIAN_FLAT6 => V,
        HALF_DIMINISHED => VI,
        ALTERED => VII,
    ],
);

define_scale!(
    name = HeptatoniaTertiaMode,
    intervals = [S, T, T, T, T, T, S],
    alias = NeapolitanMajorMode,
);

define_scale!(
    name = NeapolitanMinorMode,
    intervals = [S, T, T, T, S, A2, S],
);

define_scale!(
    name = HarmonicMinorMode,
    intervals = [T, S, T, T, S, A2, S],
);

define_scale!(
    name = DoubleHarmonicMajorMode,
    intervals = [S, A2, S, T, S, A2, S],
);

define_scale!(
    name = DoubleHarmonicMinorMode,
    intervals = [T, S, A2, S, S, A2, S],
    alias = HungarianMinorMode,
);