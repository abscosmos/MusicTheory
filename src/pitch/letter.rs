use std::fmt;
use std::str::FromStr;
use strum_macros::{EnumIter, FromRepr};
use crate::harmony::Key;
use crate::Pitch;

/// A musical note letter (C, D, E, F, G, A, or B).
///
/// Represents the diatonic position of a note in Western music notation.
/// Letters are ordered starting from C, which has step 0.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// // Letters can be compared
/// assert!(Letter::C < Letter::D);
/// assert!(Letter::G > Letter::F);
///
/// assert_eq!(Letter::C.step(), 0);
/// assert_eq!(Letter::G.step(), 4);
///
/// // Calculate distance between letters, always positive
/// assert_eq!(Letter::C.offset_between(Letter::E), 2);
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter, FromRepr, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Letter {
    /// C
    C = 0,
    /// D
    D,
    /// E
    E,
    /// F
    F,
    /// G
    G,
    /// A
    A,
    /// B
    B,
}

impl Letter {
    /// Returns the step number of this letter.
    ///
    /// Step numbers range from 0 (C) to 6 (B).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Letter::D.step(), 1);
    /// assert_eq!(Letter::B.step(), 6);
    /// ```
    pub const fn step(self) -> u8 {
        self as _
    }

    /// Creates a letter from a step number.
    ///
    /// Returns `None` if the step is not in the valid range [0, 6].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Letter::from_step(4), Some(Letter::G));
    /// assert_eq!(Letter::from_step(7), None);
    /// ```
    pub const fn from_step(step: u8) -> Option<Self> {
        Self::from_repr(step)
    }

    /// Calculates the ascending diatonic distance from this letter to another.
    ///
    /// The result is always in the range [0, 6], representing the number of
    /// diatonic steps ascending from `self` to `rhs` (wrapping around if necessary).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// // C to E is 2 steps up
    /// assert_eq!(Letter::C.offset_between(Letter::E), 2);
    /// // E to C wraps around
    /// assert_eq!(Letter::E.offset_between(Letter::C), 5);
    /// ```
    pub fn offset_between(&self, rhs: Self) -> u8 {
        (rhs.step() as i16 - self.step() as i16).rem_euclid(7) as _
    }

    /// Converts this letter to a pitch in the specified key.
    ///
    /// The resulting pitch will have the appropriate accidental for the given key signature.
    /// For example, F in G major becomes F♯, while F in C major remains F natural.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// // F becomes F♯ in G major
    /// assert_eq!(
    ///     Letter::F.to_pitch_in_key(Key::major(Pitch::G)),
    ///     Pitch::F_SHARP,
    /// );
    ///
    /// // F remains F natural in C major
    /// assert_eq!(
    ///     Letter::F.to_pitch_in_key(Key::major(Pitch::C)),
    ///     Pitch::F,
    /// );
    /// ```
    pub fn to_pitch_in_key(self, key: Key) -> Pitch {
        Pitch::from_letter_and_accidental(self, key.accidental_of(self))
    }

    /// Same as [`Pitch::as_fifths_from_c`], if `self` was converted to [`Pitch`].
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

/// Error returned when parsing a [`Letter`] from a [`&str`](prim@str) fails.
///
/// This error occurs when the input string is not one of the valid letter names
/// (A, B, C, D, E, F, or G), case-insensitive.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// assert_eq!("C".parse::<Letter>(), Ok(Letter::C));
/// assert_eq!("g".parse::<Letter>(), Ok(Letter::G));
/// assert_eq!("H".parse::<Letter>(), Err(InvalidLetter));
/// ```
#[derive(Copy, Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("Letter must be A, B, C, D, E, F, or G")]
pub struct InvalidLetter;

impl FromStr for Letter {
    type Err = InvalidLetter;

    /// Parses a letter from a string.
    ///
    /// Accepts both uppercase and lowercase single-character strings
    /// representing the letters A through G.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidLetter`] if the string is not a valid letter name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!("F".parse::<Letter>(), Ok(Letter::F));
    /// assert_eq!("a".parse::<Letter>(), Ok(Letter::A));
    /// assert_eq!("Bb".parse::<Letter>(), Err(InvalidLetter));
    /// ```
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
