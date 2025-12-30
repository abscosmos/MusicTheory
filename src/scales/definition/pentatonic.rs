#![allow(dead_code, unused_imports,  reason = "most of these constants aren't going to be used")]

use crate::scales::definition::define_scale;
use super::{S, T, TS, TT, A2};

define_scale!(
    name = Pentatonic,
    size = 5,
    intervals = [T, T, TS, T, TS],
    mode = [
        Major,
        Suspended,
        BluesMinor,
        BluesMajor,
        Minor,
    ],
    exact = [
        Major => MajorPentatonic,
        Suspended => SuspendedPentatonic, // TODO: alias egyptian scale
        BluesMinor => BluesMinorPentatonic,
        BluesMajor => BluesMajorPentatonic,
        Minor => MinorPentatonic,
        
        Major => Ryo, // I
        BluesMinor => Minyo, // III
        BluesMajor => RitsuPentatonic, // IV
        BluesMajor => JapaneseYo, // IV
    ],
);

define_scale!(
    name = Hirajoshi,
    size = 5,
    intervals = [T, S, TT, S, TT],
    // mode_aliases = [
    //     AEOLIAN => I, // kostka & payne and speed
    //     LOCRIAN => II, // sachs & slonimsky
    //     IONIAN => III,
    //     PHRYGIAN => IV,
    //     LYDIAN => V, // burrows
    // ],
    exact_single = I,
    exact = [
        I => Iwato,
        IV => MiyakoBushi,
        IV => JapaneseIn
    ],
);

define_scale!(
    name = Balinese,
    size = 5,
    intervals = [S, T, TT, S, TT],
    exact_single = I,
);