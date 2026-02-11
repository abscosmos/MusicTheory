use crate::Interval;
use crate::interval::{Number, Stability};

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

    pub fn has_interval(&self, interval: Interval) -> bool {
        self.intervals.contains(&interval)
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

    pub fn is_triad(&self) -> bool {
        self.len() == 3
    }

    pub fn is_seventh(&self) -> bool {
        self.interval_of(Number::SEVENTH).is_some()
    }

    pub fn is_extended(&self) -> bool {
        self.interval_of(Number::NINTH).is_some()
            || self.interval_of(Number::ELEVENTH).is_some()
            || self.interval_of(Number::THIRTEENTH).is_some()
    }

    pub fn contains_triad(&self) -> bool {
        self.interval_of(Number::THIRD).is_some()
            && self.interval_of(Number::FIFTH).is_some()
    }

    pub fn is_major(&self) -> bool {
        self.has_interval(Interval::MAJOR_THIRD)
            && self.has_interval(Interval::PERFECT_FIFTH)
    }

    pub fn is_minor(&self) -> bool {
        self.has_interval(Interval::MINOR_THIRD)
            && self.has_interval(Interval::PERFECT_FIFTH)
    }

    pub fn is_diminished(&self) -> bool {
        self.has_interval(Interval::MINOR_THIRD)
            && self.has_interval(Interval::DIMINISHED_FIFTH)
    }

    pub fn is_augmented(&self) -> bool {
        self.has_interval(Interval::MAJOR_THIRD)
            && self.has_interval(Interval::AUGMENTED_FIFTH)
    }

    pub fn is_suspended(&self) -> bool {
        self.interval_of(Number::THIRD).is_none()
            && (self.has_interval(Interval::MAJOR_SECOND)
                || self.has_interval(Interval::PERFECT_FOURTH))
    }

    pub fn is_dominant(&self) -> bool {
        self.has_interval(Interval::MAJOR_THIRD)
            && self.has_interval(Interval::MINOR_SEVENTH)
    }

    pub fn is_consonant(&self) -> bool {
        // TODO: test that this is equivalent checking that none of
        //     all distances between pairs of notes are consonant

        match self.intervals() {
            [] => unreachable!("chord shape must have at least P1"),
            [_p1] => true,
            [_p1, ivl] => ivl.stability().is_some_and(Stability::is_consonant),
            [_p1, _, _] => self.is_major() || self.is_minor(),
            _ => false,
        }
    }
}