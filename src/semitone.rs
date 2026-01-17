use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// A signed distance in semitones.
///
/// This is equivalent to a "half-step", and can be used to represent chromatic distances or
/// spelling agnostic intervals.
///
/// # Examples
/// ```
/// # use music_theory::prelude::*;
/// let fifth = Semitone(7);
/// let octave = Semitone::OCTAVE;
///
/// assert_eq!(fifth + fifth, Semitone(14));
/// assert_eq!(octave - fifth, Semitone(5));
/// assert_eq!(Semitone(14).mod_octave(), Semitone(2));
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Semitone(pub i16);

impl Add for Semitone {
    type Output = Semitone;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Semitone {
    type Output = Semitone;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Neg for Semitone {
    type Output = Semitone;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl AddAssign for Semitone {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Semitone {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<i16> for Semitone {
    type Output = Semitone;

    fn mul(self, rhs: i16) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<i16> for Semitone {
    type Output = Semitone;

    fn div(self, rhs: i16) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl fmt::Display for Semitone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}