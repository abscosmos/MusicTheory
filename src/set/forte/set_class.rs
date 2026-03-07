use std::cmp::Ordering;
use std::fmt;
use std::num::NonZeroU8;
use crate::PitchClass;
use crate::set::forte::set_class_form::SetClassForm;
use crate::set::{check_triangular, IntervalClassVector, PitchClassSet};
use super::tables;

/// A pitch class set equivalence class under transposition and inversion (TnI).
///
/// `SetClass` identifies a set by its [Forte number](https://en.wikipedia.org/wiki/Forte_number),
/// written as `cardinality-index`, like `3-11`. Z-related set classes include a `Z` prefix
/// before the index, like `4-Z15`.
///
/// Two pitch class sets share the same `SetClass` if one can be obtained from the other
/// by any combination of transposition and inversion. To distinguish between prime and
/// inverted forms (A vs. B), use [`SetClassForm`] instead.
///
/// # Examples
///
/// ```
/// # use music_theory::PitchClass;
/// # use music_theory::set::{PitchClassSet, SetClass};
/// use PitchClass as PC;
///
/// // C major and C minor belong to the same set class (3-11)
/// let c_major = PitchClassSet::from_iter([PC::C, PC::E, PC::G]);
/// let c_minor = PitchClassSet::from_iter([PC::C, PC::Ds, PC::G]);
///
/// assert_eq!(c_major.set_class(), c_minor.set_class());
/// assert_eq!(c_major.set_class().to_string(), "3-11");
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[doc(alias = "ForteNumberTnI")]
pub struct SetClass {
    // TODO: compress this into a u16 eventually
    pub(super) cardinality: u8,
    pub(super) index: NonZeroU8,
    pub(super) z_index: Option<NonZeroU8>,
}

/// Error returned by [`SetClass::new`] when construction fails.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NewSetClassError {
    /// The cardinality was greater than 12.
    #[error("Cardinality must be in [0, 12]")]
    InvalidCardinality,
    /// The index was out of range for the given cardinality.
    #[error("Index {index} out of range for SetClass with cardinality {cardinality}. Expected [1, {max}]")]
    InvalidIndex {
        /// The cardinality of the [`SetClass`].
        cardinality: u8,
        /// The intended (invalid) index.
        index: u8,
        /// The maximum valid index for the given `cardinality`.
        max: u8,
    },
}

impl SetClass {
    /// Creates a `SetClass` from a cardinality and Forte index.
    ///
    /// # Errors
    /// Returns [`InvalidCardinality`](NewSetClassError::InvalidCardinality) if `cardinality > 12`, or
    /// [`InvalidIndex`](NewSetClassError::InvalidIndex) if `index` is not in `[1, max]` for the
    /// given cardinality.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{SetClass, forte::NewSetClassError};
    /// let triad = SetClass::new(3, 11).unwrap();
    /// assert_eq!(triad.to_string(), "3-11");
    ///
    /// assert_eq!(SetClass::new(13, 1), Err(NewSetClassError::InvalidCardinality));
    /// assert!(matches!(
    ///     SetClass::new(3, 99),
    ///     Err(NewSetClassError::InvalidIndex { .. }),
    /// ));
    /// ```
    pub const fn new(cardinality: u8, index: u8) -> Result<Self, NewSetClassError> {
        if cardinality > 12 {
            return Err(NewSetClassError::InvalidCardinality);
        }

        let max_index = Self::max_index(cardinality).expect("should've already checked range");

        if 1 > index || index > max_index {
            return Err(NewSetClassError::InvalidIndex { cardinality, index, max: max_index });
        }

        let index = NonZeroU8::new(index).expect("should've checked for zero");

        let z_index = tables::lookup(cardinality, index.get()).z_related;

        Ok(Self { cardinality, index, z_index })
    }

