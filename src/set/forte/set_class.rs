use std::fmt;
use std::num::NonZeroU8;
use crate::PitchClass;
use crate::set::forte::set_class_form::SetClassForm;
use crate::set::{IntervalClassVector, PitchClassSet};
use super::tables;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SetClass {
    // TODO: compress this into a u16 eventually
    pub(super) cardinality: u8,
    pub(super) index: NonZeroU8,
    pub(super) z_index: Option<NonZeroU8>,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NewSetClassError {
    #[error("Cardinality must be in [1, 12]")]
    InvalidCardinality,
    #[error("Index {index} out of range for SetClass with cardinality {cardinality}. Expected [1, {max}]")]
    InvalidIndex {
        cardinality: u8,
        index: u8,
        max: u8,
    },
}

impl SetClass {
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

    #[inline]
    pub const fn cardinality(self) -> u8 {
        self.cardinality
    }

    #[inline]
    pub const fn index(self) -> u8 {
        self.index.get()
    }

    #[inline]
    pub const fn z_related(self) -> Option<Self> {
        // FIXME(const)
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

    #[inline]
    pub const fn prime_form(&self) -> PitchClassSet {
        tables::lookup(self.cardinality, self.index.get()).prime_form
    }

    #[inline]
    pub fn interval_class_vector(self) -> IntervalClassVector {
        self.prime_form().interval_class_vector()
    }

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.z_index.is_some() {
            write!(f, "{}-Z{}", self.cardinality, self.index.get())
        } else {
            write!(f, "{}-{}", self.cardinality, self.index.get())
        }
    }
}

impl From<SetClassForm> for SetClass {
    #[inline]
    fn from(form: SetClassForm) -> Self {
        form.set_class()
    }
}

impl From<PitchClassSet> for SetClass {
    #[inline]
    fn from(pcset: PitchClassSet) -> Self {
        SetClassForm::from(pcset).set_class()
    }
}