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

    // duplicate rule?
    let can_eliminate_fifth = !chord.has_seventh() // TODO: not sure about these rules
        && !(chord.triad_quality == Quality::Augmented)
        && !(chord.triad_quality == Quality::Diminished);

    // sevenths must be fully voiced
    // also, eliminating the fifth is only valid in some cases
    // TODO: needs to check that the previous chord has a seventh
    voicing_set == full_chord || (can_eliminate_fifth && voicing_set == eliminated_fifth)
}

pub fn eliminated_fifths(first_chord: Option<RomanChord>, second_chord: RomanChord, second_voicing: Voicing, key: Key) -> bool {
    debug_assert!(
        completely_voiced(second_voicing, second_chord, key),
        "second chord must be completely voiced for eliminated fifth check",
    );

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

    // valid cases
    if second_chord.degree == ScaleDegree::I
        && second_chord.inversion() == 0
        /*&& second_voicing.iter().filter(|n| n.pitch == key.tonic).count() == 3*/
        && prev_is_root_v7
    {
        return true;
    }

    // not a valid case
    false
}