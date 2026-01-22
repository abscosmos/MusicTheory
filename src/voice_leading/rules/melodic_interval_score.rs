use strum::IntoEnumIterator;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct MelodicIntervalScore;

impl MelodicIntervalScore {
    pub fn evaluate(&self, first: Voicing, second: Voicing) -> u16 {
        let mut penalty = 0;

        for voice in Voice::iter() {
            let first_note = first[voice];
            let second_note = second[voice];

            if first_note == second_note {
                continue;
            }

            let semis = first_note.distance_to(second_note)
                .as_simple()
                .semitones()
                .0
                .abs();

            penalty += match semis {
                0..=2 => 0,
                3..=4 => 1,
                5..=6 => 2,
                7 => 4,
                8..=12 => 8,
                _ => unreachable!("simple intervals have semitone count in [0,12)"),
            };
        }

        penalty
    }
}
