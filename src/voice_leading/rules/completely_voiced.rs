use crate::harmony::Key;
use crate::Pitch;
use crate::set::PitchClassSet;
use crate::voice_leading::roman_chord::{Quality, RomanChord};
use crate::voice_leading::Voicing;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct CompletelyVoiced;

impl CompletelyVoiced {
    pub fn evaluate(&self, voicing: Voicing, chord: RomanChord, key: Key) -> bool {
        let voicing_set = voicing.into_iter()
            .map(|p| p.pitch.as_pitch_class())
            .collect::<PitchClassSet>();

        let chord_pitches = chord.pitches(key);

        let full_chord = chord_pitches.iter()
            .copied()
            .map(Pitch::as_pitch_class)
            .collect::<PitchClassSet>();

        let eliminated_fifth = full_chord.with_cleared(chord_pitches[2].as_pitch_class());

        let can_eliminate_fifth = !chord.has_seventh()
            && !(chord.triad_quality == Quality::Augmented)
            && !(chord.triad_quality == Quality::Diminished);

        voicing_set == full_chord || (can_eliminate_fifth && voicing_set == eliminated_fifth)
    }
}