    /// Returns the first set class whose [interval class vector](IntervalClassVector) matches `icv`.
    ///
    /// For Z-related pairs, which are set classes that share the same ICV, the lower-indexed
    /// set class is returned.
    ///
    /// Returns `None` if `icv` does not correspond to any pitch class set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{IntervalClassVector, SetClass};
    /// let triad = IntervalClassVector::new([2, 2, 2, 1, 2, 1]).unwrap();
    ///
    /// // this only returns 5-Z12, not 5-Z36, which also has the same ICV
    /// let set_class = SetClass::from_icv_first(triad).unwrap();
    /// assert_eq!(set_class.to_string(), "5-Z12");
    /// ```
    pub fn from_icv_first(icv: IntervalClassVector) -> Option<Self> {
        let total = icv.total();

        let cardinality = if total != 0 {
            check_triangular(total as _)? as u8 + 1
        } else {
            0
        };

        let entry = tables::lookup_entries_cardinality(cardinality)
            .iter()
            .find(|e| e.prime_form.interval_class_vector() == icv)?;

        let set_class = Self {
            cardinality: entry.cardinality,
            index: entry.index,
            z_index: entry.z_related,
        };

        Some(set_class)
    }

    /// Returns the set class's cardinality, the number of distinct pitch classes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::SetClass;
    /// let triad = SetClass::new(3, 11).unwrap();
    /// assert_eq!(triad.cardinality(), 3);
    /// ```
    #[inline]
    pub const fn cardinality(self) -> u8 {
        self.cardinality
    }

    /// Returns the Forte index of this set class within its cardinality.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::SetClass;
    /// let triad = SetClass::new(3, 11).unwrap();
    ///
    /// // 3-11 is the 11th set class of cardinality 3
    /// assert_eq!(triad.index(), 11);
    /// ```
    #[inline]
    pub const fn index(self) -> u8 {
        self.index.get()
    }

    /// Returns the Z-related set class, if one exists.
    ///
    /// Z-related set classes have identical [interval class vectors](IntervalClassVector)
    /// but are not related by transposition or inversion. They always occur in pairs,
    /// so calling `z_related()` on the result returns the original set class.
    ///
    /// Returns `None` if this set class has no Z-relation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::SetClass;
    /// // 4-Z15 and 4-Z29 are Z-related: same ICV, different set class
    /// let z15 = SetClass::new(4, 15).unwrap();
    /// let z29 = z15.z_related().unwrap();
    ///
    /// assert_eq!(z29.to_string(), "4-Z29");
    /// assert_eq!(z15.interval_class_vector(), z29.interval_class_vector());
    ///
    /// // 3-11 has no Z-relation; it's the only set class with ICV <0,0,1,1,1,0>
    /// let triad = SetClass::new(3, 11).unwrap();
    /// assert!(triad.z_related().is_none());
    /// ```
    #[inline]
    pub const fn z_related(self) -> Option<Self> {
        // FIXME(const): use Try
        let Some(index) = self.z_index else {
            return None;
        };

        let z_related = Self {
            cardinality: self.cardinality,
            index,
            z_index: Some(self.index),
        };

        Some(z_related)
    }

    /// Returns the prime form of this set class as a [`PitchClassSet`].
    ///
    /// The prime form is the most compact representation of the set, starting on C.
    /// All pitch class sets in the same set class share the same prime form.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::PitchClass;
    /// # use music_theory::set::{PitchClassSet, SetClass};
    /// // Prime form of 3-11 is [0, 3, 7]
    /// let triad = SetClass::new(3, 11).unwrap();
    ///
    /// assert_eq!(
    ///     triad.prime_form(),
    ///     PitchClassSet::from_iter([
    ///         PitchClass::C,
    ///         PitchClass::Ds,
    ///         PitchClass::G,
    ///     ]),
    /// );
    /// ```
    #[inline]
    pub const fn prime_form(self) -> PitchClassSet {
        tables::lookup(self.cardinality, self.index.get()).prime_form
    }

    /// Returns the complement set class.
    ///
    /// The complement set class is the set class of the complement of this set class's prime form.
    /// The complement of a cardinality-`n` set class has cardinality `12 - n`.
    /// Except for some cardinality-6 set classes, the complement has the same index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::SetClass;
    /// // 3-11 complements to 9-11
    /// let triad = SetClass::new(3, 11).unwrap();
    ///
    /// assert_eq!(
    ///     triad.complement(),
    ///     SetClass::new(9, 11).unwrap()
    /// );
    ///
    /// // Hexachords might not have the same index
    /// let hexachord = SetClass::new(6, 29).unwrap();
    /// assert_eq!(hexachord.complement().index(), 50); // not 29!
    /// ```
    pub fn complement(self) -> Self {
        let new_cardinality = 12 - self.cardinality();

        let index = if self.cardinality() == 6 && self.z_related().is_some() {
            self.z_related().unwrap_or(self).index()
        } else {
            self.index()
        };

        Self::new(new_cardinality, index).expect("must be a valid index and inversion")
    }

