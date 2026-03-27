use crate::{EnharmonicEq, Interval};
use crate::chord::known;
use crate::chord::known::KnownChord;
use crate::interval::{Number, Stability};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    #[expect(clippy::len_without_is_empty, reason = "ChordShape cannot be empty")]
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

    /// Returns `true` if the given chord degree appears more than once in this shape.
    ///
    /// A degree is repeated when two intervals share the same [`Number`] but differ in quality
    /// (e.g. both M3 and m3 present). Always returns `false` for descending numbers.
    pub fn has_repeated_chord_step(&self, number: Number) -> bool {
        if !number.is_ascending() {
            return false;
        }

        self.intervals.windows(2).any(|w|
            w[0].number() == number && w[1].number() == number
        )
    }

    /// Returns `true` if any chord degree appears more than once in this shape.
    pub fn has_any_repeated_diatonic_note(&self) -> bool {
        self.intervals.windows(2).any(|w| w[0].number() == w[1].number())
    }

    /// Returns a new shape with `interval` added.
    ///
    /// Returns `None` if `interval` is already present or is descending.
    pub fn with_interval(&self, interval: Interval) -> Option<Self> {
        if !interval.is_ascending() || self.has_interval(interval) {
            return None;
        }

        let mut intervals = self.intervals.to_vec();
        let pos = intervals.partition_point(|&i| i < interval);
        intervals.insert(pos, interval);

        Some(Self { intervals: intervals.into_boxed_slice() })
    }

    /// Returns a new shape with the interval at `number` removed.
    ///
    /// Returns `None` if the degree doesn't exist in this shape, or if `number` is a unison.
    /// (P1 cannot be removed).
    pub fn without_degree(&self, number: Number) -> Option<Self> {
        if number == Number::UNISON {
            return None;
        }

        let pos = self.intervals.iter().position(|i| i.number() == number)?;
        let mut intervals = self.intervals.to_vec();
        intervals.remove(pos);

        Some(Self { intervals: intervals.into_boxed_slice() })
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

    pub fn is_dominant_seventh(&self) -> bool {
        self.has_interval(Interval::MAJOR_THIRD)
            && self.has_interval(Interval::PERFECT_FIFTH)
            && self.has_interval(Interval::MINOR_SEVENTH)
    }

    pub fn is_half_diminished(&self) -> bool {
        self.has_interval(Interval::MINOR_THIRD)
            && self.has_interval(Interval::DIMINISHED_FIFTH)
            && self.has_interval(Interval::MINOR_SEVENTH)
    }

    pub fn is_diminished_seventh(&self) -> bool {
        self.has_interval(Interval::MINOR_THIRD)
            && self.has_interval(Interval::DIMINISHED_FIFTH)
            && self.has_interval(Interval::DIMINISHED_SEVENTH)
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

    #[inline]
    pub fn known(&self) -> Option<KnownChord> {
        known::find_known(&self.intervals)
    }
}

impl EnharmonicEq for ChordShape {
    fn eq_enharmonic(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.intervals()
            .iter()
            .zip(other.intervals())
            .all(|(this, other)| this.eq_enharmonic(other))
    }
}

impl From<KnownChord> for ChordShape {
    fn from(known: KnownChord) -> Self {
        Self { intervals: known.intervals().into() }
    }
}