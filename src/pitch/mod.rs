//! Pitch representation and manipulation.
//!
//! The [`Pitch`] type represents a musical pitch with specific spelling, combining a
//! [`Letter`] (A-G) with an [`AccidentalSign`] (sharp, flat, natural, etc.).
//! For spelling-agnostic pitch representation, use [`PitchClass`].
//!
//! # Examples
//!
//! ```
//! # use music_theory::{PitchClass, Pitch, Letter, AccidentalSign, Interval};
//! # use music_theory::pitch::Spelling;
//! use music_theory::EnharmonicEq as _;
//! 
//! // Create a pitch from letter and accidental
//! let c_sharp = Pitch::from_letter_and_accidental(
//!     Letter::C,
//!     AccidentalSign::SHARP
//! );
//!
//! assert_eq!(c_sharp, Pitch::C_SHARP);
//!
//! // Parse from string notation
//! let d_flat: Pitch = "Db".parse().unwrap();
//! assert_eq!(d_flat, Pitch::D_FLAT);
//!
//! // Check enharmonic equivalence
//! assert!(c_sharp.eq_enharmonic(&d_flat));
//!
//! // Work with pitch classes (spelling-agnostic)
//! let cs = PitchClass::Cs;
//! assert_eq!(c_sharp.as_pitch_class(), cs);
//! assert_eq!(d_flat.as_pitch_class(), cs); // Same pitch class
//!
//! // Spell pitch classes with sharps or flats
//! assert_eq!(cs.spell_with(Spelling::Sharps), c_sharp);
//! assert_eq!(cs.spell_with(Spelling::Flats), d_flat);
//!
//! // Transpose pitches by intervals
//! assert_eq!(
//!     Pitch::D_SHARP + Interval::AUGMENTED_SIXTH,
//!     Pitch::B_DOUBLE_SHARP
//! );
//! ```

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::LazyLock;
use regex::Regex;
use crate::{Interval, Semitones, EnharmonicEq, EnharmonicOrd};
use crate::enharmonic::{self, WithoutSpelling};
use crate::interval::Quality;
use crate::harmony::Key;

mod class;
pub use class::*;

mod letter;
pub use letter::*;

mod accidental;
pub use accidental::*;

mod spelling;
pub use spelling::*;

mod consts;

#[cfg(test)]
mod tests;

/// A musical pitch with letter name and accidental.
///
/// A `Pitch` represents a note with a specific spelling, such as C# or Db.
/// Enharmonically equivalent pitches like C# and Db are treated as different values.
/// For a representation that ignores spelling, use [`PitchClass`].
///
/// For convenience, `Pitch` defines many helper constants, like [`Pitch::D_FLAT`].
///
/// # Examples
///
/// You can create a `Pitch` from a letter and accidental:
/// ```
/// # use music_theory::{Pitch, Letter, AccidentalSign};
/// let c_sharp = Pitch::from_letter_and_accidental(Letter::C, AccidentalSign::SHARP);
/// # assert_eq!(c_sharp, Pitch::C_SHARP);
/// ```
///
/// Or from a constant:
/// ```
/// # use music_theory::Pitch;
/// let a_double_sharp = Pitch::A_DOUBLE_SHARP;
/// ```
///
/// Or by [parsing][std::str::FromStr] from a [`&str`](prim@str):
/// ```
/// # use music_theory::Pitch;
/// let d_flat = "Db".parse::<Pitch>().expect("should be a valid pitch");
/// # assert_eq!(d_flat, Pitch::D_FLAT);
/// ```
///
/// # Representation
///
/// `Pitch` is stored by how many fifths away from C it is, as an [`i16`][prim@i16].
/// This makes it cheap to copy and operate on.
/// ```
/// # use music_theory::{Pitch, Letter, AccidentalSign};
/// // Helper methods to get the internal representation
/// assert_eq!(Pitch::D.as_fifths_from_c(), 2);
/// assert_eq!(Pitch::from_fifths_from_c(-13), Pitch::G_DOUBLE_FLAT);
///
/// // Internally stored as i16
/// assert_eq!(size_of::<Pitch>(), size_of::<i16>());
///
/// // Can have up to ~4860 sharps
/// let max_sharps = AccidentalSign { offset: 4680 };
/// let _ = Pitch::from_letter_and_accidental(Letter::B, max_sharps);
///
/// // ... or ~4681 flats
/// let max_flats = AccidentalSign { offset: -4681 };
/// let _ = Pitch::from_letter_and_accidental(Letter::F, max_flats);
/// ```
///
/// The maximum number of accidentals depends on the letter, as the internal representation
/// is limited to `i16` range. Exceeding this limit will panic.
/// ```should_panic
/// # use music_theory::{Pitch, AccidentalSign, Letter};
/// let too_many = AccidentalSign { offset: 4681 };
/// let _ = Pitch::from_letter_and_accidental(Letter::B, too_many);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pitch(i16);

