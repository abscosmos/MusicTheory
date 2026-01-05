//! Tuning validation utilities.
//!
//! This module provides tools for validating [`Tuning`] implementations across ranges of notes.
//! It checks properties like computability, monotonicity, inverse accuracy, and step sizes
//! to help identify issues or limitations in tuning systems.
//!
//! The main function is [`valid_ranges`], which expands outward from a starting note
//! and reports the ranges where various validity checks pass.
//! For more information, see documentation for [`valid_ranges`].
//!
//! # Example
//! ```
//! # use music_theory::prelude::*;
//! # use music_theory::tuning::{RatioBasedTuning, validate::*};
//! let report = valid_ranges(
//!     &RatioBasedTuning::DEFAULT_JUST_INTONATION,
//!     Note::MIDDLE_C,
//!     None, // check all notes!
//!     StepSizeThreshold::default(),
//!     CentsThreshold::default(),
//! ).expect("start should be computable");
//!
//! let valid_inverses_range = report.valid_inverses.expect("C4 has valid inverse");
//! let (min, max) = valid_inverses_range.into_inner();
//!
//! // For all notes in [C-100, C100], freq_to_note(note_to_freq_hz(note)) == note
//! assert!(
//!     min <= Note::new(Pitch::C, -100) && max >= Note::new(Pitch::C, 100),
//!     "valid inverses range for just intonation was unexpected small"
//! );
//!
//! assert!(report.strictly_monotonic, "frequency should rise as notes get higher");
//! ```

use std::ops::{ControlFlow, RangeInclusive};
use typed_floats::tf32::{self, NonNaNFinite, PositiveFinite, StrictlyPositiveFinite};
use crate::generator::NoteGenerator;
use crate::note::Note;
use crate::tuning::{Cents, Tuning};

/// Error returned by [`valid_ranges`], usually indicating invalid parameters.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ValidRangesError {
    /// The provided `tuning` could not compute the given `start` note.
    ///
    /// This might have happened in one of two ways:
    /// 1. The `tuning` couldn't [compute the frequency](Tuning::note_to_freq_hz) for the given note
    /// 2. The frequency was computed, but when given the same frequency,
    ///    `tuning` couldn't [compute a note](Tuning::freq_to_note). If the note computed doesn't
    ///    match `start`, indicating that it's not a valid inverse operation, it is **not** an error.
    ///    It is only an error if `tuning` returns `None` for [Tuning::note_to_freq_hz] or `None`
    ///    for [Tuning::freq_to_note], for the start note.
    #[error("Given start note wasn't computable")]
    StartNotComputable,
    /// The provided `check_range` did not contain the start note.
    #[error("Provided range did not contain start")]
    InvalidCheckRange,
}

/// Threshold for allowed deviation in [`Cents`]; used by [`valid_ranges`].
///
/// The validation process tests: `note -> freq -> (note, cents)` and checks that `cents` is
/// close to zero, since it should be an exact inverse.
/// This specifies the maximum allowed deviation from 0 cents.
///
/// # Examples
///
/// ```
/// # use music_theory::tuning::validate::CentsThreshold;
/// // Use the default threshold of 0.01 cents
/// let threshold = CentsThreshold::default();
/// # assert_eq!(threshold.0, 0.01, "example does not match implementation!");
///
/// // Cents returned must always be exactly zero
/// let exact = CentsThreshold::EXACT;
///
/// // Allow any deviation (disables the check)
/// let unchecked = CentsThreshold::UNCHECKED;
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CentsThreshold(pub StrictlyPositiveFinite);

impl CentsThreshold {
    /// [`Cents`] returned must always be exactly `0.0`
    // f32::from_bits(1) is smallest value (f32::MIN_POSITIVE doesn't include subnormal)
    pub const EXACT: Self = match StrictlyPositiveFinite::new(f32::from_bits(1)) {
        Ok(value) => Self(value),
        Err(_) => panic!("unreachable!: smallest positive f32 is in (0, inf)"),
    };

    /// Any [`Cent`](Cents) deviation is allowed; this disables checking
    pub const UNCHECKED: Self = Self(tf32::MAX);
}

