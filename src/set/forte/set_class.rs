use std::num::NonZeroU8;
use super::tables;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SetClass {
    // TODO: compress this into a u16 eventually
    cardinality: u8,
    index: NonZeroU8,
    z_index: Option<NonZeroU8>,
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

    pub(super) const fn max_index(cardinality: u8) -> Option<u8> {
        let max = match cardinality {
            0 | 1 | 11 | 12 => 1,
            2 | 10 => 6,
            3 | 9=> 12,
            4 | 8 => 29,
            5 | 7 => 38,
            6 => 50,
            _ => return None,
        };

        Some(max)
    }
}