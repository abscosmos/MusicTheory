use strum::IntoEnumIterator;
use crate::harmony::Key;
use crate::interval::Number;
use crate::voice_leading::rules::voicing::completely_voiced;
use crate::voice_leading::motion::{get_motion_between, VoiceMotion};
use crate::voice_leading::{Voice, Voicing};
use crate::voice_leading::roman_chord::RomanChord;

pub fn outer_voice_motion(first: Voicing, second: Voicing) -> u16 {
    match get_motion_between(Voice::Soprano, Voice::Bass, first, second) {
        VoiceMotion::Oblique => 0,
        VoiceMotion::Contrary => 1,
        VoiceMotion::Similar => 2,
        VoiceMotion::Parallel => 4,
    }
}

pub fn melodic_intervals(first: Voicing, second: Voicing) -> u16 {
    let mut penalty = 0;

    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        if first_note == second_note {
            continue;
        }

        penalty += match first_note.distance_to(second_note)
            .as_simple()
            .number()
            .abs()
        {
            Number::UNISON | Number::SECOND => 0,
            Number::THIRD => 1,
            Number::FOURTH => 2,
            Number::FIFTH => 4,
            Number::SIXTH | Number::SEVENTH | Number::OCTAVE => 10,
            _ => unreachable!("all cases covered")
        };
    }

    penalty
}

pub fn common_tones(first: Voicing, second: Voicing, first_chord: RomanChord, second_chord: RomanChord, key: Key) -> u16 {
    debug_assert!(
        completely_voiced(first, first_chord, key),
        "first chord must be completely voiced for common tone scoring",
    );

    debug_assert!(
        completely_voiced(second, second_chord, key),
        "second chord must be completely voiced for common tone scoring",
    );

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

pub fn unison(v: Voicing) -> u16 {
    let mut penalty = 0;

    for v1 in Voice::iter() {
        for v2 in Voice::iter() {
            if v2 > v1 && v[v1] == v[v2] {
                penalty += 1;
            }
        }
    }

    penalty
}