impl Default for CentsThreshold {
    /// [`Cents`] returned must be less than `0.01c`
    /// ```
    /// # use music_theory::tuning::validate::CentsThreshold;
    /// assert_eq!(CentsThreshold::default().0, 0.01);
    /// ```
    fn default() -> Self {
        Self (
            StrictlyPositiveFinite::new(0.01).expect("is in (0, inf)"),
        )
    }
}

/// Range for acceptable sizes (in [`Cents`]) between consecutive notes;
/// used by [`valid_ranges`].
///
/// This is meant to catch errors that cause unnaturally large or small step sizes between notes.
///
/// # Examples
///
/// ```
/// # use music_theory::tuning::{Cents, validate::StepSizeThreshold};
/// // Default allows 80-120 cents (±20% from 12-TET's 100 cents)
/// let threshold = StepSizeThreshold::default();
/// # assert_eq!(
/// #     threshold.into_inner(), Cents::new(80.0).unwrap()..=Cents::new(120.0).unwrap(),
/// #     "example doesn't match implementation"
/// # );
///
/// // Allow any step size (disables the check)
/// let unchecked = StepSizeThreshold::UNCHECKED;
/// ```
// note: this doesn't store a RangeInclusive<Cents> in order to maintain a
//       stronger invariant [0, inf) vs [-inf, inf], and to derive Copy
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StepSizeThreshold(PositiveFinite, PositiveFinite);

impl StepSizeThreshold {
    /// Any step size is allowed; this disables checking.
    pub const UNCHECKED: Self = {
        let min = Cents::new(tf32::ZERO.get()).expect("in [0, inf)");
        let max = Cents::new(tf32::MAX.get()).expect("in [0, inf)");

        Self::new(min..=max).unwrap()
    };

    /// Creates a new threshold from a range of acceptable step sizes.
    ///
    /// Returns `None` if either bound is negative.
    pub const fn new(threshold: RangeInclusive<Cents>) -> Option<Self> {
        // TODO: should this also ensure !threshold.is_empty()?
        match (PositiveFinite::new(threshold.start().get()), PositiveFinite::new(threshold.end().get())) {
            (Ok(start), Ok(end)) => Some(Self(start, end)),
            _ => None,
        }
    }

    /// Returns the inner range as `RangeInclusive<Cents>`.
    /// ```
    /// # use music_theory::tuning::{Cents, validate::StepSizeThreshold};
    /// let range = StepSizeThreshold::default().into_inner();
    /// assert_eq!(range, Cents::new(80.0).unwrap()..=Cents::new(120.0).unwrap());
    /// ```
    pub const fn into_inner(self) -> RangeInclusive<Cents> {
        // TODO: const 'From' implementation?
        match (NonNaNFinite::new(self.0.get()), NonNaNFinite::new(self.1.get())) {
            (Ok(start), Ok(end)) => Cents(start)..=Cents(end),
            _ => panic!("unreachable!: PositiveFinite values are always NonNaNFinite"),
        }
    }

    /// Converts into [`RangeInclusive`], as it's not stored as one internally.
    const fn get(self) -> RangeInclusive<PositiveFinite> {
        self.0..=self.1
    }
}

impl Default for StepSizeThreshold {
    // (+/-20% from 12-TET's 100 cents)
    fn default() -> Self {
        let start = Cents::new(80.0).expect("in (-inf, inf)");
        let end = Cents::new(120.0).expect("in (-inf, inf)");

        Self::new(start..=end).expect("cents values are non nan and finite")
    }
}

/// Report from [`valid_ranges`] describing tuning validity over a range of notes.
///
/// Each field represents a different validation check. If a field is `None`, the check failed
/// at the starting note. For more information, see [`valid_ranges`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidRangesReport {
    /// Range of notes where the tuning can compute frequencies (both directions).
    pub computable: RangeInclusive<Note>,
    /// Whether frequencies are strictly monotonically increasing over `computable`.
    pub strictly_monotonic: bool,
    /// Range where `note -> freq -> note` returns the original note.
    pub valid_inverses: Option<RangeInclusive<Note>>,
    /// Range where step sizes between consecutive notes are within [`StepSizeThreshold`].
    pub step_size_valid: Option<RangeInclusive<Note>>,
    /// Range where cents deviation from the inverse is within [`CentsThreshold`].
    pub cents_within_threshold: Option<RangeInclusive<Note>>,
}

