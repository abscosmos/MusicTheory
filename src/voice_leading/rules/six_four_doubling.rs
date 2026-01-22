use crate::harmony::Key;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::Voicing;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct SixFourDoubling;

impl SixFourDoubling {
    pub fn evaluate(&self, voicing: Voicing, chord: RomanChord, key: Key) -> bool {
        if chord.inversion() != 2 || chord.has_seventh() {
            return true;
        }

        let bass_pc = chord.bass(key).as_pitch_class();

        voicing.iter()
            .filter(|n| n.pitch.as_pitch_class() == bass_pc)
            .count() >= 2
    }
}
