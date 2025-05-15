use strum::IntoEnumIterator;
use crate::accidental::AccidentalSign;
use crate::enharmonic::EnharmonicEq;
use crate::letter::Letter;
use crate::pitch::Pitch;

#[test]
fn simplify() {
    for offset in -5..5 {
        for letter in Letter::iter() {
            let acc = AccidentalSign { offset };

            let pitch = Pitch::from_letter_and_accidental(letter, acc);

            let simplified = pitch.simplified();

            assert!(
                pitch.eq_enharmonic(&simplified),
                "{pitch:?} ({:?}), {simplified:?} ({:?})",
                pitch.as_pitch_class(),
                simplified.as_pitch_class()
            );
        }
    }
}