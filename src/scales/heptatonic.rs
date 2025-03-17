use crate::interval::Interval;
use super::{S, T, define_scale};

const A2: Interval = Interval::AUGMENTED_SECOND;

define_scale!(
    name = HeptatoniaPrimaMode,
    size = 7,
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
    size = 7,
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
    size = 7,
    intervals = [S, T, T, T, T, T, S],
    alias = NeapolitanMajorMode,
);

define_scale!(
    name = NeapolitanMinorMode,
    size = 7,
    intervals = [S, T, T, T, S, A2, S],
);

define_scale!(
    name = HarmonicMinorMode,
    size = 7,
    intervals = [T, S, T, T, S, A2, S],
    mode_aliases = [
        UKRANIAN_DORIAN => IV
    ]
);

define_scale!(
    name = DoubleHarmonicMajorMode,
    size = 7,
    intervals = [S, A2, S, T, S, A2, S],
);

define_scale!(
    name = DoubleHarmonicMinorMode,
    size = 7,
    intervals = [T, S, A2, S, S, A2, S],
    alias = HungarianMinorMode,
);

define_scale!(
    name = EnigmaticMode,
    size = 7,
    intervals = [S, A2, T, T, T, S, S],
);