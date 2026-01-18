//! Pitch class sets and set operations.
//!
//! A [`PitchClassSet`] represents a collection of pitch classes as a compact 12-bit bitset,
//! where each bit corresponds to one of the 12 pitch classes.
//!
//! # Examples
//!
//! ```
//! use music_theory::prelude::*;
//! use music_theory::set::PitchClassSet;
//!
//! // Create a C major triad
//! let c_major = PitchClassSet::from_iter([
//!     PitchClass::C,
//!     PitchClass::E,
//!     PitchClass::G,
//! ]);
//!
//! // Transpose the set
//! let d_major = c_major + Semitones(2);
//!
//! // Check if two sets are transpositions
//! assert!(c_major.is_transposition_of(d_major));
//!
//! // Compute the interval class vector
//! assert_eq!(
//!     *c_major.interval_class_vector(),
//!     [0, 0, 1, 1, 1, 0],
//! );
//! ```

use crate::pitch::PitchClass;
use crate::set::IntervalClassVector;
use crate::semitone::Semitones;
#[expect(unused_imports, reason = "used in documentation")]
use std::ops::{Add, BitAnd, BitOr, BitXor, Not};

mod fmt;
pub use fmt::*;

mod ops;
#[expect(unused_imports, reason = "ops module is for implementing std::ops traits")]
pub use ops::*;

mod into_iter;
pub use into_iter::*;

/// A collection of pitch classes, stored as a 12-bit bitset.
///
/// Pitch class sets support standard set operations (union, intersection, complement),
/// transposition, inversion, and various analysis functions.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// # use music_theory::set::PitchClassSet;
/// // Create from an iterator
/// let major_triad = PitchClassSet::from_iter([
///     PitchClass::C,
///     PitchClass::E,
///     PitchClass::G,
/// ]);
///
/// // Use set operations
/// let chromatic = PitchClassSet::CHROMATIC_AGGREGATE;
/// let complement = !major_triad;
///
/// assert_eq!(major_triad | complement, chromatic);
/// assert_eq!(major_triad & complement, PitchClassSet::EMPTY);
///
/// // Transpose and invert
/// let transposed = major_triad + Semitones(7);
/// let inverted = major_triad.invert_around(PitchClass::C);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PitchClassSet(u16);

impl PitchClassSet {
    /// An empty pitch class set containing no pitch classes.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::PitchClassSet;
    /// assert!(PitchClassSet::EMPTY.is_empty())
    /// ```
    pub const EMPTY: Self = Self(0);

    /// The chromatic aggregate containing all 12 pitch classes.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::PitchClassSet;
    /// assert_eq!(PitchClassSet::CHROMATIC_AGGREGATE.len(), 12);
    /// ```
    pub const CHROMATIC_AGGREGATE: Self = Self(Self::MASK);

    const MASK: u16 = 0xfff;

    /// Creates a new pitch class set from a raw 12-bit value.
    ///
    /// Returns `None` if the value uses bits beyond the lower 12 bits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::PitchClassSet;
    /// // Binary: 100010010000 represents C, E, G (chromas 0, 4, 7)
    /// let c_major = PitchClassSet::from_bits(0b100010010000).unwrap();
    /// assert_eq!(c_major.len(), 3);
    ///
    /// // Values beyond 12 bits return None
    /// assert_eq!(PitchClassSet::from_bits(0xFFFF), None);
    /// ```
    pub fn from_bits(set: u16) -> Option<Self> {
        (set <= Self::MASK).then_some(Self(set))
    }

    /// Creates a new pitch class set from a value, masking to 12 bits.
    ///
    /// Any bits beyond the lower 12 bits are discarded.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::PitchClassSet;
    /// // Extra bits are masked off
    /// let set = PitchClassSet::from_bits_masked(u16::MAX);
    /// assert_eq!(set, PitchClassSet::CHROMATIC_AGGREGATE);
    /// ```
    #[inline(always)]
    pub fn from_bits_masked(set: u16) -> Self {
        Self(set & Self::MASK)
    }

