use std::fmt;
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
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
