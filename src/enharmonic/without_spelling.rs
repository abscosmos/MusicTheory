use std::cmp::Ordering;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};

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
        defer::eq(self, other)
    }
}

impl<T: WithoutSpelling<Unspelled: Ord> + Copy> EnharmonicOrd for T {
    fn cmp_enharmonic(&self, other: &Self) -> Ordering {
        defer::cmp(self, other)
    }
}

pub(crate) mod defer {
    use std::cmp::Ordering;
    use super::WithoutSpelling;

    pub fn eq<T: WithoutSpelling<Unspelled: Eq> + Copy>(v1: &T, v2: &T) -> bool {
        v1.without_spelling() == v2.without_spelling()
    }

    pub fn cmp<T: WithoutSpelling<Unspelled: Ord> + Copy>(v1: &T, v2: &T) -> Ordering {
        v1.without_spelling().cmp(&v2.without_spelling())
    }
}