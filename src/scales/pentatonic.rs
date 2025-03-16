use crate::interval::Interval;
use crate::scales::{ScaleModes, T};

const MIN3: Interval = Interval::MINOR_THIRD;

#[repr(u8)] // TODO: rework define scale macro to support any size
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
pub enum PentatonicModes {
    I = 1,
    II,
    III,
    IV,
    V,
}

impl ScaleModes<5> for PentatonicModes {
    const RELATIVE_INTERVALS: [Interval; 5] = [T, T, MIN3, T, MIN3];

    fn number(&self) -> u8 {
        *self as _
    }

    fn from_number(number: u8) -> Option<Self> {
        Self::from_repr(number)
    }
}

impl PentatonicModes {
    pub const MAJOR: Self = Self::I;
    pub const SUSPENDED: Self = Self::II;
    pub const BLUES_MINOR: Self = Self::III;
    pub const BLUES_MAJOR: Self = Self::IV;
    pub const MINOR: Self = Self::V;
}


