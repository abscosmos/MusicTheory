use std::ops::{ControlFlow, RangeInclusive};
use typed_floats::tf32::{self, NonNaNFinite, PositiveFinite, StrictlyPositiveFinite};
use crate::generator::NoteGenerator;
use crate::note::Note;
use crate::tuning::{Cents, Tuning};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ValidRangesError {
    #[error("Given start note wasn't computable")]
    StartNotComputable,
    #[error("Provided range did not contain start")]
    InvalidCheckRange,
}

// this is in a newtype to define the EXACT and UNCHECKED constants
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CentsThreshold(pub StrictlyPositiveFinite);

impl CentsThreshold {
    // f32::from_bits(1) is smallest value (f32::MIN_POSITIVE doesn't include subnormal)
    pub const EXACT: Self = match StrictlyPositiveFinite::new(f32::from_bits(1)) {
        Ok(value) => Self(value),
        Err(_) => panic!("unreachable!: smallest positive f32 is in (0, inf)"),
    };

    pub const UNCHECKED: Self = Self(tf32::MAX);
}

impl Default for CentsThreshold {
    fn default() -> Self {
        Self (
            StrictlyPositiveFinite::new(0.01).expect("is in (0, inf)"),
        )
    }
}

/// Threshold for cents between individual notes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepSizeThreshold(RangeInclusive<PositiveFinite>);

impl StepSizeThreshold {
    pub const UNCHECKED: Self = {
        let min = Cents::new(tf32::ZERO.get()).expect("in [0, inf)");
        let max = Cents::new(tf32::MAX.get()).expect("in [0, inf)");

        Self::new(min..=max).unwrap()
    };

    // TODO: should this also ensure !threshold.is_empty()?
    pub const fn new(threshold: RangeInclusive<Cents>) -> Option<Self> {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidRangesReport {
    pub computable: RangeInclusive<Note>,
    pub strictly_monotonic: bool,
    pub valid_inverses: Option<RangeInclusive<Note>>,
    pub step_size_valid: Option<RangeInclusive<Note>>,
    pub cents_within_threshold: Option<RangeInclusive<Note>>,
}

pub fn valid_ranges(
    tuning: &impl Tuning,
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