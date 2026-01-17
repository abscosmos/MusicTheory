use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A signed distance in semitones.
///
/// This is equivalent to a "half-step", and can be used to represent chromatic distances or
/// spelling agnostic intervals.
///
/// # Examples
/// ```
/// # use music_theory::prelude::*;
/// let fifth = Semitones(7);
/// let octave = Semitones::OCTAVE;
///
/// assert_eq!(fifth + fifth, Semitones(14));
/// assert_eq!(octave - fifth, Semitones(5));
/// assert_eq!(Semitones(14).normalize(), Semitones(2));
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Semitones(pub i16);

impl Semitones {
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
    /// assert_eq!(Semitones(14).normalize(), Semitones(2));
    /// assert_eq!(Semitones(12).normalize(), Semitones(0));
    ///
    /// // Negative values wrap; in this case, -1 is 11 semitones minus an octave
    /// assert_eq!(Semitones(-1).normalize(), Semitones(11));
    /// assert_eq!(Semitones(-13).normalize(), Semitones(11));
    /// ```
    pub fn normalize(self) -> Self {
        Self(self.0.rem_euclid(12))
    }

    /// Returns `true` if the semitone value is positive (ascending interval).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert!(Semitones(5).is_positive());
    /// assert!(!Semitones(0).is_positive());
    /// assert!(!Semitones(-3).is_positive());
    /// ```
    pub fn is_positive(self) -> bool {
        self.0 > 0
    }

    /// Returns `true` if the semitone value is negative (descending interval).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert!(Semitones(-3).is_negative());
    /// assert!(!Semitones(0).is_negative());
    /// assert!(!Semitones(5).is_negative());
    /// ```
    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    /// Returns the number of full octaves this semitone value spans.
    ///
    /// For negative semitone amounts, returns negative amount of octaves.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Semitones(0).octaves(), 0);
    /// assert_eq!(Semitones(11).octaves(), 0);
    /// assert_eq!(Semitones(12).octaves(), 1);
    /// assert_eq!(Semitones(25).octaves(), 2);
    /// assert_eq!(Semitones(-1).octaves(), -1);
    /// assert_eq!(Semitones(-13).octaves(), -2);
    /// ```
    pub fn octaves(self) -> i16 {
        self.0.div_euclid(12)
    }

    /// Returns the absolute value of semitone distance.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Semitones(5).abs(), Semitones(5));
    /// assert_eq!(Semitones(-5).abs(), Semitones(5));
    /// assert_eq!(Semitones(0).abs(), Semitones(0));
    /// ```
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl Add for Semitones {
    type Output = Semitones;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Semitones {
    type Output = Semitones;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Neg for Semitones {
    type Output = Semitones;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl AddAssign for Semitones {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Semitones {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<i16> for Semitones {
    type Output = Semitones;

    fn mul(self, rhs: i16) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<i16> for Semitones {
    type Output = Semitones;

    fn div(self, rhs: i16) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Mul<Semitones> for i16 {
    type Output = Semitones;

    fn mul(self, rhs: Semitones) -> Self::Output {
        Semitones(self * rhs.0)
    }
}

impl MulAssign<i16> for Semitones {
    fn mul_assign(&mut self, rhs: i16) {
        self.0 *= rhs;
    }
}

impl DivAssign<i16> for Semitones {
    fn div_assign(&mut self, rhs: i16) {
        self.0 /= rhs;
    }
}

impl fmt::Display for Semitones {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants() {
        assert_eq!(Semitones::UNISON, Semitones(0));
        assert_eq!(Semitones::OCTAVE, Semitones(12));
    }

    #[test]
    fn arithmetic() {
        assert_eq!(Semitones(5) + Semitones(3), Semitones(8));
        assert_eq!(Semitones(5) - Semitones(3), Semitones(2));
        assert_eq!(-Semitones(5), Semitones(-5));
        assert_eq!(Semitones(5) * 3, Semitones(15));
        assert_eq!(3 * Semitones(5), Semitones(15));
        assert_eq!(Semitones(15) / 3, Semitones(5));

        let mut s = Semitones(10);
        s += Semitones(5);
        assert_eq!(s, Semitones(15));
        s -= Semitones(3);
        assert_eq!(s, Semitones(12));
        s *= 2;
        assert_eq!(s, Semitones(24));
        s /= 4;
        assert_eq!(s, Semitones(6));
    }

    #[test]
    fn normalize() {
        assert_eq!(Semitones(0).normalize(), Semitones(0));
        assert_eq!(Semitones(12).normalize(), Semitones(0));
        assert_eq!(Semitones(14).normalize(), Semitones(2));
        assert_eq!(Semitones(-1).normalize(), Semitones(11));
    }

    #[test]
    fn octaves() {
        assert_eq!(Semitones(0).octaves(), 0);
        assert_eq!(Semitones(11).octaves(), 0);
        assert_eq!(Semitones(12).octaves(), 1);
        assert_eq!(Semitones(-1).octaves(), -1);
    }

    #[test]
    fn abs() {
        assert_eq!(Semitones(5).abs(), Semitones(5));
        assert_eq!(Semitones(-5).abs(), Semitones(5));
    }

    #[test]
    fn signs() {
        assert!(Semitones(1).is_positive());
        assert!(!Semitones(0).is_positive());
        assert!(Semitones(-1).is_negative());
        assert!(!Semitones(0).is_negative());
    }

    #[test]
    fn ordering() {
        assert!(Semitones(5) > Semitones(3));
        assert!(Semitones(-1) < Semitones(0));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Semitones(7)), "7");
        assert_eq!(format!("{}", Semitones(-3)), "-3");
    }

    #[test]
    fn normalize_within_octave() {
        for semis in (-100..=100).map(Semitones) {
            let norm = semis.normalize();

            assert!(
                Semitones::UNISON <= norm && norm < Semitones::OCTAVE,
                "normalizing should be within an octave, failed: {semis:?}, norm: {norm}",
            );

            assert_eq!(
                norm.octaves(), 0,
                "normalizing should always be smaller than a full octave",
            );
        }
    }
}