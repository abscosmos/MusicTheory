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
/// assert_eq!(Semitone(14).normalize(), Semitone(2));
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Semitone(pub i16);

impl Semitone {
    /// Distance of a unison, 0 semitones.
    pub const UNISON: Self = Self(0);

    /// Distance of an octave, 12 semitones.
    pub const OCTAVE: Self = Self(12);

    /// Normalizes the semitone value to fit within a single octave, in `[0, 11]`.
    ///
    /// This uses Euclidean modulo, so negative values wrap. This can be though about adding or
    /// subtracting multiples of [an octave](Self::OCTAVE) until in the range `[0,11]`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Semitone(14).normalize(), Semitone(2));
    /// assert_eq!(Semitone(12).normalize(), Semitone(0));
    ///
    /// // Negative values wrap; in this case, -1 is 11 semitones minus an octave
    /// assert_eq!(Semitone(-1).normalize(), Semitone(11));
    /// assert_eq!(Semitone(-13).normalize(), Semitone(11));
    /// ```
    pub fn normalize(self) -> Self {
        Self(self.0.rem_euclid(12))
    }

    /// Returns `true` if the semitone value is positive (ascending interval).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert!(Semitone(5).is_positive());
    /// assert!(!Semitone(0).is_positive());
    /// assert!(!Semitone(-3).is_positive());
    /// ```
    pub fn is_positive(self) -> bool {
        self.0 > 0
    }

    /// Returns `true` if the semitone value is negative (descending interval).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert!(Semitone(-3).is_negative());
    /// assert!(!Semitone(0).is_negative());
    /// assert!(!Semitone(5).is_negative());
    /// ```
    pub fn is_negative(self) -> bool {
        self.0 < 0
    }
}

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