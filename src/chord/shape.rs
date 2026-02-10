use crate::Interval;
use crate::interval::Number;

pub struct ChordShape {
    intervals: Box<[Interval]>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, thiserror::Error)]
pub enum ChordShapeFromIntervalsError {
    #[error("The provided iterator was empty")]
    Empty,
    #[error("Provided intervals didn't start with a Perfect Unison")]
    NoInitialP1,
    #[error("Provided intervals weren't sorted")]
    NotSorted,
    #[error("Contains duplicate intervals")]
    ContainsDuplicates,
}

impl ChordShape {
    pub fn from_intervals(intervals: impl IntoIterator<Item=Interval>) -> Result<Self, ChordShapeFromIntervalsError> {
        let intervals = intervals.into_iter().collect::<Vec<_>>();

        match intervals.first() {
            Some(&Interval::PERFECT_UNISON) => {},
            Some(_) => return Err(ChordShapeFromIntervalsError::NoInitialP1),
            None => return Err(ChordShapeFromIntervalsError::Empty),
        }

        if !intervals.is_sorted() {
            return Err(ChordShapeFromIntervalsError::NotSorted)
        }

        if intervals.windows(2).any(|w| w[0] == w[1]) {
            return Err(ChordShapeFromIntervalsError::ContainsDuplicates);
        }

        Ok(Self { intervals: intervals.into_boxed_slice() })
    }

    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    pub fn len(&self) -> usize {
        self.intervals.len()
    }

    pub fn interval_of(&self, num: Number) -> Option<Interval> {
        if !num.is_ascending() {
            return None;
        }

        self.intervals
            .iter()
            .find(|i| i.number() == num)
            .copied()
    }
}