//! Pitch representation and manipulation.
//!
//! The [`Pitch`] type represents a musical pitch through letter and accidental.
//! It provides functionality for pitch comparison, enharmonic equivalence,
//! transposition, and various other musical operations.
//!
//! # Examples
//! ```
//! use music_theory::prelude::*;
//!
//! // Create a pitch from letter and accidental
//! let c_sharp = Pitch::from_letter_and_accidental(Letter::C, AccidentalSign::SHARP);
//!
//! // Create from string notation
//! let d_flat = "Db".parse::<Pitch>().unwrap();
//!
//! // Use a predefined constant
//! let a_double_sharp = Pitch::A_DOUBLE_SHARP;
//!
//! // Check enharmonic equivalence
//! assert!(c_sharp.eq_enharmonic(&d_flat));
//!
//! // Transpose pitches by intervals
//! assert_eq!(c_sharp + Interval::AUGMENTED_SIXTH, a_double_sharp);
//! ```

// TODO: module docs need updating, since many other types have been moved into this module

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::LazyLock;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::interval::Interval;
use crate::interval::IntervalQuality;
use crate::semitone::Semitone;

mod class;
pub use class::*;

mod letter;
pub use letter::*;

mod accidental;
pub use accidental::*;

mod consts;

#[cfg(test)]
mod tests;

/// A musical pitch with letter name and accidental.
/// 
/// A `Pitch` is internally defined by the number of fifths from C.
/// This representation allows for up to 4860 sharps or 4861 flats.
/// To get a representation without enharmonic spelling, use [`PitchClass`].
///
/// For convenience, `Pitch` defines many helper constants, like [`Pitch::D_FLAT`].
/// 
/// # Examples
///
/// You can create a `Pitch` from a letter and accidental:
/// ```
/// # use music_theory::prelude::*;
/// let c_sharp = Pitch::from_letter_and_accidental(Letter::C, AccidentalSign::SHARP);
/// # assert_eq!(c_sharp, Pitch::C_SHARP);
/// ```
///
/// Or from a constant:
/// ```
/// # use music_theory::prelude::*;
/// let a_double_sharp = Pitch::A_DOUBLE_SHARP;
/// ```
///
/// Or by [parsing][std::str::FromStr] from a `&str`:
/// ```
/// # use music_theory::prelude::*;
/// let d_flat = "Db".parse::<Pitch>().expect("should be a valid pitch");
/// # assert_eq!(d_flat, Pitch::D_FLAT);
/// ```
///
/// # Representation
///
/// `Pitch` is stored by how many fifths away from C it is, as an [`i16`][prim@i16].
/// This makes it cheap to copy and operate on.
/// ```
/// # use music_theory::prelude::*;
/// // Helper methods to get the internal representation
/// assert_eq!(Pitch::D.as_fifths_from_c(), 2);
/// assert_eq!(Pitch::from_fifths_from_c(-13), Pitch::G_DOUBLE_FLAT);
///
/// // Internally stored as i16
/// assert_eq!(size_of::<Pitch>(), size_of::<i16>());
///
/// // Can have up to 4860 sharps
/// let max_sharps = AccidentalSign { offset: 4680 };
/// let _ = Pitch::from_letter_and_accidental(Letter::B, max_sharps);
///
/// // ... or 4681 flats
/// let max_flats = AccidentalSign { offset: -4681 };
/// let _ = Pitch::from_letter_and_accidental(Letter::F, max_flats);
/// ```
///
/// Using more flats or sharps will panic.
/// ```should_panic
/// # use music_theory::prelude::*;
/// let sharps = AccidentalSign { offset: 4681 };
/// let _ = Pitch::from_letter_and_accidental(Letter::B, sharps);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pitch(i16);

impl Pitch {
    /// Creates a new `Pitch` from a letter name and accidental sign.
    pub fn from_letter_and_accidental(letter: Letter, accidental_sign: AccidentalSign) -> Self {
        let col_offset = accidental_sign.offset;

        let pitch = letter.fifths_from_c() + 7 * col_offset;

        Self::from_fifths_from_c(pitch)
    }

    /// Get the distance, in fifths, to C. (see [Representation](#representation))
    pub fn as_fifths_from_c(self) -> i16 {
        self.0
    }

