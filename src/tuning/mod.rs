use std::ops::{Add, Neg, Sub};
use serde::{Deserialize, Serialize};
use crate::note::Note;
use typed_floats::tf32::{StrictlyPositiveFinite, NonNaNFinite};
use crate::pitch::PitchClass;
use crate::semitone::Semitone;

mod twelve_tet;
mod ratio_based;
pub use twelve_tet::*;
pub use ratio_based::*;

pub mod validate;

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

    /// Convert cents to a frequency ratio.
    ///
    /// This is the inverse of [Self::from_ratio]:
    /// - `ratio = 2^(cents / 1200)`
    ///
    /// Returns `None` if the result is not a strictly positive finite number.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::tuning::Cents;
    /// // 100 cents = one semitone in 12-TET
    /// let ratio = Cents::new(100.0).unwrap().to_ratio().unwrap();
    /// assert!((ratio.get() - f32::exp2(12.0f32.recip())).abs() < 0.0001); // 2^(1/12)
    ///
    /// // 600 cents = half octave
    /// let ratio = Cents::new(600.0).unwrap().to_ratio().unwrap();
    /// assert!((ratio.get() - 2f32.sqrt()).abs() < 0.0001); // 2^(1/12)
    ///
    /// // 1200 cents = one octave
    /// let ratio = Cents::OCTAVE.to_ratio().unwrap();
    /// assert!((ratio.get() - 2.0).abs() < 0.0001);
    ///
    /// // 0 cents = unison (ratio 1.0)
    /// let ratio = Cents::new(0.0).unwrap().to_ratio().unwrap();
    /// assert!((ratio.get() - 1.0).abs() < 0.0001);
    ///
    /// // -1200 cents = one octave down
    /// let ratio = (-Cents::OCTAVE).to_ratio().unwrap();
    /// assert!((ratio.get() - 0.5).abs() < 0.0001);
    /// ```
    pub fn to_ratio(self) -> Option<StrictlyPositiveFinite> {
        let ratio = f32::exp2(self.0 / Self::OCTAVE.0);
        StrictlyPositiveFinite::new(ratio).ok()
    }

    /// Convert a frequency ratio to cents.
    ///
    /// This is the inverse of [Self::to_ratio]:
    /// - `cents = 1200 × log2(ratio)`
    ///
    /// Returns `None` if the result is not a strictly positive finite number.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::tuning::Cents;
    /// # use typed_floats::tf32::StrictlyPositiveFinite;
    /// use StrictlyPositiveFinite as F;
    ///
    /// // Octave (2:1) = 1200 cents
    /// let cents = Cents::from_ratio(F::new(2.0).unwrap()).unwrap();
    /// assert!((cents.get() - 1200.0).abs() < 0.001);
    ///
    /// // Perfect fifth in just intonation (3:2) ≈ 702 cents
    /// let cents = Cents::from_ratio(F::new(1.5).unwrap()).unwrap();
    /// assert!((cents.get() - 701.955).abs() < 0.001);
    ///
    /// // Unison (1:1) = 0 cents
    /// let cents = Cents::from_ratio(F::new(1.0).unwrap()).unwrap();
    /// assert!((cents.get() - 0.0).abs() < 0.001);
    /// ```
    pub fn from_ratio(ratio: StrictlyPositiveFinite) -> Option<Self> {
        let cents = Self::OCTAVE.0 * ratio.log2();

        Self::new(cents.get())
    }
}

impl Add for Cents {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new((self.0 + rhs.0).get()).expect("error: cents overflowed during arithmetic operation")
    }
}

impl Sub for Cents {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Neg for Cents {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

pub trait Tuning {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, Cents)>;