    /// Returns the raw 12-bit value representing the set.
    ///
    /// Each bit corresponds to a pitch class, with bit 0 representing C.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let set = PitchClassSet::from_iter([PitchClass::C, PitchClass::D]);
    /// assert_eq!(set.bits(), 0b101000000000);
    /// ```
    #[inline(always)]
    pub fn bits(self) -> u16 {
        self.0
    }

    /// Returns `true` if the set contains no pitch classes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::PitchClassSet;
    /// assert!(PitchClassSet::EMPTY.is_empty());
    /// ```
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns the number of pitch classes in the set (cardinality).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let major_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    /// assert_eq!(major_triad.len(), 3);
    /// ```
    #[doc(alias = "cardinality")]
    #[inline(always)]
    pub fn len(self) -> u8 {
        self.0.count_ones() as _
    }
    
    #[inline(always)]
    fn index(pc: PitchClass) -> u8 {
        11 - pc.chroma()
    }

    /// Computes the interval class vector for this pitch class set.
    ///
    /// For more information, see [`IntervalClassVector`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::{IntervalClassVector, PitchClassSet};
    /// let major_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// assert_eq!(
    ///     major_triad.interval_class_vector(),
    ///     IntervalClassVector::new([0, 0, 1, 1, 1, 0]).unwrap(),
    /// )
    /// ```
    pub fn interval_class_vector(self) -> IntervalClassVector {
        let mut icv = [0u8; 6];

        let mut remaining = self.into_iter();

        while let Some(pc1) = remaining.next() {
            // this only iterates over the remaining (which haven't yet been consumed)
            for pc2 in remaining.clone() {
                let interval = pc1.semitones_to(pc2).0;

                let ic = if interval > 6 { 12 - interval } else { interval };

                icv[(ic - 1) as usize] += 1;
            }
        }

        IntervalClassVector::new(icv).expect("all pcsets should be valid icvs")
    }

    /// Returns `true` if the given pitch class is in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let major_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// assert!(major_triad.is_set(PitchClass::E));
    /// assert!(!major_triad.is_set(PitchClass::D));
    /// ```
    pub fn is_set(self, pc: PitchClass) -> bool {
        (self.0 >> Self::index(pc)) & 1 == 1
    }

    /// Returns a new set with the given pitch class added.
    ///
    /// If the pitch class is already in the set, returns an identical set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// let seventh = triad.with_set(PitchClass::B);
    ///
    /// assert_eq!(seventh.len(), 4);
    /// assert!(seventh.is_set(PitchClass::B));
    /// ```
    #[must_use = "This method returns a new PitchClassSet instead of mutating the original"]
    pub fn with_set(self, pc: PitchClass) -> Self {
        Self(self.0 | (1 << Self::index(pc)))
    }

    /// Returns a new set with the given pitch class removed.
    ///
    /// If the pitch class is not in the set, returns an identical set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let seventh = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    ///     PitchClass::B,
    /// ]);
    ///
    /// let triad = seventh.with_cleared(PitchClass::B);
    ///
    /// assert_eq!(triad.len(), 3);
    /// assert!(!triad.is_set(PitchClass::B));
    /// ```
    #[must_use = "This method returns a new PitchClassSet instead of mutating the original"]
    pub fn with_cleared(self, pc: PitchClass) -> Self {
        Self(self.0 & !(1 << Self::index(pc)))
    }

