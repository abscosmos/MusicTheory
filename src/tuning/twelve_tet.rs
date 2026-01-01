use serde::{Deserialize, Serialize};
use typed_floats::tf32::StrictlyPositiveFinite;
use crate::note::Note;
use crate::pitch::{PitchClass, Pitch};
use crate::tuning::{Cents, Tuning};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwelveToneEqualTemperament {
    pub reference: Note,
    pub freq_hz: StrictlyPositiveFinite,
}

impl TwelveToneEqualTemperament {
    pub const A4_440: Self = Self::new(Note::A4, 440.0).expect("440.0 is strictly positive and finite");

    pub const fn new(reference: Note, freq_hz: f32) -> Option<Self> {
        match StrictlyPositiveFinite::new(freq_hz) {
            Ok(freq_hz) => Some(Self { reference, freq_hz }),
            Err(_) => None,
        }
    }
}

impl Tuning for TwelveToneEqualTemperament {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, Cents)> {
        let semitones_from_reference = 12.0 * (hz / self.freq_hz).log2().get();

        if !semitones_from_reference.is_finite() {
            return None;
        }

        let semitones_from_c0 = semitones_from_reference.round() as i16 + Note::new(Pitch::C, 0).semitones_to(self.reference).0;

        let octave = semitones_from_c0.div_euclid(12);

        let pitch = semitones_from_c0.rem_euclid(12)
            .try_into()
            .expect("i32::rem_euclid(12) must be within [0,12)");

        let pitch = PitchClass::from_repr(pitch)
            .expect("i32::rem_euclid(12) must be within [0,12)")
            .into();

        let note = Note { pitch, octave };

        let semitone_deviation = semitones_from_reference - semitones_from_reference.round();
        let cents = Cents::new(semitone_deviation * 100.0).expect("must be in range");

        if cfg!(debug_assertions) {
            let cents_exp = Cents::from_note(note, hz, self).expect("should be in range");

            let abs_diff = (cents.get() - cents_exp.get()).abs();

            assert!(
                abs_diff < 1e-9 || (abs_diff / semitones_from_reference) < 1e-4,
                "using fract component should be valid (fract: {cents:?}, exp: {cents_exp:?} | note: {note}, hz: {hz})",
            );
        }

        Some((note, cents))
    }

    fn note_to_freq_hz(&self, note: Note) -> Option<StrictlyPositiveFinite> {
        let semitones_from_reference = self.reference.semitones_to(note);

        let hz = self.freq_hz.get() * 2.0_f32.powf(semitones_from_reference.0 as f32 / 12.0);

        StrictlyPositiveFinite::new(hz).ok()
    }
}

impl Default for TwelveToneEqualTemperament {
    fn default() -> Self {
        Self::A4_440
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline(always)]
    fn is_relative_eq(lhs: f32, rhs: f32, error: f32) -> bool {
        ((lhs - rhs) / lhs).abs() < error
    }

    #[test]
    fn test_tuning() {
        let tuning = TwelveToneEqualTemperament::A4_440;

        // D5(-7c)
        let hz = StrictlyPositiveFinite::new(585.0).expect("valid float");
        let (note, cents) = tuning.freq_to_note(hz).expect("should be able to calculate");

        assert_eq!(note, Note::new(Pitch::D, 5));

        assert!(
            is_relative_eq(cents.get(), -6.8803, 1e-5),
            "got cents: {}", cents.get(),
        );

        // B2(+21c)
        let hz = StrictlyPositiveFinite::new(125.0).expect("valid float");
        let (note, cents) = tuning.freq_to_note(hz).expect("should be able to calculate");

        assert_eq!(note, Note::new(Pitch::B, 2));

        assert!(
            is_relative_eq(cents.get(), 21.3095, 1e-5),
            "got cents: {}", cents.get(),
        );
    }
}