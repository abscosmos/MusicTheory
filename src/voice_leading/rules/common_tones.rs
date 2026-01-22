use strum::IntoEnumIterator;
use crate::harmony::Key;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct CommonTones;

impl CommonTones {
    pub fn evaluate(&self, first: Voicing, second: Voicing, first_chord: RomanChord, second_chord: RomanChord, key: Key) -> u16 {
        let common_pcs = first_chord.pitch_class_set(key) & second_chord.pitch_class_set(key);

        if common_pcs.is_empty() {
            return 0;
        }

        let mut penalty = 0;

        for voice in Voice::iter() {
            let first_note = first[voice];
            let second_note = second[voice];

            let first_pc = first_note.pitch.as_pitch_class();

            if common_pcs.is_set(first_pc) && first_note != second_note {
                penalty += 1;
            }
        }

        penalty
    }
}
