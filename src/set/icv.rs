use std::fmt;
use std::ops::Deref;
use crate::set::PitchClassSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

impl fmt::Display for IntervalClassVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [ic1, ic2, ic3, ic4, ic5, ic6] = self.0;

        write!(f, "<{ic1}, {ic2}, {ic3}, {ic4}, {ic5}, {ic6}>")
    }
}

#[cfg(test)]
mod tests {
    use crate::set::{IntervalClassVector, PitchClassSet};

    #[test]
    fn chromatic_aggregate() {
        assert_eq!(
            PitchClassSet::CHROMATIC_AGGREGATE.interval_class_vector(),
            IntervalClassVector::CHROMATIC_AGGREGATE,
        )
    }
}