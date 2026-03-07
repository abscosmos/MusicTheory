use std::{cmp, fmt};
use std::cmp::Ordering;
use crate::PitchClass;
use crate::set::forte::{tables, NewSetClassError, SetClass};
use crate::set::{IntervalClassVector, PitchClassSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum InversionForm {
    A,
    B
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SetClassForm {
    set_class: SetClass,
    inversion: Option<InversionForm>,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NewSetClassFormError {
    #[error(transparent)]
    InvalidSetClass(#[from] NewSetClassError),
    #[error("Inversion (or lack thereof) invalid for set class")]
    InvalidInversion,
}

impl SetClassForm {
    const COMPLEMENT_FORM_PRESERVING: [(u8, u8); 6] = [
        (4, 12), (4, 14), (5, 11), (5, 26), (5, 28), (6, 14),
    ];

    pub fn new(cardinality: u8, index: u8, inversion: Option<InversionForm>) -> Result<Self, NewSetClassFormError> {
        let set_class = SetClass::new(cardinality, index)?;

        if inversion.is_none() != set_class.is_inversionally_symmetric() {
            return Err(NewSetClassFormError::InvalidInversion);
        }

        Ok(Self { set_class, inversion })
    }

    #[inline]
    pub const fn cardinality(self) -> u8 {
        self.set_class.cardinality
    }

    #[inline]
    pub const fn index(self) -> u8 {
        self.set_class.index.get()
    }

    #[inline]
    pub const fn inversion_form(self) -> Option<InversionForm> {
        self.inversion
    }

    pub const fn inversion(self) -> Self {
        let inv = match self.inversion {
            None => None,
            Some(InversionForm::A) => Some(InversionForm::B),
            Some(InversionForm::B) => Some(InversionForm::A),
        };

        Self { set_class: self.set_class, inversion: inv }
    }

    #[inline]
    pub const fn z_related(self) -> Option<Self> {
        let Some(set_class) = self.set_class.z_related() else {
            return None;
        };

        Some(Self { set_class, inversion: self.inversion })
    }

    #[inline]
    pub const fn prime_form(self) -> PitchClassSet {
        self.set_class.prime_form()
    }

    pub fn normal_form(self) -> PitchClassSet {
        if self.inversion.is_none_or(|inv| inv == InversionForm::A) {
            self.prime_form()
        } else {
            self.prime_form()
                .invert_around(PitchClass::C)
                .normal_order()
        }
    }

    pub fn complement(self) -> Self {
        let new_cardinality = 12 - self.cardinality();

        if self.cardinality() == 6 && self.z_related().is_some() {
            let pc_complement = self.normal_form().complement();

            Self::from(pc_complement)
        } else {
            let check_form = (cmp::min(self.cardinality(), new_cardinality), self.index());

            let inv = if self.is_inversionally_symmetric()
                || Self::COMPLEMENT_FORM_PRESERVING.contains(&check_form)
            {
                self.inversion
            } else {
                self.inversion().inversion
            };

            Self::new(new_cardinality, self.index(), inv).expect("must be a valid index and inversion")
        }
    }

    #[inline]
    #[doc(alias = "icv")]
    pub fn interval_class_vector(self) -> IntervalClassVector {
        self.prime_form().interval_class_vector()
    }

    pub fn is_inversionally_symmetric(self) -> bool {
        self.inversion.is_none()
    }

    #[inline]
    pub const fn set_class(self) -> SetClass {
        self.set_class
    }
}

impl From<PitchClassSet> for SetClassForm {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for SetClassForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inv) = self.inversion {
            write!(f, "{}{}", self.set_class, inv)
        } else {
            fmt::Display::fmt(&self.set_class, f)
        }
    }
}

impl From<SetClass> for SetClassForm {
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