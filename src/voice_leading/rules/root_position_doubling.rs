use crate::harmony::Key;
use crate::voice_leading::roman_chord::{RomanChord, ScaleDegree};
use crate::voice_leading::Voicing;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct RootPositionDoubling;

impl RootPositionDoubling {
    pub fn evaluate(&self, voicing: Voicing, chord: RomanChord, key: Key) -> bool {
        if chord.inversion() != 0 || chord.has_seventh() {
            return true;
        }

        if RomanChord::mode_has_raised_leading_tone(key.mode) && chord.degree == ScaleDegree::VII {
            return true;
        }

        let root = chord.root_in_key(key).as_pitch_class();

        voicing.iter()
            .map(|n| n.pitch.as_pitch_class())
            .filter(|&p| p == root)
            .count() >= 2
    }
}
