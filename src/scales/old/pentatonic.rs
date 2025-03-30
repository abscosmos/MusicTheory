#![allow(dead_code, unused_imports, clippy::upper_case_acronyms,  reason = "most of these constants aren't going to be used")]

use crate::interval::Interval;
use super::{T, TS, define_scale, S, TT};

define_scale!(
    name = PentatonicModes,
    size = 5,
    intervals = [T, T, TS, T, TS],
    mode_aliases = [
        MAJOR => I,
        SUSPENDED => II,
        BLUES_MINOR => III,
        BLUES_MAJOR => IV,
        MINOR => V,
        
        RYO => I,
        MINYO => III,
        RITSU => IV, // TODO: this is for pentatonic, hexatonic has very different intervals
        JP_YO => IV,
    ]
);

define_scale!(
    name = HirajoshiMode,
    size = 5,
    intervals = [T, S, TT, S, TT],
    mode_aliases = [
        AEOLIAN => I, // kostka & payne and speed
        LOCRIAN => II, // sachs & slonimsky
        IONIAN => III,
        PHRYGIAN => IV,
        LYDIAN => V, // burrows
        
        IWATO => I,
        MIYAKO_BUSHI => IV,
        JP_IN => IV,
    ]
);

define_scale!(
    name = InsenMode,
    size = 5,
    intervals = [S, TT, T, TS, T],
);