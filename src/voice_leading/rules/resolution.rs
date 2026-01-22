use strum::IntoEnumIterator;
use crate::harmony::Key;
use crate::Interval;
use crate::interval::Number;
use crate::voice_leading::roman_chord::{RomanChord, ScaleDegree};
use crate::voice_leading::{leading_tone, Voice, Voicing};
use crate::voice_leading::rules::voicing::completely_voiced;

pub fn chordal_seventh_resolution(
    first: Voicing,
    first_chord: RomanChord,
    second: Voicing,
    key: Key,
) -> Result<(), Voice> {
    if !first_chord.has_seventh() {
        return Ok(());
    }

    let seventh = first_chord.pitches(key)[3];

    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        if first_note.pitch.as_pitch_class() == seventh.as_pitch_class() {
            let dist = first_note.distance_to(second_note);

            if dist != Interval::PERFECT_UNISON && !matches!(-dist.number(), Number::SECOND) {
                return Err(voice)
            }
        }
    }

    Ok(())
}

pub fn leading_tone_resolution(
    first: Voicing,
    second: Voicing,
    second_chord: RomanChord,
    key: Key,
) -> Result<(), Voice> {
    // for sanity, check the second chord is accurately voiced
    debug_assert!(
        completely_voiced(second, second_chord, key),
        "second chord must be completely voiced for leading tone resolution check",
    );

    if second_chord.degree != ScaleDegree::I {
        return Ok(());
    }

    let leading_tone = leading_tone(key);

    if leading_tone.distance_to(key.tonic) != Interval::MINOR_SECOND {
        // this mode does not have a leading tone, so it's fine by default
        return Ok(());
    }

    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        if first_note.pitch.as_pitch_class() == leading_tone.as_pitch_class() {
            if second_note.pitch.as_pitch_class() != key.tonic.as_pitch_class() {
                return Err(voice);
            }

            if first_note.semitones_to(second_note) != Interval::MINOR_SECOND.semitones() {
                return Err(voice);
            }
        }

        if first_note.pitch.as_pitch_class() == leading_tone.as_pitch_class()
            && second_note.pitch.as_pitch_class() != key.tonic.as_pitch_class()
        {
            return Err(voice);
        }
    }

    Ok(())
}