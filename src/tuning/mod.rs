use std::array;
use serde::{Deserialize, Serialize};
use crate::note::Note;
use typed_floats::tf32::{StrictlyPositiveFinite, NonNaNFinite};
use crate::pitch_class::PitchClass;
use crate::semitone::Semitone;

mod twelve_tet;
mod ratio_based;
pub use twelve_tet::*;
pub use ratio_based::*;

/*
    There's two reasons TwelveToneEqualTemperament exists when it could be represented by RatioBased.
        1. When calculating cents (given by Tuning::freq_to_note), TwelveToneEqualTemperament,
           which internally uses the fractional component after taking the logarithm, is slightly
           more accurate, getting ~2 more (base-10) digits of accuracy.
        2. Much less importantly, in micro-benchmarks, while converting from frequency -> note is
           very similar for both TwelveToneEqualTemperament and RatioBased, TwelveToneEqualTemperament
           is slightly faster at converting note -> frequency (2.1871µs vs 2.8318µs).
*/

// this isn't restricted to be in [-100, 100]
// since alternate tuning systems might have higher differences
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Cents(pub NonNaNFinite);

impl Cents {
    pub const SEMITONE: Self = Self::new(100.0).expect("finite float");
    pub const OCTAVE: Self = Self::new(Self::SEMITONE.get() * 12.0).expect("finite float");

    pub const fn new(c: f32) -> Option<Self> {
        match NonNaNFinite::new(c) {
            Ok(c) => Some(Self(c)),
            _ => None,
        }
    }

    pub const fn get(self) -> f32 {
        self.0.get()
    }

    /// Calculate cents between two frequencies.
    ///
    /// Returns the logarithmic distance from `reference_freq` to `target_freq`:
    /// - Positive result = target is sharp of reference
    /// - Negative result = target is flat of reference
    ///
    /// Formula: `cents = 1200 × log2(target_freq / reference_freq)`
    ///
    /// # Examples
    /// ```
    /// # use music_theory::tuning::Cents;
    /// # use typed_floats::tf32::StrictlyPositiveFinite;
    /// // A4 at 442 Hz is about 7.85 cents sharp of 440 Hz
    /// let cents = Cents::between_frequencies(
    ///     StrictlyPositiveFinite::new(440.0).unwrap(),
    ///     StrictlyPositiveFinite::new(442.0).unwrap(),
    /// ).unwrap();
    /// assert!((cents.get() - 7.85).abs() < 0.1);
    /// ```
    pub fn between_frequencies(
        reference: StrictlyPositiveFinite,
        target: StrictlyPositiveFinite,
    ) -> Option<Self> {
        Self::new(
            Self::OCTAVE.0 * (target / reference).log2()
        )
    }

    pub fn from_note(reference: Note, target: StrictlyPositiveFinite, tuning: &impl Tuning) -> Option<Self> {
        Self::between_frequencies(tuning.note_to_freq_hz(reference)?, target)
    }

    pub fn between_notes(lhs: Note, rhs: Note, tuning: &impl Tuning) -> Option<Self> {
        Self::from_note(lhs, tuning.note_to_freq_hz(rhs)?, tuning)
    }
}

pub trait Tuning {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, Cents)>;

    fn note_to_freq_hz(&self, note: Note) -> Option<StrictlyPositiveFinite>;
}

// TODO: replace checked version of this function, which returns which failed and where
pub fn deviation_between(lhs: &impl Tuning, rhs: &impl Tuning, base: PitchClass) -> [Cents; 12] {
    array::from_fn(|chroma| {
        let pitch_class = base + Semitone(chroma as _);

        let note = Note::new(pitch_class.into(), 4);

        let lhs_freq = lhs.note_to_freq_hz(note).expect("implementations of Tuning should ideally return Some for all MIDI notes");
        let rhs_freq = rhs.note_to_freq_hz(note).expect("implementations of Tuning should ideally return Some for all MIDI notes");

        Cents::between_frequencies(lhs_freq, rhs_freq).expect("difference should be finite and not NaN")
    })
}

#[cfg(test)]
mod tests {
    use crate::pitch_class::PitchClass;
    use super::*;

    #[test]
    fn note_freq_inverses() {
        let tunings = [
            &TwelveToneEqualTemperament::A4_440 as &dyn Tuning,
            &RatioBasedTuning::DEFAULT_JUST_INTONATION,
            &RatioBasedTuning::a4_440hz(OctaveRatios::TWELVE_TET, PitchClass::A),
            &RatioBasedTuning::a4_440hz(OctaveRatios::WERCKMEISTER_II, PitchClass::Fs),
        ];

        for tuning in tunings {
            for note in (0..=u8::MAX).map(Note::from_midi) {
                let freq_hz = tuning.note_to_freq_hz(note).expect("all midi notes should be in freq range");

                if note == Note::new(crate::prelude::Pitch::F, 3) {
                    assert!(note.octave > 0);
                }

                let (calc_note, calc_cents) = tuning.freq_to_note(freq_hz).expect("all midi notes should be in freq range");

                assert_eq!(
                    note, calc_note,
                    "tuning methods should be inverses of each other",
                );

                assert!(
                    calc_cents.get().abs() < 1e-3,
                    "should be exact conversion (got {calc_cents:?} for {note})",
                );
            }
        }
    }
}