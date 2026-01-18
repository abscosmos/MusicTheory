//! Enharmonic equivalence and ordering for musical types.
//!
//! This module provides traits for comparing musical objects in a spelling-agnostic way.
//! Two notes or intervals that are spelled differently but represent the same pitch class
//! or chromatic distance are considered enharmonically equivalent.
//!
//! # Examples
//!
//! ```
//! use music_theory::prelude::*;
//! use music_theory::enharmonic::EnharmonicEq;
//!
//! // C# and Db represent the same pitch class
//! assert!(Pitch::C_SHARP.eq_enharmonic(&Pitch::D_FLAT));
//!
//! // Augmented fourth and diminished fifth span the same chromatic distance
//! assert!(Interval::AUGMENTED_FOURTH.eq_enharmonic(&Interval::DIMINISHED_FIFTH));
//! ```

use std::cmp::Ordering;

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
/// use music_theory::prelude::*;
/// use music_theory::enharmonic::EnharmonicEq;
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
    /// use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicEq;
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
    /// use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicEq;
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
/// use music_theory::prelude::*;
/// use music_theory::enharmonic::EnharmonicOrd;
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
    /// use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd;
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
    /// use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd;
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
    /// use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd;
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
    /// use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd;
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
    /// use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd;
    ///
    /// assert!(Pitch::D.ge_enharmonic(&Pitch::C_SHARP));
    /// assert!(Pitch::C_SHARP.ge_enharmonic(&Pitch::D_FLAT));
    /// ```
    fn ge_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_ge()
    }
}