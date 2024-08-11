use std::fmt;
use std::ops::{Add, Sub};
use strum_macros::EnumIter;
use crate::enharmonic::EnharmonicEq;
use crate::semitone::Semitone;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
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

}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
#[error("Given value wasn't in range [0,11]")]
pub struct InvalidPitch;

impl TryFrom<u8> for PitchClass {
    type Error = InvalidPitch;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PitchClass::C),
            1 => Ok(PitchClass::Cs),
            2 => Ok(PitchClass::D),
            3 => Ok(PitchClass::Ds),
            4 => Ok(PitchClass::E),
            5 => Ok(PitchClass::F),
            6 => Ok(PitchClass::Fs),
            7 => Ok(PitchClass::G),
            8 => Ok(PitchClass::Gs),
            9 => Ok(PitchClass::A),
            10 => Ok(PitchClass::As),
            11 => Ok(PitchClass::B),
            _ => Err(InvalidPitch)
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
        let pitch: u8 = (self as u8 as i16 + rhs.0)
            .rem_euclid(12)
            .try_into()
            .expect("must be between [0,11] since did % 12");

        pitch.try_into()
            .expect("must be between [0,11] since did % 12")
    }
}

impl Sub<Semitone> for PitchClass {
    type Output = PitchClass;

    fn sub(self, rhs: Semitone) -> Self::Output {
        self + (-rhs)
    }
}