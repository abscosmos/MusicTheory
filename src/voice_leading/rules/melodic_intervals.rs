use strum::IntoEnumIterator;
use crate::interval::Quality as IntervalQuality;
use crate::Interval;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct MelodicIntervals;

impl MelodicIntervals {
    pub fn evaluate(&self, first: Voicing, second: Voicing) -> Result<(), (Voice, Interval)> {
        for voice in Voice::iter() {
            let first_note = first[voice];
            let second_note = second[voice];

            if first_note == second_note {
                continue;
            }

            let interval = first_note.distance_to(second_note);

            match interval.quality() {
                IntervalQuality::Augmented(_) => return Err((voice, interval)),
                IntervalQuality::Diminished(_) if interval.abs() != Interval::DIMINISHED_FIFTH => return Err((voice, interval)),
                IntervalQuality::Diminished(_) if interval.abs() == Interval::DIMINISHED_FIFTH => {}
                IntervalQuality::Major | IntervalQuality::Minor | IntervalQuality::Perfect => {}
                _ => unreachable!("all cases covered"),
            }
        }

        Ok(())
    }
}
