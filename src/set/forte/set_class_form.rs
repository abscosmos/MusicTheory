use std::cmp::Ordering;
use crate::PitchClass;
use crate::set::forte::{tables, NewSetClassError, SetClass};
use crate::set::PitchClassSet;

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
    pub const fn inversion(self) -> Option<InversionForm> {
        self.inversion
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
