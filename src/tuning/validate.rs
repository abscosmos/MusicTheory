use std::ops::{ControlFlow, RangeInclusive};
use typed_floats::tf32::{self, StrictlyPositiveFinite};
use crate::generator::NoteGenerator;
use crate::note::Note;
use crate::tuning::Tuning;

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
        Ok(zero) => Self(zero),
        Err(_) => panic!("unreachable!: 0.0 is in [0, inf)"),
    };

    pub const UNCHECKED: Self = match StrictlyPositiveFinite::new(tf32::MAX.get()) {
        Ok(zero) => Self(zero),
        Err(_) => panic!("unreachable!: every strictly positive finite value is positive finite"),
    };
}

impl Default for CentsThreshold {
    fn default() -> Self {
        Self (
            StrictlyPositiveFinite::new(1e-5 * 100.0).expect("is in [0, inf)"),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidRangesResult {
    pub computable: RangeInclusive<Note>,
    pub strictly_monotonic: bool,
    pub valid_inverses: Option<RangeInclusive<Note>>,
    pub cents_within_threshold: Option<RangeInclusive<Note>>,
}

pub fn valid_ranges(tuning: &impl Tuning, start: Note, check_range: Option<RangeInclusive<Note>>, cents_threshold: CentsThreshold) -> Result<ValidRangesResult, ValidRangesError> {
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

        // 3. cents in threshold
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
    let res = ValidRangesResult {
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
        cents_within_threshold: above.last_valid_cents.map(|above|
            below.last_valid_cents.unwrap_or(start)..=above
        ),
    };

    Ok(res)
}