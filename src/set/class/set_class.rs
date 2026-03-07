use std::cmp::Ordering;
use std::fmt;
use std::num::NonZeroU8;
use crate::PitchClass;
use crate::set::class::set_class_form::SetClassForm;
use crate::set::{check_triangular, IntervalClassVector, PitchClassSet};
use super::tables;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[doc(alias = "Forte")]
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

    pub fn from_icv_first(icv: IntervalClassVector) -> Option<Self> {
        let total = icv.total();

        let cardinality = if total != 0 {
            check_triangular(total as _)? as u8 + 1
        } else {
            0
        };

        let entry = tables::lookup_entries_cardinality(cardinality)
            .into_iter()
            .find(|e| e.prime_form.interval_class_vector() == icv)?;

        let set_class = Self {
            cardinality: entry.cardinality,
            index: entry.index,
            z_index: entry.z_related,
        };

        Some(set_class)
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

    #[inline]
    pub const fn prime_form(self) -> PitchClassSet {
        tables::lookup(self.cardinality, self.index.get()).prime_form
    }

    pub fn complement(self) -> Self {
        SetClassForm::from(self).complement().set_class()
    }

    #[inline]
    #[doc(alias = "icv")]
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

#[cfg(test)]
mod tests {
    use crate::set::class::SetClass;
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