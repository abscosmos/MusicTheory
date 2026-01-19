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
        self.without_spelling() == other.without_spelling()
    }
}

impl<T: WithoutSpelling<Unspelled: Ord> + Copy> EnharmonicOrd for T {
    fn cmp_enharmonic(&self, other: &Self) -> Ordering {
        self.without_spelling().cmp(&other.without_spelling())
    }
}