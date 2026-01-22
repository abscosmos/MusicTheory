use crate::harmony::Key;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct BassNote;

impl BassNote {
    pub fn evaluate(&self, v: Voicing, chord: RomanChord, key: Key) -> bool {
        v[Voice::Bass].pitch == chord.bass(key)
    }
}