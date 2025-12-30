#![allow(dead_code, unused_imports,  reason = "most of these constants aren't going to be used")]

use crate::interval::Interval;
use crate::scales::definition::define_scale;
use super::{S, T, TS, TT, A2};

const A1: Interval = Interval::AUGMENTED_UNISON;
const D3: Interval = Interval::DIMINISHED_THIRD;

define_scale!(
    // multiple ways to spell this scale, this is chosen since no AA intervals
    name = Diminished,
    size = 8,
    intervals = [T, A1, D3, A1, T, S, T, S],
    mode = [I, II],
    exact_single = I,
    exact = [
        II => DominantDiminished,
    ]
);

define_scale!(
    name = Algerian,
    size = 8,
    intervals = [T, S, T, A1, S, S, A2, S],
    exact_single = I,
);

define_scale!(
    name = SpanishEightTone,
    size = 8,
    intervals = [S, T, A1, S, S, T, T, T],
    exact_single = I,
);