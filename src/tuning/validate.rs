use std::ops::{ControlFlow, RangeInclusive};
use typed_floats::tf32::{self, PositiveFinite};
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
pub struct CentsThreshold(pub PositiveFinite);

impl CentsThreshold {
    pub const EXACT: Self = match PositiveFinite::new(0.0) {
        Ok(zero) => Self(zero),
        Err(_) => panic!("unreachable!: 0.0 is in [0, inf)"),
    };

    pub const UNCHECKED: Self = match PositiveFinite::new(tf32::MAX.get()) {
        Ok(zero) => Self(zero),
        Err(_) => panic!("unreachable!: every strictly positive finite value is positive finite"),
    };
}

impl Default for CentsThreshold {
    fn default() -> Self {
        Self (
            PositiveFinite::new(1e-6).expect("is in [0, inf)")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidRangesResult {
    pub computable: RangeInclusive<Note>,
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

    #[derive(Default)]
    struct State {
        pub last_computable: Option<Note>,

        pub last_valid_inverse: Option<Note>,
        pub hit_invalid_inverse: bool,

        pub last_valid_cents: Option<Note>,
        pub hit_invalid_cents: bool,
    }

    let try_fold = |mut state: State, note: Note| -> ControlFlow<State, State> {
        // 1. check computable
        let Some(freq_hz) = tuning.note_to_freq_hz(note) else {
            return ControlFlow::Break(state);
        };

        let Some((comp_note, cents)) = tuning.freq_to_note(freq_hz) else {
            return ControlFlow::Break(state);
        };

        state.last_computable = Some(note);

        // 2. valid inverse
        if !state.hit_invalid_inverse && note == comp_note {
            state.last_valid_inverse = Some(note);
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

    let above = match range_above.try_fold(State::default(), try_fold) {
        ControlFlow::Continue(s) | ControlFlow::Break(s) => s
    };

    let below = match range_below.try_fold(State::default(), try_fold) {
        ControlFlow::Continue(s) | ControlFlow::Break(s) => s
    };

    // fine to unwrap below with start, because start is already shown to be valid in all cases
    let res = ValidRangesResult {
        computable: below.last_computable.unwrap_or(start)..=above.last_computable.ok_or(ValidRangesError::StartNotComputable)?,
        valid_inverses: above.last_valid_inverse.map(|above|
            below.last_valid_inverse.unwrap_or(start)..=above
        ),
        cents_within_threshold: above.last_valid_cents.map(|above|
            below.last_valid_cents.unwrap_or(start)..=above
        ),
    };

    Ok(res)
}