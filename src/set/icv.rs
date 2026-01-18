use std::{array, fmt};
use std::ops::Deref;
use crate::pitch::PitchClass;
use crate::set::PitchClassSet;

/// An interval class vector (ICV) representing the intervals present in a set of pitch classes.
///
/// An `IntervalClassVector` is a 6-element array, with each position representing
/// the count of a particular interval class. Interval classes 1 through 6 correspond to
/// semitone distances 1, 2, 3, 4, 5, and 6 (tritone), with larger intervals reduced to inversions.
///
/// Valid ICVs have [0, 12] of class 1-5, and [0, 6] of class 6.
///
/// # Examples
/// ```rust
/// # use music_theory::prelude::*;
/// # use music_theory::set::{PitchClassSet, IntervalClassVector};
/// let icv1 = IntervalClassVector::new([12, 2, 6, 2, 4, 6]).unwrap();
///
/// // has 6 minor thirds / major sixths (class 3)
/// // accessing via 'Index' is allowed due to 'Deref' implementation
/// assert_eq!(icv1[2], 6);
/// assert_eq!(icv1.total(), 32);
/// // use 'Deref' to get the underlying array
/// assert_eq!(*icv1.complement(), [0, 10, 6, 10, 8, 0]);
///
/// let icv2 = IntervalClassVector::from_iter([
///     PitchClass::C,
///     PitchClass::Cs,
///     PitchClass::E,
///     PitchClass::Fs,
/// ]);
///
/// // this is an all-interval tetrachord
/// assert_eq!(*icv2, [1, 1, 1, 1, 1, 1]);
/// assert!(icv2.is_all_interval());
///
/// assert!(icv2.is_subset_of(icv1));
/// ```
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct IntervalClassVector([u8; 6]);

impl IntervalClassVector {
    /// The interval class vector of the empty set, containing no pitch classes.
    ///
    /// This contains zero of every interval class: `<0, 0, 0, 0, 0, 0>`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::{PitchClassSet, IntervalClassVector};
    /// assert_eq!(
    ///     PitchClassSet::EMPTY.interval_class_vector(),
    ///     IntervalClassVector::EMPTY,
    /// );
    /// ```
    pub const EMPTY: Self = Self([0, 0, 0, 0, 0, 0]);

    /// The interval class vector of the chromatic aggregate, containing all 12 pitch classes.
    ///
    /// This contains all interval classes at their maximum counts: `<12, 12, 12, 12, 12, 6>`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::{PitchClassSet, IntervalClassVector};
    /// assert_eq!(
    ///     PitchClassSet::CHROMATIC_AGGREGATE.interval_class_vector(),
    ///     IntervalClassVector::CHROMATIC_AGGREGATE,
    /// );
    ///
    /// let aggregate = IntervalClassVector::CHROMATIC_AGGREGATE;
    /// assert_eq!(aggregate[0], 12, "IC 1: should be 12 m2");
    /// assert_eq!(aggregate[5], 6, "IC 6: should be 6 tritones");
    /// ```
    pub const CHROMATIC_AGGREGATE: Self = Self([12, 12, 12, 12, 12, 6]);

