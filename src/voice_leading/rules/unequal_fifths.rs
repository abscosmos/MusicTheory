use strum::IntoEnumIterator;
use crate::Interval;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct UnequalFifths;

impl UnequalFifths {
    pub fn evaluate(&self, first: Voicing, second: Voicing) -> Result<(), (Voice, Voice)> {
        for v1 in Voice::iter() {
            for v2 in Voice::iter() {
                if v2 <= v1 {
                    continue;
                }

                let v1_first = first[v1];
                let v2_first = first[v2];
                let v1_second = second[v1];
                let v2_second = second[v2];

                let first_interval = v1_first.distance_to(v2_first).as_simple();
                let second_interval = v1_second.distance_to(v2_second).as_simple();

                let is_perfect_to_dim = first_interval == Interval::PERFECT_FIFTH
                    && second_interval == Interval::DIMINISHED_FIFTH;

                let is_dim_to_perfect = first_interval == Interval::DIMINISHED_FIFTH
                    && second_interval == Interval::PERFECT_FIFTH;

                if is_perfect_to_dim || is_dim_to_perfect {
                    return Err((v1, v2));
                }
            }
        }

        Ok(())
    }
}
