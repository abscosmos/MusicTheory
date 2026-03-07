use std::{cmp, fmt};
use std::cmp::Ordering;
use crate::PitchClass;
use crate::set::forte::{tables, NewSetClassError, SetClass};
use crate::set::{IntervalClassVector, PitchClassSet};

/// The inversion form of a [`SetClassForm`]: prime (A) or inverted (B).
///
/// Set classes that are not [inversionally symmetric](SetClass::is_inversionally_symmetric)
/// have two distinct forms. By convention, the prime form is labeled `A` and the inversion
/// is labeled `B`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum InversionForm {
    /// The prime inversion form.
    A,
    /// The inverted form.
    B,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
/// A pitch class set equivalence class under transposition (Tn), with inversion forms distinguished.
///
/// `SetClassForm` extends [`SetClass`] by tracking whether a set is in its prime (A) or
/// inverted (B) form, written as `cardinality-indexA` or `cardinality-indexB`, like `3-11A`.
/// This corresponds to the Tn classification in Forte's numbering system, as opposed to
/// the TnI classification of [`SetClass`].
///
/// Inversionally symmetric set classes have no suffix (e.g. `"3-12"`).
///
/// # Examples
///
/// ```
/// # use music_theory::PitchClass;
/// # use music_theory::set::{PitchClassSet, SetClassForm};
/// use PitchClass as PC;
///
/// // Major and minor triads have the same set class but inversion forms
/// let minor = PitchClassSet::from_iter([PC::C, PC::Ds, PC::G]);
/// let major = PitchClassSet::from_iter([PC::C, PC::E, PC::G]);
///
/// assert_eq!(minor.set_class_form().to_string(), "3-11A");
/// assert_eq!(major.set_class_form().to_string(), "3-11B");
/// ```
#[doc(alias = "Forte")]
pub struct SetClassForm {
    set_class: SetClass,
    inversion: Option<InversionForm>,
}

/// Error returned by [`SetClassForm::new`] when construction fails.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NewSetClassFormError {
    /// The underlying [`SetClass`] could not be constructed.
    #[error(transparent)]
    InvalidSetClass(#[from] NewSetClassError),
    /// The inversion form is incompatible with the set class.
    ///
    /// Inversionally symmetric set classes require `inversion: None`,
    /// while non-symmetric set classes require `inversion: Some(_)`.
    #[error("Inversion (or lack thereof) invalid for set class")]
    InvalidInversion,
}

impl SetClassForm {
    /// Set classes whose complement preserves the inversion form; used in [`Self::complement`].
    const COMPLEMENT_FORM_PRESERVING: [(u8, u8); 8] = [
        (4, 12), (4, 14), (5, 11), (5, 26), (5, 28), (6, 14), (6, 10), (6, 39)
    ];

    /// Creates a `SetClassForm` from a cardinality, index, and optional inversion form.
    ///
    /// # Errors
    /// Returns [`InvalidSetClass`](NewSetClassFormError::InvalidSetClass) if the cardinality
    /// or index is invalid, or [`InvalidInversion`](NewSetClassFormError::InvalidInversion)
    /// if an inversionally-symmetric set was provided an inversion, or if a non-symmetric set was
    /// missing an inversion.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{forte::NewSetClassFormError, InversionForm, SetClassForm};
    /// // Minor triad is 3-11A
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    /// assert_eq!(minor.to_string(), "3-11A");
    ///
    /// // Inversionally symmetric sets take None
    /// let aug = SetClassForm::new(3, 12, None).unwrap();
    /// assert_eq!(aug.to_string(), "3-12");
    ///
    /// // Wrong inversion for a symmetric set class returns an error
    /// assert_eq!(
    ///     SetClassForm::new(3, 12, Some(InversionForm::A)),
    ///     Err(NewSetClassFormError::InvalidInversion),
    /// );
    /// ```
    pub fn new(cardinality: u8, index: u8, inversion: Option<InversionForm>) -> Result<Self, NewSetClassFormError> {
        let set_class = SetClass::new(cardinality, index)?;

        Self::with_set_class(set_class, inversion).ok_or(NewSetClassFormError::InvalidInversion)
    }

