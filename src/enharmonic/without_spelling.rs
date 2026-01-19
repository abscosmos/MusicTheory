use crate::enharmonic::EnharmonicEq;

/// Trait for converting musical objects to their spelling-agnostic representation.
///
/// This trait provides a way to extract the enharmonic equivalence class of a musical
/// object by removing spelling information. For example, converting a [`Pitch`] to a
/// [`PitchClass`] discards the specific spelling (C# vs Db) and retains only the
/// chromatic position.
///
/// # Supertraits
/// This requires [`EnharmonicEq`] and [`EnharmonicOrd`] to be implemented.
/// Unless [`WithoutSpelling::without_spelling`] is expensive for your type, and you can implement
/// [`EnharmonicEq`] and [`EnharmonicOrd`] cheaper, you should implement it like this:
/// ```
/// use music_theory::enharmonic::{WithoutSpelling, EnharmonicEq, EnharmonicOrd};
/// use std::cmp::Ordering;
/// #
/// # #[derive(Clone)]
/// # pub struct YourType;
/// # impl Copy for YourType {}
/// #
/// # pub struct YourTypeUnspelled;
/// #
/// # impl PartialEq for YourTypeUnspelled {
/// #     fn eq(&self, other: &Self) -> bool {
/// #         unimplemented!()
/// #     }
/// # }
/// # impl Eq for YourTypeUnspelled {}
/// #
/// # impl PartialOrd for YourTypeUnspelled {
/// #     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
/// #         Some(self.cmp(other))
/// #     }
/// # }
/// #
/// # impl Ord for YourTypeUnspelled {
/// #     fn cmp(&self, other: &Self) -> Ordering {
/// #         unimplemented!()
/// #     }
/// # }
///
/// impl WithoutSpelling for YourType {
///     type Unspelled = YourTypeUnspelled;
///
///     fn without_spelling(self) -> Self::Unspelled {
///         unimplemented!()
///     }
/// }
///
/// impl EnharmonicEq for YourType {
///     fn eq_enharmonic(&self, other: &Self) -> bool {
///         // You may need to clone 'self' and 'other' here,
///         // if 'YourType' isn't copy. If you can implement
///         // 'YourType::eq_enharmonic' cheaper than the combined
///         // cost of 'YourType::clone' and 'YourType::without_spelling',
///         // then *don't* do this.
///         self.without_spelling() == other.without_spelling()
///     }
/// }
///
/// impl EnharmonicOrd for YourType {
///     fn cmp_enharmonic(&self, other: &Self) -> Ordering {
///         // See the comment about 'eq_enharmonic' above
///         self.without_spelling().cmp(&other.without_spelling())
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic::WithoutSpelling as _;
///
/// // Pitch to PitchClass removes spelling information,
/// // and both map to the same pitch class
/// assert_eq!(Pitch::C_SHARP.without_spelling(), PitchClass::Cs);
/// assert_eq!(Pitch::D_FLAT.without_spelling(), PitchClass::Cs);
/// ```
pub trait WithoutSpelling: EnharmonicEq {
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