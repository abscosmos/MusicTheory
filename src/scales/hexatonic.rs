#![allow(dead_code, unused_imports,  reason = "most of these constants aren't going to be used")]

use crate::interval::Interval;
use crate::scales::define::define_scale;
use super::{S, T, TS, TT, A2};

const A1: Interval = Interval::AUGMENTED_UNISON;
const D3: Interval = Interval::DIMINISHED_THIRD;

define_scale!(
    name = Hexatonic,
    size = 6,
    intervals = [T, T, S, T, T, TS],
    exact = [
        I => MajorHexatonic,
        II => MinorHexatonic,
        III => RitsuOnkai,
        IV => RagaKumud,
    ],
);

define_scale!(
    name = WholeTone,
    size = 6,
     // TODO: should this scale exist?
    intervals = [T, T, T, T, T, D3],
    exact_single = I,
);

define_scale!(
    name = Augmented,
    size = 6,
    intervals = [TS, A1, TS, A1, TS, S],
    exact_single = I,
);

define_scale!(
    name = Prometheus,
    size = 6,
    intervals = [T, T, T, TS, S, T],
    exact_single = I,
);