    /// Creates a `SetClassForm` from an existing [`SetClass`] and optional inversion form.
    ///
    /// Returns `None` if the inversion form is incompatible with the set class.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClass, SetClassForm};
    /// let triad = SetClass::new(3, 11).unwrap();
    ///
    /// let minor = SetClassForm::with_set_class(triad, Some(InversionForm::A)).unwrap();
    /// assert_eq!(minor.to_string(), "3-11A");
    ///
    /// // Incompatible inversion returns None
    /// let aug = SetClass::new(3, 12).unwrap();
    /// assert!(SetClassForm::with_set_class(aug, Some(InversionForm::B)).is_none());
    /// ```
    pub fn with_set_class(set_class: SetClass, inversion: Option<InversionForm>) -> Option<Self> {
        if inversion.is_none() == set_class.is_inversionally_symmetric() {
            Some(Self { set_class, inversion })
        } else {
            None
        }
    }

    /// Returns the set class form's cardinality, the number of distinct pitch classes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClassForm};
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    /// assert_eq!(minor.cardinality(), 3);
    /// ```
    #[inline]
    pub const fn cardinality(self) -> u8 {
        self.set_class.cardinality
    }

    /// Returns the Forte index of this set class form within its cardinality.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClassForm};
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    ///
    /// // 3-11 is the 11th set class of cardinality 3
    /// assert_eq!(minor.index(), 11);
    /// ```
    #[inline]
    pub const fn index(self) -> u8 {
        self.set_class.index.get()
    }

    /// Returns the inversion form of this set class form.
    ///
    /// Returns `None` for [inversionally symmetric](SetClass::is_inversionally_symmetric)
    /// set classes, `Some(InversionForm::A)` for prime forms, and `Some(InversionForm::B)`
    /// for inverted forms.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClassForm};
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    /// assert_eq!(minor.inversion_form(), Some(InversionForm::A));
    ///
    /// // Inversionally symmetric set classes have no inversion form
    /// let aug = SetClassForm::new(3, 12, None).unwrap();
    /// assert_eq!(aug.inversion_form(), None);
    /// ```
    #[inline]
    pub const fn inversion_form(self) -> Option<InversionForm> {
        self.inversion
    }

    /// Returns this set class form with the inversion flipped.
    ///
    /// For [inversionally symmetric](SetClass::is_inversionally_symmetric) set classes,
    /// this returns an identical value since there is no inversion to flip.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClassForm};
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    /// let major = minor.inversion();
    ///
    /// assert_eq!(major.inversion_form(), Some(InversionForm::B));
    /// assert_eq!(major.inversion(), minor);
    /// ```
    pub const fn inversion(self) -> Self {
        let inv = match self.inversion {
            None => None,
            Some(InversionForm::A) => Some(InversionForm::B),
            Some(InversionForm::B) => Some(InversionForm::A),
        };

        Self { set_class: self.set_class, inversion: inv }
    }

    /// Returns the Z-related set class form, if one exists.
    ///
    /// Z-related set classes have identical [interval class vectors](IntervalClassVector)
    /// but are not related by transposition or inversion. The current inversion form is
    /// preserved in the result.
    ///
    /// Returns `None` if this set class has no Z-relation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClassForm};
    /// // 6-Z3 and 6-Z36 are Z-related
    /// let z3a = SetClassForm::new(6, 3, Some(InversionForm::A)).unwrap();
    /// let z36a = z3a.z_related().unwrap();
    ///
    /// assert_eq!(z36a.to_string(), "6-Z36A");
    /// assert_eq!(z3a.interval_class_vector(), z36a.interval_class_vector());
    ///
    /// // 3-11 has no Z-relation
    /// let  set_3_11 = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    /// assert!(set_3_11.z_related().is_none());
    /// ```
    #[inline]
    pub const fn z_related(self) -> Option<Self> {
        let Some(set_class) = self.set_class.z_related() else {
            return None;
        };

        Some(Self { set_class, inversion: self.inversion })
    }