    /// Creates a `Pitch` from its distance, in fifths, to C. (see [Representation](#representation))
    pub fn from_fifths_from_c(fifths: i16) -> Self {
        Self(fifths)
    }

    /// Returns the [`PitchClass`] of the `Pitch`, discarding enharmonic spelling.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::D_SHARP.as_pitch_class(), PitchClass::Ds);
    /// assert_eq!(Pitch::G_FLAT.as_pitch_class(), PitchClass::Fs);
    /// assert_eq!(Pitch::A_DOUBLE_SHARP.as_pitch_class(), PitchClass::B);
    /// ```
    pub fn as_pitch_class(self) -> PitchClass {
        let fifths_plus_one = self.as_fifths_from_c() + 1;

        let semitone_offset = fifths_plus_one.div_euclid(7);

        let semitones_from_c = match fifths_plus_one.rem_euclid(7) {
            n if n % 2 == 0 => n + 5,
            n => n - 1,
        } + semitone_offset;

        let semitones_from_c = semitones_from_c
            .rem_euclid(12)
            .try_into()
            .expect("i8::rem_euclid(12) must be [0, 12)");

        PitchClass::from_chroma(semitones_from_c)
            .expect("i8::rem_euclid(12) must be [0, 12)")
    }

    /// Gets the Pitch's [pitch class](PitchClass) chroma.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::C.chroma(), 0);
    /// assert_eq!(Pitch::E.chroma(), 4);
    /// assert_eq!(Pitch::D_DOUBLE_SHARP.chroma(), 4);
    /// ```
    pub fn chroma(self) -> u8 {
        self.as_pitch_class().chroma()
    }

    /// Returns how many semitones `rhs` is from `self`. Always positive, in `[0,11]`.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::F_SHARP.semitones_to(Pitch::B), Semitone(5));
    /// // Wraps around the edge of the octave.
    /// assert_eq!(Pitch::B.semitones_to(Pitch::D), Semitone(3));
    /// ```
    pub fn semitones_to(self, rhs: Self) -> Semitone {
        let lhs = self.as_pitch_class() as u8 as i8;
        let rhs = rhs.as_pitch_class() as u8 as i8;

        Semitone((rhs - lhs).rem_euclid(12) as _)
    }

    /// Returns the [`Letter`] component of the `Pitch`.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::D_SHARP.letter(), Letter::D);
    /// ```
    pub fn letter(self) -> Letter {
        match (self.as_fifths_from_c() + 1).rem_euclid(7) {
            0 => Letter::F,
            1 => Letter::C,
            2 => Letter::G,
            3 => Letter::D,
            4 => Letter::A,
            5 => Letter::E,
            6 => Letter::B,
            _ => unreachable!("i16::rem_euclid(7) must be [0, 7)"),
        }
    }

    /// Returns the [`AccidentalSign`] of the `Pitch`.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::F_DOUBLE_FLAT.accidental(), AccidentalSign::DOUBLE_FLAT);
    /// assert_eq!(Pitch::G.accidental(), AccidentalSign::NATURAL);
    /// ```
    pub fn accidental(self) -> AccidentalSign {
        AccidentalSign {
            offset: (self.as_fifths_from_c() + 1).div_euclid(7)
        }
    }

    /// Returns the signed semitone offset of `self` from C.
    ///
    /// Can return negative values for pitches below C (e.g., C♭♭ returns -2).
    pub(crate) fn semitones_offset_from_c(self) -> Semitone {
        let fifths_plus_one = self.as_fifths_from_c() + 1;

        let n = match fifths_plus_one.rem_euclid(7) {
            0 => 5, // F
            1 => 0, // C
            2 => 7, // G
            3 => 2, // D
            4 => 9, // A
            5 => 4, // E
            6 => 11, // B
            _ => unreachable!("i8::rem_euclid(7) must be [0, 7)")
        } + fifths_plus_one.div_euclid(7);

        Semitone(n as _)
    }

    /// Returns the same pitch with fewer accidentals.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::E_SHARP.simplified(), Pitch::F);
    /// assert_eq!(Pitch::F_DOUBLE_SHARP.simplified(), Pitch::G);
    ///
    /// // Already simplified notes are not further simplified
    /// assert_eq!(Pitch::G_FLAT.simplified(), Pitch::G_FLAT);
    /// assert_eq!(Pitch::G.simplified(), Pitch::G);
    /// ```
    pub fn simplified(self) -> Self {
        self.bias(self.accidental().offset > 0)
    }

