//! Enharmonic equivalence and ordering for musical types.
//!
//! This module provides traits for comparing musical objects in a spelling-agnostic way.
//! Two notes or intervals that are spelled differently but represent the same pitch class
//! or chromatic distance are considered enharmonically equivalent.
//!
//! # Examples
//!
//! ```
//! # use music_theory::prelude::*;
//! use music_theory::enharmonic::EnharmonicEq as _;
//!
//! // C# and Db represent the same pitch class
//! assert!(Pitch::C_SHARP.eq_enharmonic(&Pitch::D_FLAT));
//!
//! // Augmented fourth and diminished fifth span the same chromatic distance
//! assert!(Interval::AUGMENTED_FOURTH.eq_enharmonic(&Interval::DIMINISHED_FIFTH));
//! ```

use std::cmp::{self, Ordering};
use std::hash::{Hash, Hasher};

/// Trait for comparing musical objects enharmonically.
///
/// Two musical objects are enharmonically equal if they represent the same
/// pitch class or chromatic distance, even if they are spelled differently.
/// For example, C# and Db are enharmonically equal because they represent
/// the same pitch class, despite having different spellings.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic::EnharmonicEq as _;
///
/// // Different spellings of the same pitch class
/// assert!(Pitch::C_SHARP.eq_enharmonic(&Pitch::D_FLAT));
/// assert!(Pitch::F_SHARP.eq_enharmonic(&Pitch::G_FLAT));
///
/// // Different spellings of intervals with the same chromatic distance
/// assert!(Interval::AUGMENTED_FOURTH.eq_enharmonic(&Interval::DIMINISHED_FIFTH));
/// assert!(Interval::MAJOR_THIRD.eq_enharmonic(&Interval::DIMINISHED_FOURTH));
/// ```
pub trait EnharmonicEq {
    /// Checks if two musical objects are enharmonically equivalent.
    ///
    /// Returns `true` if the objects represent the same pitch class or chromatic
    /// distance, regardless of their spelling.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicEq as _;
    ///
    /// assert!(Pitch::C_SHARP.eq_enharmonic(&Pitch::D_FLAT));
    /// assert!(Interval::MINOR_THIRD.eq_enharmonic(&Interval::AUGMENTED_SECOND));
    /// ```
    fn eq_enharmonic(&self, other: &Self) -> bool;

    /// Checks if two musical objects are not enharmonically equivalent.
    ///
    /// This is the logical inverse of [`eq_enharmonic`](Self::eq_enharmonic).
    /// The default implementation is almost always sufficient, and should not be overridden without very good reason.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicEq as _;
    ///
    /// assert!(Pitch::C.ne_enharmonic(&Pitch::A));
    /// ```
    fn ne_enharmonic(&self, other: &Self) -> bool {
        !self.eq_enharmonic(other)
    }
}

/// Trait for ordering musical objects enharmonically.
///
/// Provides a comparison that ignores spelling differences and compares
/// based on chromatic position. For intervals, this compares by semitone size.
/// For pitches, this compares by their position in the chromatic scale.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic::EnharmonicOrd as _;
/// use std::cmp::Ordering;
///
/// // C# and Db occupy the same chromatic position
/// assert_eq!(
///     Pitch::C_SHARP.cmp_enharmonic(&Pitch::D_FLAT),
///     Ordering::Equal
/// );
///
/// // C is chromatically lower than D
/// assert_eq!(
///     Pitch::C.cmp_enharmonic(&Pitch::D),
///     Ordering::Less
/// );
///
/// // Compare intervals by chromatic distance
/// assert_eq!(
///     Interval::MAJOR_THIRD.cmp_enharmonic(&Interval::DIMINISHED_FOURTH),
///     Ordering::Equal
/// );
/// ```
pub trait EnharmonicOrd {
    /// Compares two musical objects enharmonically.
    ///
    /// Returns [`Ordering::Equal`] if the objects are enharmonically equivalent,
    /// [`Ordering::Less`] if `self` is lower than `rhs`, and [`Ordering::Greater`]
    /// if `self` is higher than `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(
    ///     Pitch::C_SHARP.cmp_enharmonic(&Pitch::D_FLAT),
    ///     Ordering::Equal
    /// );
    ///
    /// assert_eq!(
    ///     Pitch::C.cmp_enharmonic(&Pitch::E),
    ///     Ordering::Less
    /// );
    ///
    /// assert_eq!(
    ///     Interval::PERFECT_FIFTH.cmp_enharmonic(&Interval::MAJOR_THIRD),
    ///     Ordering::Greater
    /// );
    /// ```
    fn cmp_enharmonic(&self, other: &Self) -> Ordering;