    /// Transpose all pitch classes by the given number of semitones.
    ///
    /// This is equivalent to using the [+ operator](Add::add).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let c_maj = [PitchClass::C, PitchClass::E, PitchClass::G];
    ///
    /// let c_maj_pcset = PitchClassSet::from_iter(c_maj);
    ///
    /// // Transpose up by 7 semitones to get G major
    /// let g_maj_pcset = c_maj_pcset.transpose(Semitones(7));
    ///
    /// assert_eq!(
    ///     g_maj_pcset,
    ///     PitchClassSet::from_iter(
    ///         // maps to: [G, B, D]
    ///         c_maj.map(|pc| pc + Semitones(7))
    ///     ),
    /// );
    /// ```
    #[must_use = "This method returns a new PitchClassSet instead of mutating the original"]
    pub fn transpose(self, semitones: Semitones) -> Self {
        let shift = semitones.normalize().0 as u32;

        // Rotate bits (accounting for 12-bit width, not 16)
        let rotated = (self.0 >> shift) | (self.0 << (12 - shift));

        Self::from_bits_masked(rotated)
    }

    /// Invert the set around the given pitch class axis.
    ///
    /// Inversion around axis `a` maps each pitch class `p` to `(2a - p) mod 12`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// // C major triad [C, E, G] = [0, 4, 7]
    /// let c_major = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// // Invert around C: [0, 4, 7] -> [0, 8, 5] = [C, Gs, F]
    /// assert_eq!(
    ///     c_major.invert_around(PitchClass::C),
    ///     PitchClassSet::from_iter([
    ///         PitchClass::C,
    ///         PitchClass::F,
    ///         PitchClass::Gs,
    ///     ])
    /// );
    /// ```
    #[must_use = "This method returns a new PitchClassSet instead of mutating the original"]
    pub fn invert_around(self, axis: PitchClass) -> Self {
        let mut result = 0u16;

        for i in 0..12 {
            if self.0 & (1 << i) != 0 {
                let new_bit = (10i32 - i).rem_euclid(12) as u32;
                result |= 1 << new_bit;
            }
        }

        // Then transpose by 2×axis (T_2a I formula)
        Self(result).transpose(Semitones(2 * axis.chroma() as i16))
    }

    /// Returns `true` if this set is a superset of the other set.
    ///
    /// A set is a superset of another if it contains all pitch classes in the other set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let major_seventh = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    ///     PitchClass::B,
    /// ]);
    ///
    /// let major_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// assert!(major_seventh.is_superset_of(major_triad));
    /// assert!(!major_triad.is_superset_of(major_seventh));
    /// ```
    #[inline(always)]
    pub fn is_superset_of(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }

    /// Returns `true` if this set is a subset of the other set.
    ///
    /// A set is a subset of another if all of its pitch classes are contained in the other set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let major_seventh = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    ///     PitchClass::B,
    /// ]);
    ///
    /// let major_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// assert!(major_triad.is_subset_of(major_seventh));
    /// assert!(!major_seventh.is_subset_of(major_triad));
    /// ```
    #[inline(always)]
    pub fn is_subset_of(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == self.0
    }

    /// Returns `true` if the two sets have no pitch classes in common.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// use PitchClass as PC;
    ///
    /// let white_keys = PitchClassSet::from_iter([
    ///     PC::C, PC::D, PC::E, PC::F, PC::G, PC::A, PC::B,
    /// ]);
    ///
    /// let black_keys = PitchClassSet::from_iter([
    ///     PC::Cs, PC::Ds, PC::Fs, PC::Gs, PC::As,
    /// ]);
    ///
    /// assert!(white_keys.disjoint(black_keys));
    /// ```
    #[inline(always)]
    pub fn disjoint(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == 0
    }

