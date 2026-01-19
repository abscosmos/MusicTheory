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

mod without_spelling;
pub use without_spelling::WithoutSpelling;

mod wrapper;
pub use wrapper::*;

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
