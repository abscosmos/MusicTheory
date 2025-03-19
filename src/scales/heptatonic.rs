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

define_scale!(
    name = HungarianMajorMode,
    size = 7,
    intervals = [A2, S, T, S, T, S, T],
    mode_aliases = [
        MAJOR => I,
        ALTERED_DIMINISHED_DOUBLE_FLAT6 => II,
        HARMONIC_MINOR_FLAT5 => III,
        LOCRIAN_NATURAL2 => III,
        LOCRIAN_NATURAL7 => III,
        ALTERED_DOMINANT_NATURAL6 => IV,
        MELODIC_MINOR_SHARP5 => V,
        UKRAINIAN_DORIAN_FLAT2 => VI,
        LYDIAN_AUGMENTED_SHARP3 => VII,
    ],
);

define_scale!(
    name = RomanianMajorScale,
    size = 7,
    intervals = [S, A2, T, S, T, S, T],
    mode_aliases = [
        MAJOR => I,
        SUPER_LYDIAN_AUGMENTED_NATURAL6 => II,
        LOCRIAN_NATURAL2_DOUBLE_FLAT7 => III,
        BLUES_PHRYGIAN_FLAT4 => IV,
        JAZZ_MINOR_FLAT5 => V,
        SUPERPHRYGIAN_NATURAL6 => VI,
        LYDIAN_AUGMENTED_FLAT3 => VII,
    ],
);

define_scale!(
    name = RomanianMinorScale,
    size = 7,
    intervals = [T, S, A2, S, T, S, T],
);