impl Pitch {
    /// Creates a new `Pitch` from a letter name and accidental sign.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{Pitch, Letter, AccidentalSign};
    /// let e_flat = Pitch::from_letter_and_accidental(
    ///     Letter::E,
    ///     AccidentalSign::FLAT
    /// );
    /// assert_eq!(e_flat, Pitch::E_FLAT);
    ///
    /// // Can create pitches with any accidental
    /// let f_triple_sharp = Pitch::from_letter_and_accidental(
    ///     Letter::F,
    ///     AccidentalSign { offset: 3 },
    /// );
    /// ```
    pub fn from_letter_and_accidental(letter: Letter, accidental_sign: AccidentalSign) -> Self {
        let col_offset = accidental_sign.offset;

        let pitch = letter.fifths_from_c() + 7 * col_offset;

        Self::from_fifths_from_c(pitch)
    }

    /// Gets the distance, in fifths, from C. (see [Representation](#representation))
    pub fn as_fifths_from_c(self) -> i16 {
        self.0
    }

    /// Creates a `Pitch` from its distance, in fifths, from C. (see [Representation](#representation))
    pub fn from_fifths_from_c(fifths: i16) -> Self {
        Self(fifths)
    }

    /// Returns the [`PitchClass`] of the `Pitch`, discarding enharmonic spelling.
    /// # Examples
    /// ```
    /// # use music_theory::{Pitch, PitchClass};
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
    /// # use music_theory::Pitch;
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
    /// # use music_theory::{Pitch, Semitones};
    /// assert_eq!(Pitch::F_SHARP.semitones_to(Pitch::B), Semitones(5));
    /// // Wraps around the edge of the octave.
    /// assert_eq!(Pitch::B.semitones_to(Pitch::D), Semitones(3));
    /// ```
    pub fn semitones_to(self, rhs: Self) -> Semitones {
        let lhs = self.as_pitch_class() as u8 as i8;
        let rhs = rhs.as_pitch_class() as u8 as i8;

        Semitones((rhs - lhs).rem_euclid(12) as _)
    }

    /// Returns the [`Letter`] component of the `Pitch`.
    /// # Examples
    /// ```
    /// # use music_theory::{Pitch, Letter};
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
    /// # use music_theory::{Pitch, AccidentalSign};
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
    pub(crate) fn semitones_offset_from_c(self) -> Semitones {
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

        Semitones(n as _)
    }

    /// Returns the same pitch spelled with either [sharps](Spelling::Sharps) or [flats](Spelling::Flats).
    ///
    /// If the pitch is already spelled with the given spelling, *it is returned unchanged*,
    /// even if it can be written in a simpler way. For example spelling `G##` with `sharps`
    /// will return `G##`, not `A`. If you'd like it to return `A` instead, consider using
    /// [`Self::simplified`].
    /// # Examples
    /// ```
    /// # use music_theory::Pitch;
    /// # use music_theory::pitch::Spelling;
    /// // Spell a pitch with flats
    /// assert_eq!(Pitch::A_SHARP.respell_with(Spelling::Flats), Pitch::B_FLAT);
    /// // ... or with sharps
    /// assert_eq!(Pitch::E_FLAT.respell_with(Spelling::Sharps), Pitch::D_SHARP);
    ///
    ///
    /// // Does nothing if a pitch with sharps is called with sharps
    /// assert_eq!(Pitch::C_SHARP.respell_with(Spelling::Sharps), Pitch::C_SHARP);
    ///
    /// // This will not simplify pitches if they're already spelled as intended
    /// assert_eq!(
    ///     Pitch::G_DOUBLE_SHARP.respell_with(Spelling::Sharps),
    ///     Pitch::G_DOUBLE_SHARP,
    /// );
    /// ```
    pub fn respell_with(self, spelling: Spelling) -> Self {
        if Spelling::from_accidental(self.accidental()) != Some(spelling) {
            self.as_pitch_class().spell_with(spelling)
        } else {
            self
        }
    }

