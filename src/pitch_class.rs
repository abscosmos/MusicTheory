use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use strum_macros::{EnumIter, FromRepr};
use crate::accidental::AccidentalSign;
use crate::enharmonic::EnharmonicEq;
use crate::interval::Interval;
use crate::letter::Letter;
use crate::pitch::{Pitch, PitchFromStrError};
use crate::semitone::Semitone;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, FromRepr, EnumIter, Ord, PartialOrd)]
pub enum PitchClass {
    C = 0,  /* C /B# */
    Cs, /* C#/D♭ */
    D,
    Ds, /* D#/E♭ */
    E,  /* E /F♭ */
    F,  /* F /E♭ */
    Fs, /* F#/G♭ */
    G,
    Gs, /* G#/A♭ */
    A,
    As, /* A#/B♭ */
    B,  /* B /C♭ */
}

impl PitchClass {
    pub fn transpose(&self, interval: &Interval, ascending: bool) -> Self {
        let offset = if ascending {
            interval.semitones()
        } else {
            -interval.semitones()
        };

        *self + offset
    }

    pub fn transpose_ascending(&self, interval: &Interval) -> Self {
        self.transpose(interval, true)
    }

    pub fn transpose_descending(&self, interval: &Interval) -> Self {
        self.transpose(interval, false)
    }

    pub fn letter(&self) -> Letter {
        use PitchClass as PC;
        use Letter as L;

        match *self {
            PC::C | PC::Cs => L::C,
            PC::D | PC::Ds => L::D,
            PC::E => L::E,
            PC::F | PC::Fs => L::F,
            PC::G | PC::Gs => L::G,
            PC::A | PC::As => L::A,
            PC::B => L::B,
        }
    }

    pub fn accidental(&self) -> AccidentalSign {
        use PitchClass as PC;

        match *self {
            PC::Cs | PC::Ds | PC::Fs | PC::Gs | PC::As => AccidentalSign::SHARP,
            _ => AccidentalSign::NATURAL,
        }
    }

    pub fn chroma(&self) -> u8 {
        *self as u8
    }

    // TODO: better name?
    pub fn bias(&self, sharp: bool) -> Pitch {
        if self.accidental() == AccidentalSign::NATURAL || sharp {
            (*self).into()
        } else {
            let base = Self::from_repr(*self as u8 + 1)
                .expect("must be <= 11")
                .letter();

            Pitch::from_letter_and_accidental(base, AccidentalSign::FLAT)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
#[error("Given value wasn't in range [0,11]")]
pub struct InvalidPitch;

// impl TryFrom<u8> for PitchClass {
//     type Error = InvalidPitch;
//
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(PitchClass::C),
//             1 => Ok(PitchClass::Cs),
//             2 => Ok(PitchClass::D),
//             3 => Ok(PitchClass::Ds),
//             4 => Ok(PitchClass::E),
//             5 => Ok(PitchClass::F),
//             6 => Ok(PitchClass::Fs),
//             7 => Ok(PitchClass::G),
//             8 => Ok(PitchClass::Gs),
//             9 => Ok(PitchClass::A),
//             10 => Ok(PitchClass::As),
//             11 => Ok(PitchClass::B),
//             _ => Err(InvalidPitch)
//         }
//     }
// }

impl EnharmonicEq for PitchClass {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl Add<Semitone> for PitchClass {
    type Output = PitchClass;

    fn add(self, rhs: Semitone) -> Self::Output {
        let pitch = (self as u8 as i16 + rhs.0)
            .rem_euclid(12)
            .try_into()
            .expect("must be between [0,11] since did % 12");

        PitchClass::from_repr(pitch)
            .expect("must be between [0,11] since did % 12")
    }
}

impl Sub<Semitone> for PitchClass {
    type Output = PitchClass;

    fn sub(self, rhs: Semitone) -> Self::Output {
        self + (-rhs)
    }
}

impl FromStr for PitchClass {
    type Err = PitchFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pitch::from_str(s)?.into())
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pitch = Pitch::from(*self);
        write!(f, "{pitch}")
    }
}