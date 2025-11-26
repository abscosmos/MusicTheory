use crate::chord::{Chord, InvalidInversion};
use crate::interval::Interval;
use crate::key::Key;
use crate::pitch::Pitch;
use crate::scales::heptatonic::DiatonicMode;
use strum_macros::FromRepr;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromRepr)]
pub enum ScaleDegree {
    I = 1,
    II = 2,
    III = 3,
    IV = 4,
    V = 5,
    VI = 6,
    VII = 7,
}

impl ScaleDegree {
    pub fn as_idx(self) -> u8 {
        (self as u8) - 1
    }

    pub fn from_idx(idx: u8) -> Option<Self> {
        Self::from_repr(idx + 1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
}


#[derive(Debug, thiserror::Error, Clone, Eq, PartialEq)]
#[error("Invalid inversion for chord type")]
pub struct InvalidInversionError;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RomanChord {
    pub degree: ScaleDegree,
    pub triad_quality: Quality,
    seventh_quality: Option<Quality>,
    inversion: u8,
}

impl RomanChord {
    pub fn new(
        degree: ScaleDegree,
        triad_quality: Quality,
        seventh_quality: Option<Quality>,
        inversion: u8,
    ) -> Result<Self, InvalidInversionError> {
        let max_inversion = if seventh_quality.is_some() { 3 } else { 2 };

        if inversion > max_inversion {
            return Err(InvalidInversionError);
        }

        Ok(Self {
            degree,
            triad_quality,
            seventh_quality,
            inversion,
        })
    }
}
