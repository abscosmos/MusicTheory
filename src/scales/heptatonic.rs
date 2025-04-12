#![allow(dead_code, unused_imports,  reason = "most of these constants aren't going to be used")]

use crate::scales::define::define_scale;
use super::{S, T, TS, TT, A2};

define_scale! {
    name = Diatonic,
    size = 7,
    intervals = [T, T, S, T, T, T, S],
    mode = [
        Ionian,
        Dorian,
        Phrygian,
        Lydian,
        Mixolydian,
        Aeolian,
        Locrian,
    ],
    mode_aliases = [
        MAJOR => Ionian,
        NATURAL_MINOR => Aeolian,
    ],
    typed = DiatonicScale,
    exact = [
        Ionian => Major,
        Dorian => Dorian,
        Phrygian => Phrygian,
        Lydian => Lydian,
        Mixolydian => Mixolydian,
        Aeolian => NaturalMinor,
        Locrian => Locrian,
    ]
    // alias = HeptatoniaPrimaMode,
}

define_scale! {
    name = MelodicMinor,
    size = 7,
    intervals = [T, S, T, T, T, T, S],
    // mode_aliases = [
    //     MELODIC_MINOR => I,
    //     DORIAN_FLAT2 => II,
    //     LYDIAN_AUGMENTED => III,
    //     LYDIAN_DOMINANT => IV,
    //     ACOUSTIC => IV,
    //     AEOLIAN_DOMINANT => V,
    //     HALF_DIMINISHED => VI,
    //     ALTERED => VII,
    // ],
    // typed = AbstractMelodicMinor,
    exact_single = I,
    exact = [
        IV => Acoustic,
        VI => HalfDiminished,
        VII => Altered,
    ]
    // alias = HeptatoniaSecundaMode,
}

define_scale!(
    name = NeapolitanMajor,
    size = 7,
    intervals = [S, T, T, T, T, T, S],
    exact_single = I,
    // alias = HeptatoniaTertiaMode,
);

define_scale!(
    name = NeapolitanMinor,
    size = 7,
    intervals = [S, T, T, T, S, A2, S],
    exact_single = I,
);

define_scale!(
    name = HarmonicMinor,
    size = 7,
    intervals = [T, S, T, T, S, A2, S],
    exact_single = I,
    exact = [
        IV => UkrainianDorian,
    ]
);

define_scale!(
    name = DoubleHarmonicMajor,
    size = 7,
    intervals = [S, A2, S, T, S, A2, S],
    exact_single = I,
    // alias = HungarianMinorMode,
);

define_scale!(
    name = Enigmatic,
    size = 7,
    intervals = [S, A2, T, T, T, S, S],
    exact_single = I,
);

define_scale!(
    name = HungarianMajor,
    size = 7,
    intervals = [A2, S, T, S, T, S, T],
    // mode_aliases = [
    //     MAJOR => I,
    //     ALTERED_DIMINISHED_DOUBLE_FLAT6 => II,
    //     HARMONIC_MINOR_FLAT5 => III,
    //     LOCRIAN_NATURAL2 => III,
    //     LOCRIAN_NATURAL7 => III,
    //     ALTERED_DOMINANT_NATURAL6 => IV,
    //     MELODIC_MINOR_SHARP5 => V,
    //     UKRAINIAN_DORIAN_FLAT2 => VI,
    //     LYDIAN_AUGMENTED_SHARP3 => VII,
    // ],
    exact_single = I,
);

define_scale!(
    name = RomanianMajor,
    size = 7,
    intervals = [S, A2, T, S, T, S, T],
    // mode_aliases = [
    //     MAJOR => I,
    //     SUPER_LYDIAN_AUGMENTED_NATURAL6 => II,
    //     LOCRIAN_NATURAL2_DOUBLE_FLAT7 => III,
    //     BLUES_PHRYGIAN_FLAT4 => IV,
    //     JAZZ_MINOR_FLAT5 => V,
    //     SUPERPHRYGIAN_NATURAL6 => VI,
    //     LYDIAN_AUGMENTED_FLAT3 => VII,
    // ],
    exact_single = I,
);

define_scale!(
    name = RomanianMinor,
    size = 7,
    intervals = [T, S, A2, S, T, S, T],
    exact_single = I,
);