    /// Returns the pitch's enharmonic.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::C_SHARP.enharmonic(), Pitch::D_FLAT);
    /// assert_eq!(Pitch::F_DOUBLE_SHARP.enharmonic(), Pitch::G);
    ///
    /// // Notes with no accidentals will return themselves
    /// assert_eq!(Pitch::G.enharmonic(), Pitch::G);
    /// ```
    pub fn enharmonic(self) -> Self {
        self.bias(self.accidental().offset < 0)
    }
    
    // TODO: should this function simplify if called with G## & true?
    /// Returns the same pitch spelled with sharps, if `sharps` is `true`, or with flats, if `false`.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Spell a note with flats
    /// assert_eq!(Pitch::A_SHARP.bias(false), Pitch::B_FLAT);
    /// // ... or with sharps
    /// assert_eq!(Pitch::E_FLAT.bias(true), Pitch::D_SHARP);
    ///
    ///
    /// // Does nothing if a pitch with sharps is called with true
    /// assert_eq!(Pitch::C_SHARP.bias(true), Pitch::C_SHARP);
    /// // This will simplify a note if it can be written with fewer accidentals
    /// assert_eq!(Pitch::G_DOUBLE_SHARP.bias(true), Pitch::A);
    /// ```
    pub fn bias(self, sharp: bool) -> Self {
        self.as_pitch_class().bias(sharp)
    }

    /// Transposes the pitch by the given interval. Has the same behavior as the [`+` operator](Add::add).
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::D.transpose(Interval::MAJOR_THIRD), Pitch::F_SHARP);
    /// // Descending intervals are also supported
    /// assert_eq!(Pitch::B.transpose(-Interval::MINOR_SIXTH), Pitch::D_SHARP);
    /// // Same behavior as Add::add
    /// let m2 = Interval::MINOR_SECOND;
    /// assert_eq!(Pitch::C.transpose(m2), Pitch::C + m2);
    /// ```
    pub fn transpose(self, interval: Interval) -> Self {
        use IntervalQuality as Q;
        
        let start = match interval.number().as_simple().get().abs() {
            1 | 8 => 7,
            2 => 9,
            3 => 11,
            4 => 6,
            5 => 8,
            6 => 10,
            7 => 12,
            _ => unreachable!("a simple interval can't be bigger than an octave")
        };

        let quality_offset = match interval.quality() {
            Q::Augmented(n) => -(n.get() as i16 - 1),
            Q::Perfect | Q::Major => 1,
            Q::Minor => 2,
            
            Q::Diminished(n) => match interval.number().is_perfect() {
                true => n.get() as i16 + 1,
                false => n.get() as i16 + 2,
            }
        };

        let offset = start - 7 * quality_offset;

        let dir_offset = if interval.is_ascending() { offset } else { -offset };

        self.transpose_fifths(dir_offset)
    }
    
    /// Calculates the interval between `self` and `rhs`. Will always return an ascending interval.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Pitch::C.distance_to(Pitch::G), Interval::PERFECT_FIFTH);
    /// 
    /// // Aware of enharmonic spelling, like a tritone being either an A4 or d5
    /// assert_eq!(Pitch::G.distance_to(Pitch::C_SHARP), Interval::AUGMENTED_FOURTH);
    /// assert_eq!(Pitch::G.distance_to(Pitch::D_FLAT), Interval::DIMINISHED_FIFTH);
    /// ```
    pub fn distance_to(self, rhs: Self) -> Interval {
        Interval::between_pitches(self, rhs)
    }
    
    /// Transposes the pitch by a given number of fifths.
    /// This is much more efficient than [`Self::transpose`] due to internal [representation](#representation).
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // B is 3 fifths clockwise from D on the circle of fifths
    /// assert_eq!(Pitch::D.transpose_fifths(3), Pitch::B);
    /// // ... and Eb is 4 fifths counterclockwise
    /// assert_eq!(Pitch::G.transpose_fifths(-4), Pitch::E_FLAT);
    /// ```
    pub fn transpose_fifths(self, fifths: i16) -> Self {
        let curr = self.as_fifths_from_c();

        Self::from_fifths_from_c(curr + fifths)
    }
}

impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = self.letter();
        let accidental = self.accidental();

        if accidental != AccidentalSign::NATURAL {
            write!(f, "{letter:?}{accidental:?}")
        } else {
            write!(f, "{letter:?}")
        }
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = self.letter();
        let accidental = self.accidental();

        if accidental != AccidentalSign::NATURAL {
            write!(f, "{letter}{accidental}")
        } else {
            write!(f, "{letter}")
        }
    }
}

impl EnharmonicEq for Pitch {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.as_pitch_class() == rhs.as_pitch_class()
    }
}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Pitch {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.letter()
            .cmp(&rhs.letter())
            .then(
                self.accidental()
                    .cmp(&rhs.accidental())
            )
    }
}

impl EnharmonicOrd for Pitch {
    fn cmp_enharmonic(&self, rhs: &Self) -> Ordering {
        self.as_pitch_class()
            .cmp(&rhs.as_pitch_class())
    }
}

impl From<PitchClass> for Pitch {
    fn from(value: PitchClass) -> Self {
        match value {
            PitchClass::C => Pitch::C,
            PitchClass::Cs => Pitch::C_SHARP,
            PitchClass::D => Pitch::D,
            PitchClass::Ds => Pitch::D_SHARP,
            PitchClass::E => Pitch::E,
            PitchClass::F => Pitch::F,
            PitchClass::Fs => Pitch::F_SHARP,
            PitchClass::G => Pitch::G,
            PitchClass::Gs => Pitch::G_SHARP,
            PitchClass::A => Pitch::A,
            PitchClass::As => Pitch::A_SHARP,
            PitchClass::B => Pitch::B,
        }
    }
}

impl From<Letter> for Pitch {
    fn from(letter: Letter) -> Self {
        Self::from_letter_and_accidental(letter, AccidentalSign::NATURAL)
    }
}

impl From<Pitch> for PitchClass {
    fn from(pitch: Pitch) -> Self {
        pitch.as_pitch_class()
    }
}

impl From<Pitch> for Letter {
    fn from(pitch: Pitch) -> Self {
        pitch.letter()
    }
}

/// Error returned by [`Pitch::from_str`] if the [`&str`](prim@str) could not be parsed into a `Pitch`.
/// # Examples
/// ```
/// # use music_theory::prelude::*;
/// assert_eq!("D half-flat".parse::<Pitch>(), Err(PitchFromStrError));
/// ```
#[derive(Debug, thiserror::Error, Eq, PartialEq, Copy, Clone)]
#[error("The str could not be converted to a pitch")]
pub struct PitchFromStrError;

// TODO: add support for pitches like F(25x)Flat
// TODO: change to make more like tonaljs/note's Note::name
impl FromStr for Pitch {
    type Err = PitchFromStrError;

    // TODO: add documentation specifying use of this method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: LazyLock<Regex> = LazyLock::new(||
            Regex::new(r"(?i)^([A-G])\s?((?-i)b|(?-i)bb|(?i)sharp|♯|\+|\++|#|##|♯♯|𝄪|flat|♭|-|--|♭♭|𝄫|double\s?sharp|double\s?flat)?$")
                .expect("valid regex")
        );

        let caps = REGEX.captures(s)
            .ok_or(PitchFromStrError)?;

        let letter = caps.get(1)
            .ok_or(PitchFromStrError)?
            .as_str()
            .parse()
            .map_err(|_| PitchFromStrError)?;

        let accidental = caps.get(2);

        let acc = match accidental {
            None => AccidentalSign::NATURAL,
            Some(acc) => match acc
                .as_str()
                .trim()
                .to_lowercase()
                .as_str()
            {
                "+" | "#" | "♯" | "sharp" => AccidentalSign::SHARP,
                "-" | "b" | "♭" | "flat" => AccidentalSign::FLAT,
                "++" | "##" | "♯♯" | "𝄪" | "double sharp" | "doublesharp" => AccidentalSign::DOUBLE_SHARP,
                "--" | "bb" | "♭♭" | "𝄫" | "double flat" | "doubleflat" => AccidentalSign::DOUBLE_FLAT,
                _ => unreachable!("all cases should be covered"),
            }
        };

        Ok(Self::from_letter_and_accidental(letter, acc))
    }
}

impl Add<Interval> for Pitch {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Interval> for Pitch {
    type Output = Self;

    fn sub(self, rhs: Interval) -> Self::Output {
        self + (-rhs)
    }
}