use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Sub};
use crate::prelude::Semitones;
use crate::set::PitchClassSet;

impl Not for PitchClassSet {
    type Output = Self;

    /// Returns the complement of the pitch class set.
    ///
    /// See [`PitchClassSet::complement`] for more information.
    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl BitAnd for PitchClassSet {
    type Output = Self;

    /// Returns the intersection of two pitch class sets.
    ///
    /// See [`PitchClassSet::intersection`] for more information.
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitXor for PitchClassSet {
    type Output = Self;

    /// Returns the symmetric difference of two pitch class sets.
    ///
    /// See [`PitchClassSet::symmetric_difference`] for more information.
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitOr for PitchClassSet {
    type Output = Self;

    /// Returns the union of two pitch class sets.
    ///
    /// See [`PitchClassSet::union`] for more information.
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl Add<Semitones> for PitchClassSet {
    type Output = Self;

    /// Transpose the pitch class set up by the given number of semitones.
    ///
    /// See [`PitchClassSet::transpose`] for more information.
    fn add(self, rhs: Semitones) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Semitones> for PitchClassSet {
    type Output = Self;

    /// Transpose the pitch class set down by the given number of semitones.
    ///
    /// See [`PitchClassSet::transpose`] for more information.
    fn sub(self, rhs: Semitones) -> Self::Output {
        self + (-rhs)
    }
}