    /// Returns the same pitch with fewer accidentals.
    /// # Examples
    /// ```
    /// # use music_theory::Pitch;
    /// assert_eq!(Pitch::E_SHARP.simplified(), Pitch::F);
    /// assert_eq!(Pitch::F_DOUBLE_SHARP.simplified(), Pitch::G);
    /// assert_eq!(Pitch::C_DOUBLE_FLAT.simplified(), Pitch::B_FLAT);
    ///
    /// // Already simplified pitches are not further simplified
    /// assert_eq!(Pitch::G_FLAT.simplified(), Pitch::G_FLAT);
    /// assert_eq!(Pitch::G.simplified(), Pitch::G);
    /// ```
    pub fn simplified(self) -> Self {
        let spelling = Spelling::from_accidental(self.accidental())
            .unwrap_or_default();

        self.as_pitch_class().spell_with(spelling)
    }

    /// Returns the pitch's enharmonic.
    ///
    /// If a pitch can't be written with a natural, the returned pitch will always have the
    /// opposite spelling as before. Notably, this means the enharmonic of `Ex` is `Gb`, *not* `F#`.
    /// For the opposite behavior, see [`Self::simplified`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Pitch;
    /// assert_eq!(Pitch::C_SHARP.enharmonic(), Pitch::D_FLAT);
    /// assert_eq!(Pitch::B_DOUBLE_FLAT.enharmonic(), Pitch::A);
    /// assert_eq!(Pitch::E_DOUBLE_SHARP.enharmonic(), Pitch::G_FLAT);
    ///
    /// // Pitches that can be written with no accidentals will be written
    /// // with no accidentals. As such, pitches with no accidentals will
    /// // return themselves.
    /// assert_eq!(Pitch::G.enharmonic(), Pitch::G);
    /// ```
    pub fn enharmonic(self) -> Self {
        let spelling = Spelling::from_accidental(self.accidental())
            .unwrap_or_default()
            .flip();

        self.as_pitch_class().spell_with(spelling)
    }

    /// Respells this pitch according to the key signature.
    ///
    /// Corrects the spelling of notes diatonic to the key (notes that appear in the key's scale)
    /// to match the key signature. Notes not diatonic to the key preserve original spelling.
    ///
    /// For example, respelling `Bb` in G major will remain `Bb`, even though G major is a key with sharps.
    /// This is because `Bb` doesn't appear in the G major scale (disregarding spelling). If you intend
    /// for it to return `A#`, use `self.as_pitch_class().spell_in_key(key)`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Pitch;
    /// # use music_theory::harmony::Key;
    /// let g_major = Key::major(Pitch::G);
    ///
    /// // Diatonic notes are respelled to match the key
    /// assert_eq!(Pitch::G_FLAT.respell_in_key(g_major), Pitch::F_SHARP);
    ///
    /// // Notes that aren't diatonic preserve spelling.
    /// assert_eq!(Pitch::B_FLAT.respell_in_key(g_major), Pitch::B_FLAT);
    /// assert_eq!(Pitch::A_SHARP.respell_in_key(g_major), Pitch::A_SHARP);
    /// // ... but if you don't want this behavior, call 'as_pitch_class()' first
    /// assert_eq!(
    ///     Pitch::B_FLAT.as_pitch_class().spell_in_key(g_major),
    ///     Pitch::A_SHARP,
    /// );
    /// ```
    pub fn respell_in_key(self, key: Key) -> Self {
        if let Some(pitch) = key.scale_experimental()
            .build_default()
            .into_iter()
            .find(|p| self.eq_enharmonic(p))
        {
            return pitch;
        }

        self
    }

