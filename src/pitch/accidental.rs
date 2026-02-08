use std::fmt;
use std::str::FromStr;
use crate::Semitones;

/// An accidental that modifies a pitch.
///
/// Represents sharps, flats, naturals, and their multiples (double sharp, triple flat, etc.).
///
/// # Examples
///
/// ```
/// # use music_theory::{AccidentalSign, Semitones};
/// // Use the predefined constants
/// let sharp = AccidentalSign::SHARP;
/// let flat = AccidentalSign::FLAT;
/// let natural = AccidentalSign::NATURAL;
///
/// assert_eq!(sharp.offset_semitones(), Semitones(1));
/// assert_eq!(flat.offset_semitones(), Semitones(-1));
/// assert_eq!(natural.offset_semitones(), Semitones(0));
///
/// // Create from semitone offsets
/// let double_sharp = AccidentalSign::from_offset_semitones(Semitones(2));
/// assert_eq!(double_sharp, AccidentalSign::DOUBLE_SHARP);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccidentalSign {
    /// The semitone offset of this accidental.
    ///
    /// Positive values are sharps, negative values are flats, zero is natural.
    pub offset: i16,
}

impl AccidentalSign {
    /// Double flat (bb), lowers pitch by 2 semitones.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// assert_eq!(AccidentalSign::DOUBLE_FLAT.offset_semitones(), Semitones(-2));
    /// ```
    pub const DOUBLE_FLAT: Self = Self { offset: -2 };

    /// Flat (b), lowers pitch by 1 semitone.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// assert_eq!(AccidentalSign::FLAT.offset_semitones(), Semitones(-1));
    /// ```
    pub const FLAT: Self = Self { offset: -1 };

    /// Natural, no pitch alteration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// assert_eq!(AccidentalSign::NATURAL.offset_semitones(), Semitones(0));
    /// ```
    pub const NATURAL: Self = Self { offset: 0 };

    /// Sharp (#), raises pitch by 1 semitone.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// assert_eq!(AccidentalSign::SHARP.offset_semitones(), Semitones(1));
    /// ```
    pub const SHARP: Self = Self { offset: 1 };

    /// Double sharp (x), raises pitch by 2 semitones.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// assert_eq!(AccidentalSign::DOUBLE_SHARP.offset_semitones(), Semitones(2));
    /// ```
    pub const DOUBLE_SHARP: Self = Self { offset: 2 };

    /// Returns the semitone offset of this accidental.
    ///
    /// Positive values indicate sharps, negative values indicate flats, and zero indicates a natural.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// assert_eq!(AccidentalSign::SHARP.offset_semitones(), Semitones(1));
    /// assert_eq!(AccidentalSign::NATURAL.offset_semitones(), Semitones(0));
    ///
    /// // Works with any offset
    /// let triple_sharp = AccidentalSign::from_offset_semitones(Semitones(3));
    /// assert_eq!(triple_sharp.offset_semitones(), Semitones(3));
    /// ```
    pub fn offset_semitones(self) -> Semitones {
        Semitones(self.offset)
    }

    /// Creates an accidental from a semitone offset.
    ///
    /// Positive offsets create sharps, negative offsets create flats,
    /// and zero creates a natural. Any integer offset is valid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// let sharp = AccidentalSign::from_offset_semitones(Semitones(1));
    /// assert_eq!(sharp, AccidentalSign::SHARP);
    ///
    /// let triple_flat = AccidentalSign::from_offset_semitones(Semitones(-3));
    /// assert_eq!(triple_flat.offset_semitones(), Semitones(-3));
    /// ```
    pub fn from_offset_semitones(offset: Semitones) -> Self {
        Self { offset: offset.0 }
    }

    /// Returns a wrapper that formats the accidental using Unicode musical symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// assert_eq!(format!("{}", AccidentalSign::NATURAL.display_unicode()), "♮");
    /// assert_eq!(format!("{}", AccidentalSign::SHARP.display_unicode()), "♯");
    /// assert_eq!(format!("{}", AccidentalSign::DOUBLE_FLAT.display_unicode()), "𝄫");
    ///
    /// let triple_sharp = AccidentalSign::from_offset_semitones(Semitones(3));
    /// assert_eq!(format!("{}", triple_sharp.display_unicode()), "♯𝄪");
    /// ```
    pub fn display_unicode(self) -> DisplayUnicode {
        DisplayUnicode(self)
    }

    /// Helper function to implement [`fmt::Display`] with both ASCII and Unicode characters.
    fn fmt_with_symbols(
        &self,
        f: &mut fmt::Formatter<'_>,
        natural: &str,
        sharp: &str,
        flat: &str,
        double_sharp: &str,
        double_flat: &str,
    ) -> fmt::Result {
        let offset = self.offset;

        if offset == 0 {
            f.write_str(natural)
        } else {
            let num_double = offset.abs() / 2;
            let add_single = offset.abs() % 2 == 1;

            let (d, s) = if offset > 0 {
                (double_sharp, sharp)
            } else {
                (double_flat, flat)
            };

            let single = if add_single { s } else { "" };
            let double = d.repeat(num_double as _);

            write!(f, "{single}{double}")
        }
    }
}