/// Validates a tuning over a range of notes, returning a report of validity ranges.
///
/// Starting from `start`, expands outward in both directions checking various properties
/// of the tuning. If `check_range` is `None`, expands until min/max possible note.
/// (This is around notes with octave [`i16::MIN`]/[`i16::MAX`])
///
/// This should be used to test implementations of [`Tuning`] to ensure they produce
/// sane results over a desired range. You may want to limit inputs to [`Tuning::freq_to_note`] or
/// [`Tuning::note_to_freq_hz`] if validation shows your implementation behaviors irregularly for
/// extreme inputs.
///
/// This function does the following checks:
/// 1. **Computable:** Checks that a note is "computable". This means that calling
///    [`Tuning::note_to_freq_hz`] on the note returns `Some(freq_hz)` **and** calling
///    [`Tuning::freq_to_note`] with that same frequency, is `Some((calc_note, calc_cents))`.
///    *The note originally used and the computed note do not need to be equal!*
/// 2. **Inverse:** Checks that the note returned, `calc_note`, is the same as the note used. Or,
///    in other words, [`Tuning::note_to_freq_hz`] and [`Tuning::freq_to_note`] are inverse operations
///    for a given note.
/// 3. **Cents within threshold:** If [`Tuning`] methods are inverses for a given note, checks that
///    the cents returned is within the [threshold](CentsThreshold) specified by the `cents_threshold`
///    parameter. For proper inverses, this should *technically* always be [zero](CentsThreshold::EXACT),
///    but this threshold accounts for floating point error. This check can be disabled using
///    [`CentsThreshold::UNCHECKED`].
/// 4. **Step size within threshold:** Checks that the step size between two notes is not
///    irregularly large or small. The [default](StepSizeThreshold::default) is +/-20% from
///    12-TET's 100 cents. This check can be disabled using [`StepSizeThreshold::UNCHECKED`].
/// 5. **Strictly monotonically increasing:** Over the computable range (see #1), as notes get
///    higher, their frequency also gets higher.
///
/// Unless you have reason to, it's advised to pass `None` into `check_range`, as this will test
/// *almost* all possible notes with your implementation (excluding notes where excessive
/// accidentals would put them out of the range `[C-32768, B32767]`).
///
/// Ideally, your tuning function should also be safe to call (should not panic)
/// for notes in `[C-32768, B32767]`. In case of error, it should return `None`. If your tuning
/// implementation panics, this function **does not** [catch](std::panic::catch_unwind) it.
///
/// # Errors
/// - Returns [`ValidRangesError::StartNotComputable`] if the tuning cannot compute the start note.
/// - Returns [`ValidRangesError::InvalidCheckRange`] if `check_range` doesn't contain `start`.
///
/// # Examples
/// ```
/// # use music_theory::prelude::*;
/// # use music_theory::tuning::{TwelveToneEqualTemperament, validate::*};
/// let report = valid_ranges(
///     &TwelveToneEqualTemperament::A4_440,
///     Note::MIDDLE_C,
///     None, // check all notes!
///     StepSizeThreshold::default(),
///     CentsThreshold::default(),
/// ).expect("start should be computable");
///
/// let valid_inverses_range = report.valid_inverses.expect("C4 has valid inverse");
/// let (min, max) = valid_inverses_range.into_inner();
///
/// // For all notes in [C-100, C100], freq_to_note(note_to_freq_hz(note)) == note
/// assert!(
///     min <= Note::new(Pitch::C, -100) && max >= Note::new(Pitch::C, 100),
///     "valid inverses range for 12TET was unexpected small"
/// );
/// ```
pub fn valid_ranges(
    tuning: &(impl Tuning + ?Sized),
    start: Note,
    check_range: Option<RangeInclusive<Note>>,
    step_size_threshold: StepSizeThreshold,
    cents_threshold: CentsThreshold,
) -> Result<ValidRangesReport, ValidRangesError> {
    // no need to check for empty, since the contains test will catch that
    if check_range.as_ref().is_some_and(|range| !range.contains(&start)){
        return Err(ValidRangesError::InvalidCheckRange);
    }

    let mut range_below = match &check_range {
        Some(range) => NoteGenerator::range(start, *range.start()),
        None => {
            let mut note_gen = NoteGenerator::reversed(start).take_until_overflow();

            let remove_start = note_gen.next();

            assert_eq!(
                remove_start, Some(start),
                "start should be removed from the iterator"
            );

            note_gen
        },
    };

    let mut range_above = match &check_range {
        Some(range) => NoteGenerator::range_inclusive(start, *range.end()),
        None => NoteGenerator::new(start).take_until_overflow(),
    };

    #[derive(Default, Clone)]
    struct State {
        pub last_computable: Option<(Note, StrictlyPositiveFinite)>,
        pub strictly_monotonic: bool,

        pub last_valid_inverse: Option<Note>,
        pub hit_invalid_inverse: bool,

        pub last_valid_step_size: Option<Note>,
        pub hit_invalid_step_size: bool,

        pub last_valid_cents: Option<Note>,
        pub hit_invalid_cents: bool,
    }

    let try_fold = |mut state: State, note: Note, increasing: bool| -> ControlFlow<State, State> {
        // 1. check computable
        let Some(freq_hz) = tuning.note_to_freq_hz(note) else {
            return ControlFlow::Break(state);
        };

        let Some((comp_note, cents)) = tuning.freq_to_note(freq_hz) else {
            return ControlFlow::Break(state);
        };

        let prev_note_freq = state.last_computable.replace((note, freq_hz));

        // 2. valid inverse
        if !state.hit_invalid_inverse && note == comp_note {
            state.last_valid_inverse = Some(note);

            if let Some((_, prev_freq)) = prev_note_freq &&
                ( increasing && (prev_freq >= freq_hz) || !increasing && (prev_freq <= freq_hz) )
            {
                state.strictly_monotonic = false
            }
        } else {
            state.hit_invalid_inverse = true;
        }

        // 3. step size in threshold
        if !state.hit_invalid_inverse && !state.hit_invalid_step_size {
            if let Some((_, prev_freq)) = prev_note_freq {
                if let Some(step_cents) = Cents::between_frequencies(prev_freq, freq_hz)
                    && step_size_threshold.get().contains(&step_cents.0.abs())
                {
                    state.last_valid_step_size = Some(note);
                } else {
                    state.hit_invalid_step_size = true;
                }
            } else {
                state.last_valid_step_size = Some(note);
            }
        }

        // 4. cents in threshold
        if !state.hit_invalid_cents && !state.hit_invalid_inverse && cents.0.abs() < cents_threshold.0 {
            state.last_valid_cents = Some(note);
        } else {
            state.hit_invalid_cents = true;
        }

        ControlFlow::Continue(state)
    };

    let init_state = State {
        strictly_monotonic: true,
        .. State::default()
    };

    let above = match range_above.try_fold(init_state.clone(), |s, n| try_fold(s, n, true)) {
        ControlFlow::Continue(s) | ControlFlow::Break(s) => s
    };

    let below = match range_below.try_fold(init_state, |s, n| try_fold(s, n, false)) {
        ControlFlow::Continue(s) | ControlFlow::Break(s) => s
    };

    // fine to unwrap below with start, because start is already shown to be valid in all cases
    let res = ValidRangesReport {
        computable: {
            let below = below.last_computable.map(|(note, _)| note);
            let above = above.last_computable.map(|(note, _)| note);

            below.unwrap_or(start)..=above.ok_or(ValidRangesError::StartNotComputable)?
        },
        // TODO: do we need to check the boundary between start & (start - 1)?
        strictly_monotonic: below.strictly_monotonic && above.strictly_monotonic,
        valid_inverses: above.last_valid_inverse.map(|above|
            below.last_valid_inverse.unwrap_or(start)..=above
        ),
        step_size_valid: above.last_valid_step_size.map(|above|
            below.last_valid_step_size.unwrap_or(start)..=above
        ),
        cents_within_threshold: above.last_valid_cents.map(|above|
            below.last_valid_cents.unwrap_or(start)..=above
        ),
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::tuning::TwelveToneEqualTemperament;
    use super::*;

    #[test]
    fn validate_twelve_tet() {
        let validate = valid_ranges(
            &TwelveToneEqualTemperament::A4_440,
            Note::MIDDLE_C,
            None,
            StepSizeThreshold::UNCHECKED,
            CentsThreshold::default(),
        );

        assert!(validate.is_ok());
    }
}