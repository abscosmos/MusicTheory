use strum::IntoEnumIterator;
use crate::harmony::Key;
use crate::Interval;
use crate::voice_leading::roman_chord::{RomanChord, ScaleDegree};
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct LeadingToneResolution;

impl LeadingToneResolution {
    pub fn evaluate(&self, first: Voicing, second: Voicing, second_chord: RomanChord, key: Key) -> Result<(), Voice> {
        if second_chord.degree != ScaleDegree::I {
            return Ok(());
        }

        let leading_tone = {
            let mut vii = key.scale_experimental().build_default()[6];

            if RomanChord::mode_has_raised_leading_tone(key.mode) {
                vii = vii.transpose(Interval::AUGMENTED_UNISON);
            }

            vii
        };

        if leading_tone.distance_to(key.tonic) != Interval::MINOR_SECOND {
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
}
