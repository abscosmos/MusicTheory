use crate::harmony::Key;
use crate::Interval;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::Voicing;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct LeadingToneDoubling;

impl LeadingToneDoubling {
    pub fn evaluate(&self, voicing: Voicing, chord: RomanChord, key: Key) -> bool {
        let chord_pitches = chord.pitches(key);

        let leading_tone = {
            let mut vii = key.scale_experimental().build_default()[6];

            if RomanChord::mode_has_raised_leading_tone(key.mode) {
                vii = vii.transpose(Interval::AUGMENTED_UNISON);
            }

            vii
        };

        let chordal_seventh = chord.has_seventh().then(|| chord_pitches[3]);

        for note in voicing.iter() {
            let count = voicing.iter()
                .filter(|n| n.pitch == note.pitch)
                .count();

            if count > 1 {
                if note.pitch == leading_tone {
                    return false;
                }

                if chordal_seventh.is_some_and(|seventh| note.pitch == seventh) {
                    return false;
                }
            }
        }

        true
    }
}