    /// Returns the interval class vector of this set class.
    ///
    /// For more information, see [`IntervalClassVector`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{IntervalClassVector, SetClass};
    /// let triad = SetClass::new(3, 11).unwrap();
    ///
    /// assert_eq!(
    ///     triad.interval_class_vector(),
    ///     IntervalClassVector::new([0, 0, 1, 1, 1, 0]).unwrap(),
    /// );
    /// ```
    #[inline]
    #[doc(alias = "icv")]
    pub fn interval_class_vector(self) -> IntervalClassVector {
        self.prime_form().interval_class_vector()
    }

    /// Returns `true` if this set class is its own inversion.
    ///
    /// Inversionally symmetric set classes have only one form; there is no distinct A or B
    /// inversion.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::SetClass;
    /// // 3-12 (augmented triad) is inversionally symmetric
    /// assert!(SetClass::new(3, 12).unwrap().is_inversionally_symmetric());
    ///
    /// // 3-11 inverts between major and minor triad
    /// assert!(!SetClass::new(3, 11).unwrap().is_inversionally_symmetric());
    /// ```
    pub fn is_inversionally_symmetric(self) -> bool {
        let prime_form = self.prime_form();

        prime_form.invert_around(PitchClass::C).normal_order() == prime_form
    }

    pub(super) const fn max_index(cardinality: u8) -> Option<u8> {
        let max = match cardinality {
            0 | 1 | 11 | 12 => 1,
            2 | 10 => 6,
            3 | 9 => 12,
            4 | 8 => 29,
            5 | 7 => 38,
            6 => 50,
            _ => return None,
        };

        Some(max)
    }
}

impl fmt::Display for SetClass {
    /// Formats the set class as a Forte number string.
    ///
    /// The format is `cardinality-index` (e.g. `"3-11"`). Z-related set classes include a
    /// `Z` prefix before the index (e.g. `"4-Z15"`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::SetClass;
    /// assert_eq!(SetClass::new(3, 11).unwrap().to_string(), "3-11");
    /// assert_eq!(SetClass::new(4, 15).unwrap().to_string(), "4-Z15");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.z_index.is_some() {
            write!(f, "{}-Z{}", self.cardinality, self.index.get())
        } else {
            write!(f, "{}-{}", self.cardinality, self.index.get())
        }
    }
}

impl Ord for SetClass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cardinality.cmp(&other.cardinality).then(self.index.cmp(&other.index))
    }
}

impl PartialOrd for SetClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<SetClassForm> for SetClass {
    /// Converts a [`SetClassForm`] into its underlying `SetClass`, discarding the inversion form.
    #[inline]
    fn from(form: SetClassForm) -> Self {
        form.set_class()
    }
}

impl From<PitchClassSet> for SetClass {
    /// Returns the set class of the given pitch class set.
    ///
    /// This is equivalent to calling [`PitchClassSet::set_class`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::PitchClass;
    /// # use music_theory::set::{PitchClassSet, SetClass};
    /// let pcset = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G
    /// ]);
    ///
    /// assert_eq!(SetClass::from(pcset).to_string(), "3-11");
    /// ```
    #[inline]
    fn from(pcset: PitchClassSet) -> Self {
        SetClassForm::from(pcset).set_class()
    }
}

#[cfg(test)]
mod tests {
    use crate::set::forte::SetClass;
    use crate::set::{IntervalClassVector, PitchClassSet};

    #[test]
    fn round_trip_icv() {
        for pcset in (0..=0xfff).map(PitchClassSet::from_bits_masked) {
            let set_class = pcset.set_class();
            let icv = pcset.interval_class_vector();

            assert_eq!(
                set_class.interval_class_vector(), icv,
                "calculating icv should be correct",
            );

            if icv == IntervalClassVector::EMPTY {
                assert_eq!(
                    SetClass::from_icv_first(icv), Some(SetClass::new(0, 1).expect("valid set class")),
                    "both 0-1 and 1-1 map to <0,0,0,0,0,0>, so this should return the first",
                );
            } else {
                let from_icv = SetClass::from_icv_first(icv);

                assert!(
                   from_icv == Some(set_class) || from_icv == set_class.z_related(),
                    "should be able to get the set class back from it, {icv}, from_icv: {from_icv:?}, original: {set_class:?}",
                );
            }
        }
    }
}
