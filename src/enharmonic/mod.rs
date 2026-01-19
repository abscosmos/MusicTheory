//! Enharmonic equivalence and ordering for musical types.
//!
//! This module provides traits and utilities for comparing musical objects in a
//! spelling-agnostic way. The core traits are [`EnharmonicEq`] and [`EnharmonicOrd`].
//! The [`WithoutSpelling`] trait converts objects to a spelling-agnostic form.
//!
//! The module also provides the [`CmpEnharmonic`] wrapper, which uses enharmonic
//! equivalence for [`Eq`] / [`PartialEq`] and [`Ord`] / [`PartialOrd`].
//!
//! Helper functions like [`max`], [`min`], and [`minmax`] provide convenient ways to
//! compare values enharmonically, similar to methods in [`std::cmp`].
//!
//! # Examples
//!
//! ```
//! # use music_theory::{Pitch, PitchClass};
//! # use std::collections::BTreeSet;
//! use music_theory::enharmonic::{
//!     self,
//!     EnharmonicEq as _,
//!     EnharmonicOrd as _,
//!     WithoutSpelling as _,
//!     CmpEnharmonic,
//! };
//!
//! // Compare pitches enharmonically
//! assert!(Pitch::C_SHARP.eq_enharmonic(&Pitch::D_FLAT));
//! assert!(Pitch::C.lt_enharmonic(&Pitch::E));
//!
//! // Pitches without spelling are PitchClasses
//! assert_eq!(Pitch::D_FLAT.without_spelling(), PitchClass::Cs);
//!
//! // Find the maximum pitch enharmonically
//! let highest = enharmonic::max(Pitch::C_SHARP, Pitch::E);
//! assert_eq!(highest, Pitch::E);
//!
//! // Use CmpEnharmonic wrapper for collections
//! let mut pitches = BTreeSet::new();
//! pitches.insert(CmpEnharmonic(Pitch::C_SHARP));
//! pitches.insert(CmpEnharmonic(Pitch::D_FLAT)); // Treated as duplicate
//! assert_eq!(pitches.len(), 1);
//! ```

mod eq;
pub use eq::*;

mod ord;
pub use ord::*;

mod without_spelling;
pub use without_spelling::WithoutSpelling;
pub(crate) use without_spelling::defer as defer_without_spelling;

mod wrapper;
pub use wrapper::*;

/// Compares and returns the maximum of two values enharmonically.
///
/// Returns the second argument if the comparison determines them to be enharmonically equal.
///
/// # Examples
///
/// ```
/// # use music_theory::Pitch;
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
/// # use music_theory::Pitch;
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
/// # use music_theory::Pitch;
/// use music_theory::enharmonic;
///
/// assert_eq!(enharmonic::minmax(Pitch::E, Pitch::C), (Pitch::C, Pitch::E));
/// assert_eq!(enharmonic::minmax(Pitch::C, Pitch::E), (Pitch::C, Pitch::E));
/// ```
#[inline]
pub fn minmax<T: EnharmonicOrd>(v1: T, v2: T) -> (T, T) {
    if v2.lt_enharmonic(&v1) { (v2, v1) } else { (v1, v2) }
}