    /// Returns the prime form of this set class as a [`PitchClassSet`].
    ///
    /// Always returns the prime (A) form regardless of the current [inversion form](InversionForm).
    /// To get the pitch class set corresponding to the current form, use [`normal_form`](Self::normal_form).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::PitchClass;
    /// # use music_theory::set::{InversionForm, PitchClassSet, SetClassForm};
    /// // Prime form of 3-11 is [0, 3, 7] regardless of inversion form
    /// let major = SetClassForm::new(3, 11, Some(InversionForm::B)).unwrap();
    ///
    /// assert_eq!(
    ///     major.prime_form(),
    ///     PitchClassSet::from_iter([
    ///         PitchClass::C,
    ///         PitchClass::Ds,
    ///         PitchClass::G,
    ///     ]),
    /// );
    /// ```
    #[inline]
    pub const fn prime_form(self) -> PitchClassSet {
        self.set_class.prime_form()
    }

    /// Returns the normal form of this set class as a [`PitchClassSet`], respecting the inversion form.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::PitchClass;
    /// # use music_theory::set::{InversionForm, PitchClassSet, SetClassForm};
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    ///
    /// assert_eq!(
    ///     minor.normal_form(),
    ///     PitchClassSet::from_iter([
    ///         PitchClass::C,
    ///         PitchClass::Ds,
    ///         PitchClass::G
    ///     ]),
    /// );
    ///
    /// let major = SetClassForm::new(3, 11, Some(InversionForm::B)).unwrap();
    ///
    /// assert_eq!(
    ///     major.normal_form(),
    ///     PitchClassSet::from_iter([
    ///         PitchClass::C,
    ///         PitchClass::E,
    ///         PitchClass::G
    ///     ]),
    /// );
    /// ```
    pub fn normal_form(self) -> PitchClassSet {
        if self.inversion.is_none_or(|inv| inv == InversionForm::A) {
            self.prime_form()
        } else {
            self.prime_form()
                .invert_around(PitchClass::C)
                .normal_order()
        }
    }

    /// Returns the complement set class form.
    ///
    /// The complement set class form is the set class form of the complement of this form's
    /// [normal form](Self::normal_form). The complement of a cardinality-`n` set class form
    /// has cardinality `12 - n`. Except for some cardinality-6 set class forms, it has the
    /// same index.
    ///
    /// The inversion form of the complement may or may not be the same as the original set class's.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClassForm};
    /// // 3-11A complements to 9-11B
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    ///
    /// assert_eq!(minor.complement().to_string(), "9-11B");
    /// assert_eq!(minor.complement().complement(), minor);
    ///
    /// // Hexachords might not have the same index
    /// let hexachord = SetClassForm::new(6, 39, Some(InversionForm::B)).unwrap();
    /// let complement = hexachord.complement();
    /// assert_eq!(complement.index(), 10); // not 39!
    /// // The inversion form may or may not be the same
    /// assert_eq!(complement.inversion_form(), Some(InversionForm::B));
    /// ```
    pub fn complement(self) -> Self {
        let mut base = Self::from(self.set_class.complement());

        let check_form = (cmp::min(self.cardinality(), base.cardinality()), self.index());

        if self.is_inversionally_symmetric() || Self::COMPLEMENT_FORM_PRESERVING.contains(&check_form) {
            base.inversion = self.inversion;
        } else {
            base.inversion = self.inversion().inversion;
        }

        base
    }

    /// Returns the interval class vector of this set class form.
    ///
    /// For more information, see [`IntervalClassVector`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{IntervalClassVector, InversionForm, SetClassForm};
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    ///
    /// assert_eq!(
    ///     minor.interval_class_vector(),
    ///     IntervalClassVector::new([0, 0, 1, 1, 1, 0]).unwrap(),
    /// );
    /// ```
    #[inline]
    #[doc(alias = "icv")]
    pub fn interval_class_vector(self) -> IntervalClassVector {
        self.prime_form().interval_class_vector()
    }

    /// Returns `true` if this set class form has no distinct inversion.
    ///
    /// Inversionally symmetric set class forms have only one form; there is no distinct A or B
    /// inversion.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClassForm};
    /// // 3-12 (augmented triad) is inversionally symmetric
    /// let aug_triad = SetClassForm::new(3, 12, None).unwrap();
    /// assert!(aug_triad.is_inversionally_symmetric());
    ///
    /// // 3-11 inverts between major and minor triad
    /// let triad = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    /// assert!(!triad.is_inversionally_symmetric());
    /// ```
    pub fn is_inversionally_symmetric(self) -> bool {
        self.inversion.is_none()
    }

