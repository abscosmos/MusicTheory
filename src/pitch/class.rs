use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};
use crate::enharmonic::EnharmonicEq;
use crate::interval::Interval;
use crate::pitch::{Pitch, Letter, AccidentalSign, PitchFromStrError, Spelling};
use crate::semitone::Semitone;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, FromRepr, EnumIter, Ord, PartialOrd, Serialize, Deserialize)]
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
    pub fn transpose(&self, interval: Interval) -> Self {
        *self + interval.semitones()
    }

    pub fn letter(self) -> Letter {
        use PitchClass as PC;
        use Letter as L;

        match self {
            PC::C | PC::Cs => L::C,
            PC::D | PC::Ds => L::D,
            PC::E => L::E,
            PC::F | PC::Fs => L::F,
            PC::G | PC::Gs => L::G,
            PC::A | PC::As => L::A,
            PC::B => L::B,
        }
    }

    pub fn accidental(self) -> AccidentalSign {
        use PitchClass as PC;

        match self {
            PC::Cs | PC::Ds | PC::Fs | PC::Gs | PC::As => AccidentalSign::SHARP,
            _ => AccidentalSign::NATURAL,
        }
    }

    #[inline(always)]
    pub const fn from_chroma(chroma: u8) -> Option<Self> {
        Self::from_repr(chroma)
    }

    pub fn chroma(self) -> u8 {
        self as u8
    }

    pub fn spell_with(self, spelling: Spelling) -> Pitch {
        if self.accidental() == AccidentalSign::NATURAL || spelling.is_sharps() {
            self.into()
        } else {
            let base = Self::from_chroma(self as u8 + 1)
                .expect("must be <= 11")
                .letter();

            Pitch::from_letter_and_accidental(base, AccidentalSign::FLAT)
        }
    }
}

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

        Self::from_chroma(pitch)
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

impl Add<Interval> for PitchClass {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Interval> for PitchClass {
    type Output = Self;

    fn sub(self, rhs: Interval) -> Self::Output {
        self + (-rhs)
    }
}