use std::ops::RangeInclusive;
use crate::generator::NoteGenerator;
use crate::note::Note;
use crate::tuning::Tuning;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ComputableRangeError {
    #[error("Given start note wasn't computable")]
    StartNotComputable,
    #[error("Provided range did not contain start")]
    InvalidCheckRange,
}

pub fn computable_range(tuning: &impl Tuning, start: Note, check_range: Option<RangeInclusive<Note>>) -> Result<RangeInclusive<Note>, ComputableRangeError> {
    // no need to check for empty, since the contains test will catch that
    if check_range.as_ref().is_some_and(|range| !range.contains(&start)){
        return Err(ComputableRangeError::InvalidCheckRange);
    }

    let range_below = match &check_range {
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

    let is_computable = |note: &Note| -> bool {
        let Some(freq_hz) = tuning.note_to_freq_hz(*note) else {
            return false;
        };

        let Some(_) = tuning.freq_to_note(freq_hz) else {
            return false;
        };

        true
    };

    let range_above = match &check_range {
        Some(range) => NoteGenerator::range_inclusive(start, *range.end()),
        None => NoteGenerator::new(start).take_until_overflow(),
    };

    // the only way this returns none is if the first element (start) isn't computable
    let above_fail = range_above.take_while(is_computable).last()
        .ok_or(ComputableRangeError::StartNotComputable)?;

    // fine to unwrap with start, because start is already shown to be valid
    let below_fail = range_below.take_while(is_computable).last().unwrap_or(start);

    Ok(below_fail..=above_fail)
}