    fn note_to_freq_hz(&self, note: Note) -> Option<StrictlyPositiveFinite>;
}

pub fn deviation_between(lhs: &impl Tuning, rhs: &impl Tuning, base: PitchClass) -> Result<[Cents; 12], DeviationBetweenError> {
    use DeviationBetweenError as DevErr;

    let mut cents = [Cents::default(); 12];

    for (chroma, deviation) in cents.iter_mut().enumerate() {
        let pitch_class = base + Semitone(chroma as _);

        let note = Note::new(pitch_class.into(), 4);

        let lhs_freq = lhs.note_to_freq_hz(note).ok_or(DevErr::NoteToFreqError { pitch_class, lhs: true })?;
        let rhs_freq = rhs.note_to_freq_hz(note).ok_or(DevErr::NoteToFreqError { pitch_class, lhs: false })?;

        *deviation = Cents::between_frequencies(lhs_freq, rhs_freq).ok_or(DevErr::InvalidCents(pitch_class))?;
    }

    Ok(cents)
}

#[derive(Clone, thiserror::Error, Debug, Eq, PartialEq)]
pub enum DeviationBetweenError {
    #[error(
        "The {tuning} tuning failed to calculate a frequency for {note}",
        note = Note::new((*pitch_class).into(), 4),
        tuning = if *lhs { "lhs" } else { "rhs" },
    )]
    NoteToFreqError {
        pitch_class: PitchClass,
        lhs: bool,
    },
    #[error("Calculating the cents between tunings of {0} was either NaN or infinite")]
    InvalidCents(PitchClass),
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;
    use crate::pitch::Pitch;
    use crate::tuning::validate::{CentsThreshold, StepSizeThreshold, ValidRangesReport};
    use super::*;

    fn range_within<T: PartialOrd>(outer: RangeInclusive<T>, inner: RangeInclusive<T>) -> bool {
        outer.start() <= inner.start() && outer.end() >= inner.end()
    }

    #[test]
    fn note_freq_inverses() {
        let tunings = [
            &TwelveToneEqualTemperament::A4_440 as &dyn Tuning,
            &RatioBasedTuning::DEFAULT_JUST_INTONATION,
            &RatioBasedTuning::a4_440hz(OctaveRatios::TWELVE_TET, PitchClass::A),
            &RatioBasedTuning::a4_440hz(OctaveRatios::WERCKMEISTER_II, PitchClass::Fs),
            &RatioBasedTuning::a4_440hz(OctaveRatios::QUARTER_COMMA_MEANTONE, PitchClass::B),
            &RatioBasedTuning::a4_440hz(OctaveRatios::KIRNBERGER_III, PitchClass::E),
        ];

        let permissive = {
            let min = Cents::new(50.0).expect("in (-inf, inf)");
            let max = Cents::new(150.0).expect("in (-inf, inf)");

            StepSizeThreshold::new(min..=max).expect("both bounds are positive")
        };

        for (i, tuning) in tunings.into_iter().enumerate() {
            let report = validate::valid_ranges(
                tuning,
                Note::MIDDLE_C,
                None,
                permissive,
                CentsThreshold::default(),
            ).expect("C4 should be computable by all tunings!");

            let ValidRangesReport {
                computable: _,
                strictly_monotonic,
                valid_inverses: Some(valid_inverses),
                step_size_valid: Some(step_size_valid),
                cents_within_threshold: Some(cents_within_threshold),
            } = report else {
                panic!("C4 should pass all checks");
            };

            assert!(
                range_within(
                    valid_inverses.clone(),
                    Note::new(Pitch::C, -100)..=Note::new(Pitch::C, 100),
                ),
                "tuning methods should be inverses for all notes in [C-100, C100], failed: (#{i}): {valid_inverses:?}",
            );

            assert!(
                range_within(
                    step_size_valid.clone(),
                    Note::new(Pitch::C, -100)..=Note::new(Pitch::C, 100),
                ),
                "tuning methods should have sane step sizes for all notes in [C-100, C100], failed: (#{i}): {step_size_valid:?}",
            );

            assert!(
                range_within(
                    cents_within_threshold.clone(),
                    Note::new(Pitch::C, -75)..=Note::new(Pitch::C, 75),
                ),
                "cents should be within threshold for all notes in [C-75, C75], failed: (#{i}): {cents_within_threshold:?}",
            );

            assert!(
                strictly_monotonic,
                "tuning should be strictly monotonic"
            );
        }
    }
}