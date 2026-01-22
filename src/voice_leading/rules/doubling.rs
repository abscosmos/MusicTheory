use crate::harmony::Key;
use crate::Interval;
use crate::voice_leading::roman_chord::{RomanChord, ScaleDegree};
use crate::voice_leading::rules::voicing::{bass_note, completely_voiced};
use crate::voice_leading::Voicing;

// TODO: not sure if this method is right
pub fn root_position_doubling(voicing: Voicing, chord: RomanChord, key: Key) -> bool {
    // sanity check it's fully voiced
    debug_assert!(
        completely_voiced(voicing, chord, key),
        "chord must be completely voiced for doubling check",
    );

    if chord.inversion() != 0 || chord.has_seventh() {
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

    if chord.inversion() != 2 || chord.has_seventh() {
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

    let voicing_pitch_classes = v.iter()
        .map(|n| n.pitch)
        .collect::<Vec<_>>();

    let chord_pitches = chord.pitches(key);

    let leading_tone = {
        let mut vii = key.scale_experimental().build_default()[6];

        if RomanChord::mode_has_raised_leading_tone(key.mode) {
            vii = vii.transpose(Interval::AUGMENTED_UNISON);
        }

        vii
    };

    let chordal_seventh = chord.has_seventh().then(|| chord_pitches[3]);

    for pc in voicing_pitch_classes.iter() {
        let count = voicing_pitch_classes.iter().filter(|&p| p == pc).count();

        if count > 1 {
            if *pc == leading_tone {
                return false;
            }

            let chordal_seventh_not_doubled = chordal_seventh.is_none_or(|seventh| *pc != seventh);

            debug_assert!(chordal_seventh_not_doubled, "chordal seventh doubling should've already been caught");

            if !chordal_seventh_not_doubled {
                return false;
            }
        }
    }

    true
}