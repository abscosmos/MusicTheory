use strum::IntoEnumIterator;
use crate::Interval;
use crate::Note;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ParallelInterval {
    interval: Interval,
}

impl ParallelInterval {
    pub fn new(interval: Interval) -> Option<Self> {
        interval.is_ascending().then_some(Self { interval })
    }

    pub fn evaluate(&self, first: Voicing, second: Voicing) -> Result<(), (Voice, Voice)> {
        fn check(v1: Note, v2: Note, interval: Interval) -> bool {
            v1.distance_to(v2).as_simple().abs().semitones() == interval.semitones()
        }

        for v1 in Voice::iter() {
            for v2 in Voice::iter() {
                if v2 <= v1 {
                    continue;
                }

                let v1_first = first[v1];
                let v2_first = first[v2];
                let v1_second = second[v1];
                let v2_second = second[v2];

                if v1_first != v1_second
                    && check(v1_first, v2_first, self.interval)
                    && check(v1_second, v2_second, self.interval)
                {
                    return Err((v1, v2));
                }
            }
        }

        Ok(())
    }
}
