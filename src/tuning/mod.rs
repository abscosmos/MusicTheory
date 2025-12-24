use serde::{Deserialize, Serialize};
use crate::note::Note;
use typed_floats::tf32::{StrictlyPositiveFinite, NonNaNFinite};

mod twelve_tet;
mod just_intonation;
pub use twelve_tet::*;
pub use just_intonation::*;

// this isn't restricted to be in [-100. 100]
// since alternate tuning systems might have higher differences
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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