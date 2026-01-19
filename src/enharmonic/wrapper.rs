use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::{EnharmonicEq, EnharmonicOrd};
use crate::enharmonic::WithoutSpelling;

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
#[derive(Copy, Clone, Default)]
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

impl<T: fmt::Debug> fmt::Debug for CmpEnharmonic<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for CmpEnharmonic<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}