use std::cmp::Ordering;
use strum::IntoEnumIterator;
use crate::interval::Number as IntervalNumber;
use crate::Interval;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct DirectFifthsOctaves;

impl DirectFifthsOctaves {
    pub fn evaluate(&self, first: Voicing, second: Voicing) -> Result<(), Voice> {
        for voice in Voice::iter().skip(1) {
            let soprano_first = first[Voice::Soprano];
            let soprano_second = second[Voice::Soprano];
            let other_first = first[voice];
            let other_second = second[voice];

            let s_motion = soprano_first.cmp(&soprano_second);

            if !(s_motion == other_first.cmp(&other_second) && s_motion != Ordering::Equal) {
                continue;
            }

            let second_interval = soprano_second.distance_to(other_second).as_simple();

            if !matches!(second_interval, Interval::PERFECT_FIFTH | Interval::PERFECT_OCTAVE) {
                continue;
            }

            let soprano_motion = soprano_first.distance_to(soprano_second).as_simple().abs();

            if soprano_motion.number() != IntervalNumber::SECOND {
                return Err(voice);
            }
        }

        Ok(())
    }
}
