use crate::harmony::Key;
use crate::Pitch;
use crate::set::PitchClassSet;
use crate::voice_leading::roman_chord::{Quality, RomanChord, ScaleDegree};
use crate::voice_leading::{Voice, Voicing};

pub fn bass_note(v: Voicing, chord: RomanChord, key: Key) -> bool {
    v[Voice::Bass].pitch == chord.bass(key)
}

// this does not check spelling, which it probably should
pub fn completely_voiced(v: Voicing, chord: RomanChord, key: Key) -> bool {
    let voicing_set = v.into_iter()
        .map(|p| p.pitch.as_pitch_class())
        .collect::<PitchClassSet>();

    let chord_pitches = chord.pitches(key);

    let full_chord = chord_pitches.iter()
        .copied()
        .map(Pitch::as_pitch_class)
        .collect::<PitchClassSet>();

    let eliminated_fifth = full_chord.with_cleared(chord_pitches[2].as_pitch_class());

    let is_v7 = chord.degree == ScaleDegree::V && chord.has_seventh();

    // TODO: this does not allow eliminating from I, and assumes it's always fine to eliminate from V7
    voicing_set == full_chord || (is_v7 && voicing_set == eliminated_fifth)
}