    /// Tests enharmonically less than (for `self` and `other`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert!(Pitch::C.lt_enharmonic(&Pitch::D));
    /// ```
    fn lt_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_lt()
    }

    /// Tests enharmonically less than or equal to (for `self` and `other`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert!(Pitch::C.le_enharmonic(&Pitch::C_SHARP));
    /// assert!(Pitch::C_SHARP.le_enharmonic(&Pitch::D_FLAT));
    /// ```
    fn le_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_le()
    }

    /// Tests enharmonically greater than (for `self` and `other`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert!(Pitch::E.gt_enharmonic(&Pitch::C));
    /// ```
    fn gt_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_gt()
    }

    /// Tests enharmonically greater than or equal to (for `self` and `other`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert!(Pitch::D.ge_enharmonic(&Pitch::C_SHARP));
    /// assert!(Pitch::C_SHARP.ge_enharmonic(&Pitch::D_FLAT));
    /// ```
    fn ge_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_ge()
    }

    /// Compares and returns the maximum of two values enharmonically.
    ///
    /// Returns the second argument if the comparison determines them to be enharmonically equal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert_eq!(Pitch::C.max_enharmonic(Pitch::E), Pitch::E);
    /// ```
    fn max_enharmonic(self, other: Self) -> Self
        where Self: Sized
    {
        cmp::max_by(self, other, EnharmonicOrd::cmp_enharmonic)
    }

    /// Compares and returns the minimum of two values enharmonically.
    ///
    /// Returns the first argument if the comparison determines them to be enharmonically equal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert_eq!(Pitch::C.min_enharmonic(Pitch::E), Pitch::C);
    /// ```
    fn min_enharmonic(self, other: Self) -> Self
        where Self: Sized
    {
        cmp::min_by(self, other, EnharmonicOrd::cmp_enharmonic)
    }

    /// Restrict a value to a certain interval enharmonically.
    ///
    /// Returns `max` if `self` is enharmonically greater than `max`, and `min` if `self` is
    /// enharmonically less than `min`. Otherwise this returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max` enharmonically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert_eq!(Pitch::A.clamp_enharmonic(Pitch::C, Pitch::E), Pitch::C);
    /// assert_eq!(Pitch::D.clamp_enharmonic(Pitch::C, Pitch::E), Pitch::D);
    /// assert_eq!(Pitch::G.clamp_enharmonic(Pitch::C, Pitch::E), Pitch::E);
    /// ```
    fn clamp_enharmonic(self, min: Self, max: Self) -> Self
        where Self: Sized
    {
        assert!(
            min.le_enharmonic(&max),
            "min must be less than max!"
        );

        if self.lt_enharmonic(&min) {
            min
        } else if self.gt_enharmonic(&max) {
            max
        } else {
            self
        }
    }
}

/// Trait for converting musical objects to their spelling-agnostic representation.
///
/// This trait provides a way to extract the enharmonic equivalence class of a musical
/// object by removing spelling information. For example, converting a [`Pitch`] to a
/// [`PitchClass`] discards the specific spelling (C# vs Db) and retains only the
/// chromatic position.
///
/// Types that implement this trait automatically get [`EnharmonicEq`] and [`EnharmonicOrd`]
/// implementations through blanket impls.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic::WithoutSpelling as _;
///
/// // Pitch to PitchClass removes spelling information
/// assert_eq!(Pitch::C_SHARP.without_spelling(), PitchClass::Cs);
/// assert_eq!(Pitch::D_FLAT.without_spelling(), PitchClass::Cs);
///
/// // Both pitches map to the same pitch class
/// assert_eq!(
///     Pitch::C_SHARP.without_spelling(),
///     Pitch::D_FLAT.without_spelling()
/// );
/// ```
pub trait WithoutSpelling {
    /// The type representing the spelling-agnostic form.
    type Unspelled;

    /// Converts to the spelling-agnostic representation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::WithoutSpelling as _;
    ///
    /// assert_eq!(Pitch::C_SHARP.without_spelling(), PitchClass::Cs);
    /// ```
    fn without_spelling(self) -> Self::Unspelled;
}

