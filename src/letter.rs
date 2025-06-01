use std::fmt;
use std::str::FromStr;
use strum_macros::{EnumIter, FromRepr};
use crate::key::Key;
use crate::pitch::Pitch;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter, FromRepr, Ord, PartialOrd)]
pub enum Letter {
    C = 0,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Letter {
    pub const fn step(&self) -> u8 {
        *self as _
    }
    
    pub const fn from_step(step: u8) -> Option<Self> {
        Self::from_repr(step)
    }
    
    pub fn offset_between(&self, rhs: Self) -> u8 {
        (rhs.step() as i16 - self.step() as i16).rem_euclid(7) as _
    }

    pub fn to_pitch_in_key(self, key: Key) -> Pitch {
        Pitch::from_letter_and_accidental(self, key.accidental_of(self))
    }

    pub(crate) fn fifths_from_c(self) -> i16 {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => -1,
            Self::G => 1,
            Self::A => 3,
            Self::B => 5,
        }
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Letter must be A, B, C, D, E, F, or G")]
pub struct InvalidLetter;

impl FromStr for Letter {
    type Err = InvalidLetter;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" | "c" => Ok(Self::C),
            "D" | "d" => Ok(Self::D),
            "E" | "e" => Ok(Self::E),
            "F" | "f" => Ok(Self::F),
            "G" | "g" => Ok(Self::G),
            "A" | "a" => Ok(Self::A),
            "B" | "b" => Ok(Self::B),
            _ => Err(InvalidLetter),
        }
    }
}
