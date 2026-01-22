use crate::harmony::Key;
use crate::set::PitchClassSet;
use crate::voice_leading::roman_chord::{RomanChord, ScaleDegree};
use crate::voice_leading::Voicing;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct EliminatedFifths;

impl EliminatedFifths {
    pub fn evaluate(&self, first_chord: Option<RomanChord>, second_chord: RomanChord, second_voicing: Voicing, key: Key) -> bool {
        let voicing_set = second_voicing.into_iter()
            .map(|p| p.pitch.as_pitch_class())
            .collect::<PitchClassSet>();

        if voicing_set.len() == second_chord.len() {
            return true;
        }

        let prev_is_root_v7 = if let Some(first_chord) = first_chord {
            first_chord.degree == ScaleDegree::V
                && first_chord.has_seventh()
                && first_chord.inversion() == 0
        } else {
            false
        };

        if second_chord.degree == ScaleDegree::I
            && second_chord.inversion() == 0
            && prev_is_root_v7
        {
            return true;
        }

        false
    }
}