    /// Creates an `IntervalClassVector` from a 6-element array.
    ///
    /// Returns `None` if any interval class count exceeds its maximum:
    /// - Interval classes 1-5 (indices 0-4) must be in [0, 12]
    /// - Interval class 6 (index 5, tritone) must be in range [0, 6]
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv = IntervalClassVector::new([2, 5, 4, 3, 6, 1]);
    /// assert!(icv.is_some());
    ///
    /// // invalid: IC 6 count exceeds maximum of 6
    /// assert!(IntervalClassVector::new([1, 1, 11, 5, 2, 7]).is_none());
    ///
    /// // invalid: IC 3 count exceeds maximum of 12
    /// assert!(IntervalClassVector::new([6, 4, 13, 9, 2, 2]).is_none());
    /// ```
    pub const fn new(arr: [u8; 6]) -> Option<Self> {
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

    /// Returns `true` if this ICV could have been derived from some pitch class set.
    ///
    /// Not all valid ICVs correspond to an actual pitch class set.
    /// This checks whether there exists any [`PitchClassSet`] whose ICV equals `self`.
    ///
    /// Currently, this function is implemented as a **brute-force search through all 4096 possible
    /// pitch class sets**. Avoid calling this method in performance critical loops.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::{IntervalClassVector, PitchClassSet};
    /// let major_triad = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    /// // since this is built from a pitch class set,
    /// // it (obviously) came from a pitch class set
    /// assert!(major_triad.interval_class_vector().came_from_pitch_class_set());
    ///
    /// // manually constructed ICV that happens to match a pcset
    /// let icv = IntervalClassVector::new([0, 0, 1, 1, 1, 0]).unwrap();
    /// assert!(icv.came_from_pitch_class_set());
    ///
    /// // valid ICV that doesn't correspond to any pitch class set
    /// let invalid = IntervalClassVector::new([1, 0, 0, 0, 0, 1]).unwrap();
    /// assert!(!invalid.came_from_pitch_class_set());
    /// ```
    pub fn came_from_pitch_class_set(self) -> bool {
        // TODO: before checking all possible pcsets, check if total() is a triangular number; might be faster on average, bench

        (0..=PitchClassSet::CHROMATIC_AGGREGATE.get())
            .map(|set| PitchClassSet::new_masked(set).interval_class_vector())
            .any(|icv| icv == self)
    }

    /// Returns the total count of all intervals in the ICV.
    ///
    /// This should not be confused with calling `icv.len()`, which will always return `6`
    /// due to calling [`slice::len`] through [`Deref`].
    ///
    /// If constructed from a pitch class set, this is equivalent to
    /// nC2 ([binomial coefficient](https://en.wikipedia.org/wiki/Binomial_coefficient)),
    /// where `n` is the number of distinct pitch classes.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::IntervalClassVector;
    /// let icv = IntervalClassVector::new([2, 5, 7, 0, 3, 0]).unwrap();
    ///
    /// assert_eq!(icv.total(), 2 + 5 + 7 + 3);
    ///
    /// let dom7 = IntervalClassVector::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    ///     PitchClass::As,
    /// ]);
    ///
    /// // 4 choose 2 = 6 total intervals
    /// assert_eq!(dom7.total(), 6);
    /// ```
    // TODO: might get confused with (*icv).len() due to auto-deref
    #[doc(alias = "len")]
    #[doc(alias = "l1_norm")]
    pub fn total(self) -> u8 {
        self.0.iter().sum()
    }

    /// Returns the number of positions where the two ICVs differ.
    ///
    /// This is the Hamming distance between the two vectors, and doesn't take into the
    /// magnitude of difference between two counts.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::set::IntervalClassVector;
    ///
    /// let icv1 = IntervalClassVector::new([1, 2, 3, 4, 5, 6]).unwrap();
    /// // identical icvs have a hamming distance of zero
    /// assert_eq!(icv1.hamming_distance(icv1), 0);
    ///
    /// let icv2 = IntervalClassVector::new([0, 2, 0, 4, 0, 6]).unwrap();
    /// // differs at three positions: 0, 2, 4
    /// assert_eq!(icv1.hamming_distance(icv2), 3);
    ///
    /// let icv3 = IntervalClassVector::new([12, 2, 12, 4, 12, 6]).unwrap();
    /// // although 'icv3' is seemingly "more" different from 'icv1' than 'icv2',
    /// // magnitude isn't taken into account
    /// assert_eq!(icv1.hamming_distance(icv2), icv1.hamming_distance(icv3));
    /// ```
    pub fn hamming_distance(self, other: Self) -> u8 {
        self.0
            .into_iter()
            .zip(other.0)
            .filter(|(a, b)| a != b)
            .count() as _
    }

    /// Manhattan distance (L1 distance)
    ///
    /// This is the sum of absolute differences of each interval class.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv1 = IntervalClassVector::new([1, 2, 3, 4, 5, 1]).unwrap();
    /// let icv2 = IntervalClassVector::new([2, 2, 3, 1, 5, 1]).unwrap();
    /// // ic1 differs by 1, icv4 differs by 3
    /// assert_eq!(icv1.manhattan_distance(icv2), 4);
    /// ```
    #[doc(alias = "l1_distance")]
    pub fn manhattan_distance(self, other: Self) -> u8 {
        self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum()
    }

    /// Euclidean distance squared (L2 distance squared).
    ///
    /// Computes Euclidean distance without taking a square root.
    /// This preserves ordering (d1^2 < d2^2 if and only if d1 < d2) while
    /// avoiding floating-point arithmetic.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv1 = IntervalClassVector::new([1, 2, 3, 4, 2, 1]).unwrap();
    /// let icv2 = IntervalClassVector::new([2, 1, 0, 4, 6, 2]).unwrap();
    /// // 1^2 + 1^2 + 3^2 + 0^2 + 4^2 + 1^1 = 28
    /// assert_eq!(icv1.euclidean_distance_squared(icv2), 28);
    /// ```
    #[doc(alias = "l2_distance_squared")]
    pub fn euclidean_distance_squared(self, other: Self) -> u16 {
        // 'u16' is the return type here, as range is [0, 756]
        self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| {
                let diff = a.abs_diff(*b) as u16;
                diff * diff
            })
            .sum()
    }