    /// Returns the underlying [`SetClass`], discarding the inversion form.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClass, SetClassForm};
    /// let minor = SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap();
    /// let major = SetClassForm::new(3, 11, Some(InversionForm::B)).unwrap();
    ///
    /// // Both share the same underlying set class
    /// assert_eq!(minor.set_class(), major.set_class());
    /// assert_eq!(minor.set_class(), SetClass::new(3, 11).unwrap());
    /// ```
    #[inline]
    pub const fn set_class(self) -> SetClass {
        self.set_class
    }
}

impl From<PitchClassSet> for SetClassForm {
    /// Returns the set class form of the given pitch class set.
    ///
    /// This is equivalent to calling [`PitchClassSet::set_class_form`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::PitchClass;
    /// # use music_theory::set::{PitchClassSet, SetClassForm};
    /// let c_minor = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::Ds,
    ///     PitchClass::G,
    /// ]);
    ///
    /// assert_eq!(SetClassForm::from(c_minor).to_string(), "3-11A");
    /// ```
    fn from(pcset: PitchClassSet) -> Self {
        use InversionForm as Form;

        let norm = pcset.normal_order();
        let inv_norm = norm.invert_around(PitchClass::C).normal_order();

        let (inversion, prime) = match norm.cmp_lexicographically(inv_norm) {
            Ordering::Less => (Some(Form::A), norm),
            Ordering::Equal => (None, norm),
            Ordering::Greater => (Some(Form::B), inv_norm),
        };

        let entry = tables::lookup_prime_form(prime).expect("must be prime form");

        Self {
            set_class: SetClass {
                cardinality: entry.cardinality,
                index: entry.index,
                z_index: entry.z_related,
            },
            inversion,
        }
    }
}

impl fmt::Display for InversionForm {
    /// Formats the inversion form as `"A"` or `"B"`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for SetClassForm {
    /// Formats the set class form as a Forte number string with inversion suffix.
    ///
    /// The format is `cardinality-index` followed by `A` or `B` for the inversion form,
    /// like `"3-11A"` or `"3-11B"`. Z-related set classes include a `Z` prefix before the
    /// index, like `"6-Z3A"`. Inversionally symmetric set classes have no suffix, like `"3-12"`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClassForm};
    /// assert_eq!(SetClassForm::new(3, 11, Some(InversionForm::A)).unwrap().to_string(), "3-11A");
    /// assert_eq!(SetClassForm::new(3, 11, Some(InversionForm::B)).unwrap().to_string(), "3-11B");
    /// assert_eq!(SetClassForm::new(3, 12, None).unwrap().to_string(), "3-12");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inv) = self.inversion {
            write!(f, "{}{}", self.set_class, inv)
        } else {
            fmt::Display::fmt(&self.set_class, f)
        }
    }
}

impl From<SetClass> for SetClassForm {
    /// Converts a [`SetClass`] into a `SetClassForm`, defaulting to form A for non-symmetric set classes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::set::{InversionForm, SetClass, SetClassForm};
    /// // Non-symmetric set classes default to form A
    /// let form = SetClassForm::from(SetClass::new(3, 11).unwrap());
    /// assert_eq!(form.inversion_form(), Some(InversionForm::A));
    ///
    /// // Symmetric set classes have no inversion form
    /// let form = SetClassForm::from(SetClass::new(3, 12).unwrap());
    /// assert_eq!(form.inversion_form(), None);
    /// ```
    fn from(set_class: SetClass) -> Self {
        if set_class.is_inversionally_symmetric() {
            Self { set_class, inversion: None }
        } else {
            Self { set_class, inversion: Some(InversionForm::A) }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::set::forte::SetClassForm;
    use crate::set::PitchClassSet;

    #[test]
    fn complement() {
        for pcset in (0..=0xfff).map(PitchClassSet::from_bits_masked) {
            let set_class = SetClassForm::from(pcset);

            let complement = set_class.complement();
            let of_complement = SetClassForm::from(pcset.complement());

            assert_eq!(
                complement, of_complement,
                "complement should match set class form from complement of pcset; {set_class}",
            );

            assert_eq!(
                complement.complement(), set_class,
                "complement of complement should be original set class form"
            );
        }
    }
}