    /// Transposes the pitch by the given interval. Has the same behavior as the [`+` operator](Add::add).
    /// # Examples
    /// ```
    /// # use music_theory::{Pitch, Interval};
    /// assert_eq!(Pitch::D.transpose(Interval::MAJOR_THIRD), Pitch::F_SHARP);
    /// // Descending intervals are also supported
    /// assert_eq!(Pitch::B.transpose(-Interval::MINOR_SIXTH), Pitch::D_SHARP);
    /// // Same behavior as Add::add
    /// let m2 = Interval::MINOR_SECOND;
    /// assert_eq!(Pitch::C.transpose(m2), Pitch::C + m2);
    /// ```
    pub fn transpose(self, interval: Interval) -> Self {
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
            Quality::Augmented(n) => -(n.get() as i16 - 1),
            Quality::Perfect | Quality::Major => 1,
            Quality::Minor => 2,

            Quality::Diminished(n) => match interval.number().is_perfect() {
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
    /// # use music_theory::{Pitch, Interval};
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
    /// # use music_theory::Pitch;
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
    /// Formats the pitch for debugging output.
    ///
    /// Uses letter names (like "CSharp", "EFlat") rather than symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::Pitch;
    /// assert_eq!(format!("{:?}", Pitch::C), "C");
    /// assert_eq!(format!("{:?}", Pitch::F_SHARP), "FSharp");
    /// assert_eq!(format!("{:?}", Pitch::B_FLAT), "BFlat");
    /// assert_eq!(format!("{:?}", Pitch::G_DOUBLE_SHARP), "GDoubleSharp");
    /// ```
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
    /// Formats the pitch using Unicode musical symbols.
    ///
    /// Uses standard notation with sharp (♯), flat (♭), double sharp (𝄪),
    /// and double flat (𝄫) symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::Pitch;
    /// assert_eq!(Pitch::C.to_string(), "C");
    /// assert_eq!(Pitch::F_SHARP.to_string(), "F♯");
    /// assert_eq!(Pitch::B_FLAT.to_string(), "B♭");
    /// assert_eq!(Pitch::G_DOUBLE_SHARP.to_string(), "G𝄪");
    /// assert_eq!(Pitch::E_DOUBLE_FLAT.to_string(), "E𝄫");
    /// ```
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

impl WithoutSpelling for Pitch {
    type Unspelled = PitchClass;

    fn without_spelling(self) -> Self::Unspelled {
        self.as_pitch_class()
    }
}

impl EnharmonicEq for Pitch {
    fn eq_enharmonic(&self, other: &Self) -> bool {
        enharmonic::defer_without_spelling::eq(self, other)
    }
}

impl EnharmonicOrd for Pitch {
    fn cmp_enharmonic(&self, other: &Self) -> Ordering {
        enharmonic::defer_without_spelling::cmp(self, other)
    }
}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Pitch {
    /// Compares two pitches by spelling (letter, then accidental).
    ///
    /// This compares pitches alphabetically by spelling. For example, E# < F because [`Letter::E`]
    /// is less than [`Letter::F`], even though they are enharmonically equivalent.
    ///
    /// To compare in a spelling agnostic way instead, use [`EnharmonicOrd::cmp_enharmonic`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::Pitch;
    /// assert!(Pitch::C < Pitch::D);
    /// assert!(Pitch::C_SHARP < Pitch::D_FLAT); // C comes before D
    /// assert!(Pitch::E_SHARP < Pitch::F); // E comes before F
    /// assert!(Pitch::C < Pitch::C_SHARP); // Natural before sharp
    /// ```
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.letter()
            .cmp(&rhs.letter())
            .then(
                self.accidental()
                    .cmp(&rhs.accidental())
            )
    }
}

impl From<PitchClass> for Pitch {
    /// Converts a pitch class to a pitch with the default sharp spelling.
    ///
    /// Natural pitch classes become natural pitches, and pitches with accidentals are spelled
    /// with sharps. To spell in a different way, see [`Self::respell_with`] or [`Self::respell_in_key`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{Pitch, PitchClass};
    /// let pitch: Pitch = PitchClass::C.into();
    /// assert_eq!(pitch, Pitch::C);
    ///
    /// let pitch: Pitch = PitchClass::As.into();
    /// assert_eq!(pitch, Pitch::A_SHARP);
    /// ```
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
    /// Converts a letter to a natural pitch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{Letter, Pitch};
    /// let pitch: Pitch = Letter::C.into();
    /// assert_eq!(pitch, Pitch::C);
    ///
    /// let pitch: Pitch = Letter::G.into();
    /// assert_eq!(pitch, Pitch::G);
    /// ```
    fn from(letter: Letter) -> Self {
        Self::from_letter_and_accidental(letter, AccidentalSign::NATURAL)
    }
}

impl From<Pitch> for PitchClass {
    /// Converts a pitch to its pitch class, discarding the spelling.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{Pitch, PitchClass};
    /// let pc: PitchClass = Pitch::C_SHARP.into();
    /// assert_eq!(pc, PitchClass::Cs);
    ///
    /// let pc: PitchClass = Pitch::D_FLAT.into();
    /// assert_eq!(pc, PitchClass::Cs);
    /// ```
    fn from(pitch: Pitch) -> Self {
        pitch.as_pitch_class()
    }
}

impl From<Pitch> for Letter {
    /// Extracts the letter component from a pitch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{Pitch, Letter};
    /// let letter: Letter = Pitch::E_FLAT.into();
    /// assert_eq!(letter, Letter::E);
    /// ```
    fn from(pitch: Pitch) -> Self {
        pitch.letter()
    }
}

/// Error returned by [`Pitch::from_str`] if the [`&str`](prim@str) could not be parsed into a `Pitch`.
/// # Examples
/// ```
/// # use music_theory::Pitch;
/// # use music_theory::pitch::PitchFromStrError;
/// assert_eq!("D half-flat".parse::<Pitch>(), Err(PitchFromStrError));
/// ```
#[derive(Debug, thiserror::Error, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("The str could not be converted to a pitch")]
pub struct PitchFromStrError;

// TODO: add support for pitches like F(25x)Flat
// TODO: change to make more like tonaljs/note's Note::name
impl FromStr for Pitch {
    type Err = PitchFromStrError;

    /// Parses a pitch from a string.
    ///
    /// Accepts pitch notation with optional accidentals using various formats:
    /// - Sharp: `#`, `♯`, `sharp`, `+`
    /// - Flat: `b`, `♭`, `flat`, `-`
    /// - Double sharp: `##`, `♯♯`, `𝄪`, `double sharp`
    /// - Double flat: `bb`, `♭♭`, `𝄫`, `double flat`
    ///
    /// The parsing is case-insensitive for letters and word-based accidentals.
    ///
    /// # Errors
    ///
    /// Returns [`PitchFromStrError`] if the string cannot be parsed as a valid pitch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::Pitch;
    /// assert_eq!("C".parse::<Pitch>(), Ok(Pitch::C));
    /// assert_eq!("F#".parse::<Pitch>(), Ok(Pitch::F_SHARP));
    /// assert_eq!("Bb".parse::<Pitch>(), Ok(Pitch::B_FLAT));
    /// assert_eq!("G sharp".parse::<Pitch>(), Ok(Pitch::G_SHARP));
    /// assert_eq!("E♭".parse::<Pitch>(), Ok(Pitch::E_FLAT));
    /// assert_eq!("C##".parse::<Pitch>(), Ok(Pitch::C_DOUBLE_SHARP));
    /// assert_eq!("D double flat".parse::<Pitch>(), Ok(Pitch::D_DOUBLE_FLAT));
    ///
    /// // Case insensitive
    /// assert_eq!("c".parse::<Pitch>(), Ok(Pitch::C));
    /// assert_eq!("Ab".parse::<Pitch>(), Ok(Pitch::A_FLAT));
    ///
    /// // Invalid inputs return errors
    /// assert!("H".parse::<Pitch>().is_err());
    /// assert!("C4".parse::<Pitch>().is_err()); // Octave numbers not allowed
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: accept 'x' as double sharp
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

    /// Transposes a pitch up by an interval.
    ///
    /// This is equivalent to calling [`Pitch::transpose`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{Pitch, Interval};
    /// assert_eq!(Pitch::C + Interval::MAJOR_THIRD, Pitch::E);
    /// ```
    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Interval> for Pitch {
    type Output = Self;

    /// Transposes a pitch down by an interval.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{Pitch, Interval};
    /// assert_eq!(Pitch::B_FLAT - Interval::MINOR_SECOND, Pitch::A);
    /// ```
    fn sub(self, rhs: Interval) -> Self::Output {
        self + (-rhs)
    }
}