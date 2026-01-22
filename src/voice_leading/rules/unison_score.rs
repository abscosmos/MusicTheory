use strum::IntoEnumIterator;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct UnisonScore;

impl UnisonScore {
    pub fn evaluate(&self, voicing: Voicing) -> u16 {
        let mut penalty = 0;

        for v1 in Voice::iter() {
            for v2 in Voice::iter() {
                if v2 > v1 && voicing[v1] == voicing[v2] {
                    penalty += 1;
                }
            }
        }

        penalty
    }
}