    /// L2 norm squared (Euclidean norm squared).
    ///
    /// Computes L2 norm without taking a square root.
    /// This preserves ordering (m1^2 < m2^2 if and only if m1 < m2) while
    /// avoiding floating-point arithmetic.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv = IntervalClassVector::new([1, 2, 3, 4, 5, 1]).unwrap();
    /// assert_eq!(icv.l2_norm_squared(), 56);
    /// ```
    #[doc(alias = "euclidean_norm_squared")]
    pub fn l2_norm_squared(self) -> u16 {
        self.euclidean_distance_squared(Self::default())
    }

    /// Cosine similarity between two interval class vectors.
    ///
    /// Returns a value in `[0.0, 1.0]` where:
    /// - 1.0 means identical direction (same relative interval distribution)
    /// - 0.0 means orthogonal (no similarity in distribution)
    ///
    /// Since no components of [`IntervalClassVector`] can be negative, cosine similarity is
    /// always positive.
    ///
    /// Cosine similarity is scale-invariant: ICVs with the same proportions but different
    /// magnitudes will have similarity close to 1.0.
    ///
    /// Returns `None` if either vector is all zeros.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv1 = IntervalClassVector::new([0, 0, 1, 1, 1, 0]).unwrap();
    /// let icv2 = IntervalClassVector::new([0, 0, 2, 2, 2, 0]).unwrap();
    /// let sim = icv1.cosine_similarity(icv2).unwrap();
    /// assert!((sim - 1.0).abs() < 0.001); // Same direction, different magnitude
    /// ```
    pub fn cosine_similarity(self, other: Self) -> Option<f32> {
        let dot_product: u32 = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| (*a as u32) * (*b as u32))
            .sum();

        let denominator_sq = self.l2_norm_squared() * other.l2_norm_squared();

        if denominator_sq == 0 {
            return None;
        }

