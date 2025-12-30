use std::{array, fmt};
use std::ops::Deref;
use crate::pitch::PitchClass;
use crate::set::PitchClassSet;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct IntervalClassVector([u8; 6]);

impl IntervalClassVector {
    pub const CHROMATIC_AGGREGATE: Self = Self([12, 12, 12, 12, 12, 6]);

    pub const fn new(arr: [u8; 6]) -> Option<Self> {
        // interval classes 1-5 (indices 0-4) can appear 0-12 times
        // interval class 6 (index 5, tritone) can only appear 0-6 times
        if arr[0] > 12
            || arr[1] > 12
            || arr[2] > 12
            || arr[3] > 12
            || arr[4] > 12
            || arr[5] > 6
        {
            return None;
        }
        Some(Self(arr))
    }

    // TODO: might get confused with (*icv).len() due to auto-deref
    pub fn total(self) -> u8 {
        self.0.iter().sum()
    }

    pub fn hamming_distance(self, other: Self) -> u8 {
        self.0
            .into_iter()
            .zip(other.0)
            .filter(|(a, b)| a != b)
            .count() as _
    }

    pub fn distinct_classes(self) -> u8 {
        self.0.iter().filter(|&&count| count != 0).count() as u8
    }

    pub fn has_all_classes(self) -> bool {
        self.distinct_classes() == 6
    }

    pub fn is_all_interval(self) -> bool {
        self.into_iter().all(|ic| ic == 1)
    }

    pub fn complement(self) -> Self {
        Self::CHROMATIC_AGGREGATE
            .difference(self)
            .expect("chromatic aggregate is superset of all")
    }

    pub fn is_superset_of(self, other: Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(a, b)| a >= b)
    }

    #[inline]
    pub fn is_subset_of(self, other: Self) -> bool {
        other.is_superset_of(self)
    }

    pub fn difference(self, other: Self) -> Option<Self> {
        if !other.is_subset_of(self) {
            return None;
        }

        // yes, this expects just to wrap it in Some again,
        // but this checks that the only case this function returns
        // None is if 'other' isn't a subset of self
        let diff = Self::new(array::from_fn(|i| self[i] - other[i]))
            .expect("should be valid ICV");

        Some(diff)
    }
}

impl Deref for IntervalClassVector {
    type Target = [u8; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PitchClassSet> for IntervalClassVector {
    fn from(pcset: PitchClassSet) -> Self {
        pcset.interval_class_vector()
    }
}

impl FromIterator<PitchClass> for IntervalClassVector {
    fn from_iter<T: IntoIterator<Item = PitchClass>>(iter: T) -> Self {
        PitchClassSet::from_iter(iter).into()
    }
}

impl fmt::Display for IntervalClassVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [ic1, ic2, ic3, ic4, ic5, ic6] = self.0;

        write!(f, "<{ic1}, {ic2}, {ic3}, {ic4}, {ic5}, {ic6}>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chromatic_aggregate() {
        assert_eq!(
            PitchClassSet::CHROMATIC_AGGREGATE.interval_class_vector(),
            IntervalClassVector::CHROMATIC_AGGREGATE,
        )
    }

    #[test]
    pub fn total() {
        for pcset in (0x000..=0xfff).map(PitchClassSet::new_masked) {
            assert_eq!(
                pcset.interval_class_vector().total(),
                pcset.len() * pcset.len().saturating_sub(1) / 2,
                "should be 'n choose 2', since each pair of pitches"
            );
        }
    }

    #[test]
    fn all_interval_tetrachords() {
        fn from_chromas(chromas: [u8; 4]) -> IntervalClassVector {
            chromas
                .into_iter()
                .map(|chroma| PitchClass::from_repr(chroma).expect("valid chromas"))
                .collect()
        }

        assert!(
            from_chromas([0, 1, 4, 6]).is_all_interval(),
            "[0, 1, 4, 6] should be all interval tetrachord",
        );

        assert!(
            from_chromas([0, 1, 3, 7]).is_all_interval(),
            "[0, 1, 3, 7] should be all interval tetrachord",
        );
    }
}