// TODO: not sure if these should be blanket impls or supertraits, since if
//     'without_spelling' is expensive, but there's a cheap way to check for Eq / Ord,
//     calling Enharmonic{Eq, Ord} must perform the expensive conversion
impl<T: WithoutSpelling<Unspelled: Eq> + Copy> EnharmonicEq for T {
    fn eq_enharmonic(&self, other: &Self) -> bool {
        self.without_spelling() == other.without_spelling()
    }
}

impl<T: WithoutSpelling<Unspelled: Ord> + Copy> EnharmonicOrd for T {
    fn cmp_enharmonic(&self, other: &Self) -> Ordering {
        self.without_spelling().cmp(&other.without_spelling())
    }
}

/// Compares and returns the maximum of two values enharmonically.
///
/// Returns the second argument if the comparison determines them to be enharmonically equal.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic;
///
/// assert_eq!(enharmonic::max(Pitch::C, Pitch::E), Pitch::E);
/// ```
#[inline]
pub fn max<T: EnharmonicOrd>(v1: T, v2: T) -> T {
    v1.max_enharmonic(v2)
}

/// Compares and returns the minimum of two values enharmonically.
///
/// Returns the first argument if the comparison determines them to be enharmonically equal.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic;
///
/// assert_eq!(enharmonic::min(Pitch::C, Pitch::E), Pitch::C);
/// ```
#[inline]
pub fn min<T: EnharmonicOrd>(v1: T, v2: T) -> T {
    v1.min_enharmonic(v2)
}

/// Compares and returns the minimum and maximum of two values enharmonically.
///
/// Returns `(v1, v2)` if the comparison determines them to be enharmonically equal.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic;
///
/// assert_eq!(enharmonic::minmax(Pitch::E, Pitch::C), (Pitch::C, Pitch::E));
/// assert_eq!(enharmonic::minmax(Pitch::C, Pitch::E), (Pitch::C, Pitch::E));
/// ```
#[inline]
pub fn minmax<T: EnharmonicOrd>(v1: T, v2: T) -> (T, T) {
    if v2.lt_enharmonic(&v1) { (v2, v1) } else { (v1, v2) }
}

/// A wrapper that implements standard comparison traits using enharmonic comparison.
///
/// This type allows using enharmonic comparison with standard library collections
/// and algorithms that require [`Ord`], such as [`BTreeMap`], [`BTreeSet`], and
/// sorting methods.
///
/// The wrapper implements [`PartialEq`], [`Eq`], [`PartialOrd`], and [`Ord`] by
/// delegating to the wrapped type's [`EnharmonicEq`] and [`EnharmonicOrd`] implementations.
///
/// # Examples
///
/// Using in a sorted collection:
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic::CmpEnharmonic;
/// use std::collections::BTreeSet;
///
/// let mut set = BTreeSet::new();
/// set.insert(CmpEnharmonic(Pitch::C_SHARP));
/// set.insert(CmpEnharmonic(Pitch::D_FLAT));
/// set.insert(CmpEnharmonic(Pitch::E));
///
/// // C# and Db are enharmonically equivalent, so only one is kept
/// assert_eq!(set.len(), 2);
/// ```
///
/// Using as a HashMap key:
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic::CmpEnharmonic;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert(CmpEnharmonic(Pitch::C_SHARP), "C# major");
///
/// // D♭ is enharmonically equivalent to C#, so it maps to the same value
/// assert_eq!(map.get(&CmpEnharmonic(Pitch::D_FLAT)), Some(&"C# major"));
/// ```
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`BTreeSet`]: std::collections::BTreeSet
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CmpEnharmonic<T>(pub T);

impl<T: EnharmonicEq> PartialEq for CmpEnharmonic<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_enharmonic(&other.0)
    }
}

impl<T: EnharmonicEq> Eq for CmpEnharmonic<T> {}

impl<T: EnharmonicOrd + EnharmonicEq> Ord for CmpEnharmonic<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp_enharmonic(&other.0)
    }
}

impl<T: EnharmonicOrd + EnharmonicEq> PartialOrd for CmpEnharmonic<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: WithoutSpelling<Unspelled: Hash> + Copy> Hash for CmpEnharmonic<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.without_spelling().hash(state);
    }
}