        Some((dot_product as f32) / (denominator_sq as f32).sqrt())
    }

    /// Similarity coefficient based on shared interval content.
    ///
    /// Returns a value in [0.0, 1.0] where:
    /// - 1.0 means identical ICVs
    /// - 0.0 means maximally different ICVs
    ///
    /// Uses the formula: `1 - (l1_distance / max_possible_l1_distance)`
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv1 = IntervalClassVector::new([2, 5, 4, 3, 6, 1]).unwrap();
    /// // identical ICVs have max similarity of 1
    /// assert_eq!(icv1.similarity_coefficient(icv1), 1.0);
    ///
    /// // Empty and chromatic aggregate are maximally different
    /// let empty = IntervalClassVector::new([0, 0, 0, 0, 0, 0]).unwrap();
    /// let chromatic = IntervalClassVector::CHROMATIC_AGGREGATE;
    /// assert_eq!(empty.similarity_coefficient(chromatic), 0.0);
    /// ```
    pub fn similarity_coefficient(self, other: Self) -> f32 {
        let max_distance = Self::CHROMATIC_AGGREGATE.total();

        let distance = self.manhattan_distance(other);
        1.0 - (distance as f32 / max_distance as f32)
    }

    /// Returns how many distinct interval classes are present (nonzero count).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::IntervalClassVector;
    /// let icv = IntervalClassVector::new([4, 4, 0, 3, 0, 2]).unwrap();
    /// // has ic1, ic2, ic4, and ic6
    /// assert_eq!(icv.distinct_classes(), 4)
    ///
    /// let major_triad = IntervalClassVector::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G,
    /// ]);
    /// // has ic3, ic4, ic5
    /// assert_eq!(major_triad.distinct_classes(), 3);
    /// ```
    pub fn distinct_classes(self) -> u8 {
        self.0.iter().filter(|&&count| count != 0).count() as u8
    }

    /// Returns `true` if all six interval classes are present.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::IntervalClassVector;
    /// let icv1 = IntervalClassVector::new([4, 2, 1, 2, 4, 2]).unwrap();
    /// assert!(icv1.has_all_classes());
    ///
    /// let icv2 = IntervalClassVector::new([3, 0, 3, 0, 3, 1]).unwrap();
    /// assert!(!icv2.has_all_classes());
    /// ```
    pub fn has_all_classes(self) -> bool {
        self.distinct_classes() == 6
    }

    /// Returns `true` if contains every interval class exactly once.
    ///
    /// # Examples
    /// ```rust
    /// # use music_theory::prelude::PitchClass;
    /// # use music_theory::set::{IntervalClassVector, PitchClassSet};
    /// // [0, 1, 4, 6] is an all interval tetrachord
    /// let icv1 = IntervalClassVector::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::Cs,
    ///     PitchClass::E,
    ///     PitchClass::Fs,
    /// ]);
    /// // icv: <1,1,1,1,1,1>
    /// assert!(icv1.is_all_interval());
    ///
    /// let icv2 = IntervalClassVector::new([2, 5, 6, 8, 1, 4]).unwrap();
    /// // must be exactly one, not more
    /// assert!(!icv2.is_all_interval());
    /// ```
    pub fn is_all_interval(self) -> bool {
        self.into_iter().all(|ic| ic == 1)
    }

    /// Returns the complement ICV (difference from [chromatic aggregate](Self::CHROMATIC_AGGREGATE)).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::IntervalClassVector;
    /// let icv1 = IntervalClassVector::new([4, 4, 3, 3, 5, 2]).unwrap();
    /// // deref used here to get as array
    /// assert_eq!(*icv1.complement(), [8, 8, 9, 9, 7, 4]);
    ///
    /// assert_eq!(
    ///     IntervalClassVector::default().complement(),
    ///     IntervalClassVector::CHROMATIC_AGGREGATE
    /// );
    /// ```
    pub fn complement(self) -> Self {
        Self::CHROMATIC_AGGREGATE
            .difference(self)
            .expect("chromatic aggregate is superset of all")
    }

    /// Returns `true` if `self` is a superset of `other`.
    ///
    /// An ICV is a superset of another if every interval class count in `self`
    /// is greater than or equal to the corresponding count in `other`.
    ///
    /// The inverse of this is [Self::is_subset_of].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let larger = IntervalClassVector::new([3, 4, 5, 6, 7, 2]).unwrap();
    /// let smaller = IntervalClassVector::new([1, 2, 3, 4, 5, 1]).unwrap();
    ///
    /// assert!(larger.is_superset_of(smaller));
    /// assert!(!smaller.is_superset_of(larger));
    ///
    /// // every icv is superset of itself
    /// assert!(larger.is_superset_of(larger));
    ///
    /// // chromatic aggregate is a superset of all ICVs
    /// assert!(IntervalClassVector::CHROMATIC_AGGREGATE.is_superset_of(larger));
    /// ```
    pub fn is_superset_of(self, other: Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(a, b)| a >= b)
    }

    /// Returns `true` if `self` is a subset of `other`.
    ///
    /// An ICV is a subset of another if every interval class count in `self`
    /// is less than or equal to the corresponding count in `other`.
    ///
    /// The inverse of this is [Self::is_superset_of].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let smaller = IntervalClassVector::new([1, 2, 3, 4, 5, 1]).unwrap();
    /// let larger = IntervalClassVector::new([3, 4, 5, 6, 7, 2]).unwrap();
    ///
    /// assert!(smaller.is_subset_of(larger));
    /// assert!(!larger.is_subset_of(smaller));
    ///
    /// // every icv is subset of itself
    /// assert!(smaller.is_subset_of(smaller));
    ///
    /// // all ICVs are subsets of chromatic aggregate
    /// assert!(smaller.is_subset_of(IntervalClassVector::CHROMATIC_AGGREGATE));
    /// ```
    #[inline]
    pub fn is_subset_of(self, other: Self) -> bool {
        other.is_superset_of(self)
    }

    /// Returns the element-wise difference between two ICVs.
    ///
    /// Returns `None` if `other` is not a [subset](Self::is_subset_of) of `self`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv1 = IntervalClassVector::new([5, 6, 7, 8, 9, 3]).unwrap();
    /// let icv2 = IntervalClassVector::new([2, 3, 4, 5, 6, 1]).unwrap();
    ///
    /// let diff = icv1.difference(icv2).unwrap();
    /// assert_eq!(diff, IntervalClassVector::new([3, 3, 3, 3, 3, 2]).unwrap());
    ///
    /// // returns None if other is not a subset
    /// let larger = IntervalClassVector::new([10, 10, 10, 10, 10, 5]).unwrap();
    /// assert!(icv1.difference(larger).is_none() && !larger.is_subset_of(icv1));
    ///
    /// // difference with self is all zeros
    /// let zero = icv1.difference(icv1).unwrap();
    /// assert_eq!(zero, IntervalClassVector::new([0, 0, 0, 0, 0, 0]).unwrap());
    /// ```
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

    /// Dereferences to the underlying `[u8; 6]` array for direct element access.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv = IntervalClassVector::new([2, 4, 12, 8, 10, 6]).unwrap();
    ///
    /// // access elements using 'Index'
    /// assert_eq!(icv[0], 2); // ic1: 2
    /// assert_eq!(icv[4], 10); // ic5: 10
    ///
    /// // use array methods
    /// assert_eq!(icv.first(), Some(&2));
    /// assert!(icv.into_iter().all(|c| c.is_multiple_of(2)));
    /// ```
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
    /// Formats `IntervalClassVector` using angle brackets.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::set::IntervalClassVector;
    /// let icv = IntervalClassVector::new([1, 2, 3, 4, 5, 6]).unwrap();
    /// assert_eq!(icv.to_string(), "<1, 2, 3, 4, 5, 6>");
    /// ```
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