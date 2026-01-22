use strum::IntoEnumIterator;
use crate::harmony::Key;
use crate::Interval;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct ChordalSeventhResolution;

impl ChordalSeventhResolution {
    pub fn evaluate(&self, first: Voicing, first_chord: RomanChord, second: Voicing, key: Key) -> Result<(), Voice> {
        if !first_chord.has_seventh() {
            return Ok(());
        }

        let seventh = first_chord.pitches(key)[3];

        for voice in Voice::iter() {
            let first_note = first[voice];
            let second_note = second[voice];

            if first_note.pitch.as_pitch_class() == seventh.as_pitch_class() {
                let dist = first_note.distance_to(second_note);

                if dist != Interval::PERFECT_UNISON && !matches!(-first_note.distance_to(second_note), Interval::MAJOR_SECOND | Interval::MINOR_SECOND) {
                    return Err(voice)
                }
            }
        }

        Ok(())
    }
}