    /// Returns the union of two pitch class sets.
    ///
    /// The union contains all pitch classes present in either set.
    /// This is equivalent to using the [| operator](BitOr::bitor).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let c_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// let f_triad = PitchClassSet::from_iter([
    ///     PitchClass::F,
    ///     PitchClass::A,
    ///     PitchClass::C,
    /// ]);
    ///
    /// let union = c_triad.union(f_triad);
    /// assert_eq!(union.len(), 5); // C, E, F, G, A
    /// ```
    #[inline(always)]
    pub fn union(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    /// Returns the intersection of two pitch class sets.
    ///
    /// The intersection contains only pitch classes present in both sets.
    /// This is equivalent to using the [& operator](BitAnd::bitand).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let c_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// let e_triad = PitchClassSet::from_iter([
    ///     PitchClass::E,
    ///     PitchClass::G,
    ///     PitchClass::B,
    /// ]);
    ///
    /// assert_eq!(
    ///     c_triad.intersection(e_triad),
    ///     PitchClassSet::from_iter([
    ///         PitchClass::E,
    ///         PitchClass::G,
    ///     ]),
    /// );
    /// ```
    #[inline(always)]
    pub fn intersection(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    /// Returns the difference of two pitch class sets.
    ///
    /// The difference contains pitch classes in this set but not in the other set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// use PitchClass as PC;
    ///
    /// let c_major_scale = PitchClassSet::from_iter([
    ///     PC::C, PC::D, PC::E, PC::F, PC::G, PC::A, PC::B,
    /// ]);
    ///
    /// let c_triad = PitchClassSet::from_iter([PC::C, PC::E, PC::G]);
    ///
    /// assert_eq!(
    ///     c_major_scale.difference(c_triad),
    ///     PitchClassSet::from_iter([PC::D, PC::F, PC::A, PC::B])
    /// );
    ///
    /// assert_eq!(c_triad.difference(c_major_scale), PitchClassSet::EMPTY)
    /// ```
    #[inline(always)]
    pub fn difference(self, rhs: Self) -> Self {
        Self(self.0 & !rhs.0)
    }

    /// Returns the symmetric difference of two pitch class sets.
    ///
    /// The symmetric difference contains pitch classes in either set but not in both.
    /// This is equivalent to using the [^ operator](BitXor::bitxor).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let c_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// let f_triad = PitchClassSet::from_iter([
    ///     PitchClass::F,
    ///     PitchClass::A,
    ///     PitchClass::C,
    /// ]);
    ///
    /// assert_eq!(
    ///     c_triad.symmetric_difference(f_triad),
    ///     PitchClassSet::from_iter([
    ///         // C is excluded because it's in both sets
    ///         PitchClass::E,
    ///         PitchClass::F,
    ///         PitchClass::G,
    ///         PitchClass::A,
    ///     ]),
    /// );
    /// ```
    #[inline(always)]
    pub fn symmetric_difference(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// Returns the complement of this pitch class set.
    ///
    /// The complement contains all pitch classes not in this set.
    /// This is equivalent to using the [! operator](Not::not).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let c_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// let complement = c_triad.complement();
    ///
    /// assert_eq!(complement.len(), 9);
    /// assert!(!complement.is_set(PitchClass::C));
    /// assert!(complement.is_set(PitchClass::D));
    /// ```
    #[inline(always)]
    pub fn complement(self) -> Self {
        Self::from_bits_masked(!self.0)
    }

    /// Returns the normalized (prime) form of this pitch class set.
    ///
    /// Provides a canonical representation for comparing sets in pitch-class set theory.
    /// If the set is not empty, [`PitchClass::C`] is guaranteed to be set.
    ///
    /// If you're comparing normalized pitch class sets, consider [`Self::is_transposition_of`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let c_major = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    ///
    /// let d_major = PitchClassSet::from_iter([
    ///     PitchClass::D,
    ///     PitchClass::Fs,
    ///     PitchClass::A,
    /// ]);
    ///
    /// // The D and C major pcsets normalize to the same thing,
    /// // as they're transpositions of each other
    /// assert_eq!(c_major.normalized(), d_major.normalized());
    /// ```
    #[must_use = "This method returns a new PitchClassSet instead of mutating the original"]
    pub fn normalized(self) -> Self {
        (0..12)
            .map(|s| self + Semitones(s))
            .filter(|pcset| pcset.is_set(PitchClass::C))
            .min_by_key(|pcset| pcset.bits())
            .unwrap_or_default()
    }

    /// Returns `true` if this set is a transposition of the other set.
    ///
    /// Two pitch class sets are transpositions of each other if one can be obtained
    /// by transposing the other by some number of semitones.
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
    /// // D major triad [D, F#, A]
    /// let d_major = PitchClassSet::from_iter([
    ///     PitchClass::D,
    ///     PitchClass::Fs,
    ///     PitchClass::A,
    /// ]);
    ///
    /// // C minor triad [C, Eb, G]
    /// let c_minor = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::Ds,
    ///     PitchClass::G,
    /// ]);
    ///
    /// assert!(c_major.is_transposition_of(d_major));
    /// assert!(d_major.is_transposition_of(c_major));
    /// assert!(!c_major.is_transposition_of(c_minor));
    /// ```
    pub fn is_transposition_of(self, other: Self) -> bool {
        self.normalized() == other.normalized()
    }

    /// Returns a helper type that displays pitch classes as their chroma values.
    ///
    /// # Example
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let set = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G
    /// ]);
    ///
    /// assert_eq!(
    ///     format!("{}", set.display_chromas()),
    ///     "{0, 4, 7}"
    /// );
    /// ```
    pub fn display_chromas(self) -> DisplayChromas {
        DisplayChromas(self)
    }
}

impl FromIterator<PitchClass> for PitchClassSet {
    /// Creates a pitch class set from an iterator of pitch classes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let pitches = vec![PitchClass::C, PitchClass::E, PitchClass::G];
    /// let set = PitchClassSet::from_iter(pitches);
    /// assert_eq!(set.len(), 3);
    /// ```
    fn from_iter<T: IntoIterator<Item = PitchClass>>(iter: T) -> Self {
        let mut new = Self::default();
        new.extend(iter);
        new
    }
}

impl Extend<PitchClass> for PitchClassSet {
    /// Extends the pitch class set with pitch classes from an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let mut set = PitchClassSet::from_iter([PitchClass::C, PitchClass::E]);
    /// set.extend([PitchClass::G, PitchClass::B]);
    /// assert_eq!(set.len(), 4);
    /// ```
    fn extend<T: IntoIterator<Item=PitchClass>>(&mut self, iter: T) {
        *self = iter.into_iter().fold(*self, PitchClassSet::with_set);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cde = PitchClassSet::from_bits(2688).expect("in range");

        assert_eq!(format!("{cde:?}"), "{C (0), D (2), E (4)}");

        assert_eq!(cde, PitchClassSet::from_iter([PitchClass::C, PitchClass::D, PitchClass::E]));
    }

    #[test]
    fn invert() {
        use strum::IntoEnumIterator;

        let pcset = PitchClass::iter()
            .take(7)
            .collect::<PitchClassSet>();

        let inverted = PitchClass::iter()
            .skip(6)
            .chain([PitchClass::C])
            .collect::<PitchClassSet>();

        assert_eq!(
            pcset.invert_around(PitchClass::C),
            inverted,
        );
    }
    
    #[test]
    fn set_ops() {
        let set = PitchClassSet::from_bits(0b000011001100).expect("only necessary bits set");
        
        assert_eq!(!set, PitchClassSet::from_bits(0b111100110011).expect("only necessary bits set"));
        
        assert_eq!(!!set, set);
        
        let cmaj = [
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
            PitchClass::G,
            PitchClass::A,
            PitchClass::B,
        ];
        
        let cmaj_pentatonic = [
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
            PitchClass::G,
        ];
        
        let pcs_cmaj = PitchClassSet::from_iter(cmaj);
        let pcs_cmaj_pentatonic = PitchClassSet::from_iter(cmaj_pentatonic);
        
        assert!(pcs_cmaj.is_superset_of(pcs_cmaj_pentatonic));
        assert!(pcs_cmaj_pentatonic.is_subset_of(pcs_cmaj));
    }
}

