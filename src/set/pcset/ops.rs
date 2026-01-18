use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Sub};
use crate::prelude::Semitones;
use crate::set::PitchClassSet;

impl Not for PitchClassSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl BitAnd for PitchClassSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitXor for PitchClassSet {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitOr for PitchClassSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl Add<Semitones> for PitchClassSet {
    type Output = Self;

    /// Transpose the pitch class set up by the given number of semitones.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// // C major triad [C, E, G]
    /// let c_major = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// // Transpose up by 2 semitones to get D major [D, Fs, A]
    /// let d_major = c_major + Semitones(2);
    /// assert_eq!(
    ///     d_major,
    ///     PitchClassSet::from_iter([
    ///         PitchClass::D,
    ///         PitchClass::Fs,
    ///         PitchClass::A,
    ///     ]),
    /// );
    /// ```
    fn add(self, rhs: Semitones) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Semitones> for PitchClassSet {
    type Output = Self;

    /// Transpose the pitch class set down by the given number of semitones.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// // C major triad [C, E, G]
    /// let c_major = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// // Transpose down by 3 semitones to get A major [A, Cs, E]
    /// let a_major = c_major - Semitones(3);
    /// assert_eq!(
    ///     a_major,
    ///     PitchClassSet::from_iter([
    ///         PitchClass::A,
    ///         PitchClass::Cs,
    ///         PitchClass::E,
    ///     ])
    /// );
    /// ```
    fn sub(self, rhs: Semitones) -> Self::Output {
        self + (-rhs)
    }
}