impl fmt::Debug for AccidentalSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num = match self.offset.abs() {
            0 | 1 => "".to_owned(),
            2 => "Double".to_owned(),
            3 => "Triple".to_owned(),
            4 => "Quadruple".to_owned(),
            5 => "Quintuple".to_owned(),
            n => format!("({n}x)"),
        };

        let ty = match self.offset.signum() {
            0 => "Natural",
            1 => "Sharp",
            -1 => "Flat",
            _ => unreachable!(".signum() only returns -1, 0, 1")
        };

        write!(f, "{num}{ty}")
    }
}

impl fmt::Display for AccidentalSign {
    /// Formats the accidental using ASCII symbols.
    ///
    /// Uses standard ASCII notation. Multiple accidentals are combined,
    /// like `#x` for a triple sharp.
    ///
    /// For Unicode symbols, use [`display_unicode`](AccidentalSign::display_unicode).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::AccidentalSign;
    /// assert_eq!(format!("{}", AccidentalSign::NATURAL), "n");
    /// assert_eq!(format!("{}", AccidentalSign::SHARP), "#");
    /// assert_eq!(format!("{}", AccidentalSign::FLAT), "b");
    /// assert_eq!(format!("{}", AccidentalSign::DOUBLE_SHARP), "x");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_symbols(f, "n", "#", "b", "x", "bb")
    }
}

/// Wrapper for formatting [`AccidentalSign`] using Unicode musical symbols.
///
/// Obtained via [`AccidentalSign::display_unicode`].
pub struct DisplayUnicode(AccidentalSign);

impl fmt::Display for DisplayUnicode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_with_symbols(f, "♮", "♯", "♭", "𝄪", "𝄫")
    }
}

/// Error returned when parsing an [`AccidentalSign`] from [`&str`](prim@str) fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("The provided &str could not be converted into a AccidentalSign")]
pub struct ParseAccidentalError;

impl FromStr for AccidentalSign {
    type Err = ParseAccidentalError;

    /// Parses an accidental sign from a string.
    ///
    /// Accepts both ASCII (`n`, `#`, `b`, `x`, `bb`) and Unicode (`♮`, `♯`, `♭`, `𝄪`, `𝄫`) symbols.
    /// Multiple accidentals can be combined. Mixing sharps and flats is not allowed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::AccidentalSign;
    /// assert_eq!("#".parse(), Ok(AccidentalSign::SHARP));
    /// assert_eq!("♭".parse(), Ok(AccidentalSign::FLAT));
    /// assert_eq!("bb".parse(), Ok(AccidentalSign::DOUBLE_FLAT));
    ///
    /// // Triple sharp
    /// assert_eq!("#x".parse(), Ok(AccidentalSign { offset: 3 }));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseAccidentalError);
        }

        if matches!(s, "n" | "♮") {
            return Ok(Self::NATURAL);
        }

        let mut offset = 0i16;

        for c in s.chars() {
            match c {
                '+' | '#' | '♯' if offset >= 0 => offset += 1,
                'x' | '𝄪' if offset >= 0 => offset += 2,
                '-' | 'b' | '♭' if offset <= 0 => offset -= 1,
                '𝄫' if offset <= 0 => offset -= 2,
                _ if is_accidental_char(c) => unreachable!("all accidental chars should be handled"),
                _ => return Err(ParseAccidentalError),
            }
        }

        Ok(Self { offset })
    }
}

/// Checks if this char can be parsed as an accidental
pub(crate) fn is_accidental_char(c: char) -> bool {
    matches!(c, '+' | '#' | '♯' | 'x' | '𝄪' | '-' | 'b' | '♭' | '𝄫')
}

impl From<Semitones> for AccidentalSign {
    /// Converts a semitone offset into an accidental sign.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// let sharp: AccidentalSign = Semitones(1).into();
    /// assert_eq!(sharp, AccidentalSign::SHARP);
    ///
    /// let double_flat: AccidentalSign = Semitones(-2).into();
    /// assert_eq!(double_flat, AccidentalSign::DOUBLE_FLAT);
    /// ```
    fn from(value: Semitones) -> Self {
        Self::from_offset_semitones(value)
    }
}

impl From<AccidentalSign> for Semitones {
    /// Converts an accidental sign into its semitone offset.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{AccidentalSign, Semitones};
    /// let semitones: Semitones = AccidentalSign::SHARP.into();
    /// assert_eq!(semitones, Semitones(1));
    ///
    /// let semitones: Semitones = AccidentalSign::DOUBLE_FLAT.into();
    /// assert_eq!(semitones, Semitones(-2));
    /// ```
    fn from(value: AccidentalSign) -> Self {
        value.offset_semitones()
    }
}