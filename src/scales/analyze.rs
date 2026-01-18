use std::array;
use crate::interval::Interval;
use crate::pitch::Pitch;

pub fn scale_relative_intervals<const N: usize>(pitches: [Pitch; N]) -> [Interval; N] {
    array::from_fn(|i| {
        pitches[i].distance_to(pitches[(i + 1) % pitches.len()])
    })
}