use crate::harmony::Key;
use crate::voice_leading::roman_chord::{inversions, RomanChord, ScaleDegree};
use crate::voice_leading::rules::voicing::{bass_note, completely_voiced};
use crate::voice_leading::{leading_tone, Voicing};

// TODO: not sure if this method is right
pub fn root_position_doubling(voicing: Voicing, chord: RomanChord, key: Key) -> bool {
    // sanity check it's fully voiced
    debug_assert!(
        completely_voiced(voicing, chord, key),
        "chord must be completely voiced for doubling check",
    );

    if chord.inversion() != inversions::INV_ROOT || chord.has_seventh() {
        return true;
    }

    let root = chord.root_in_key(key).as_pitch_class();

    // don't double the leading tone!
    if RomanChord::mode_has_raised_leading_tone(key.mode) && chord.degree == ScaleDegree::VII {
        return true;
    }

    voicing.iter()
        .map(|n| n.pitch.as_pitch_class())
        .filter(|&p| p == root)
        .count() >= 2
}

pub fn six_four_doubling(v: Voicing, chord: RomanChord, key: Key) -> bool {
    // sanity check the chord is voiced correctly
    debug_assert!(
        completely_voiced(v, chord, key),
        "chord must be completely voiced for 6/4 doubling check",
    );

    // also, ensure the bass is correct
    debug_assert!(
        bass_note(v, chord, key),
        "bass note must be correct for 6/4 doubling check",
    );

    if chord.inversion() != inversions::INV_64 || chord.has_seventh() {
        return true;
    }

    let bass_pc = chord.bass(key).as_pitch_class();

    let count = v.iter()
        .filter(|n| n.pitch.as_pitch_class() == bass_pc)
        .count();

    count >= 2
}

pub fn leading_tone_not_doubled(v: Voicing, chord: RomanChord, key: Key) -> bool {
    // sanity check it's fully voiced
    debug_assert!(
        completely_voiced(v, chord, key),
        "chord must be completely voiced for doubling check",
    );

    let leading_tone = leading_tone(key);

    v.iter().filter(|p| p.pitch == leading_tone).count() <= 1
}