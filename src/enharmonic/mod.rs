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

mod eq;
pub use eq::*;

mod ord;
pub use ord::*;

mod wrapper;
pub use wrapper::*;

use std::cmp::Ordering;

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
