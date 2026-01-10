use std::fmt;
use serde::{Deserialize, Serialize};
use crate::semitone::Semitone;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AccidentalSign {
    pub offset: i16,
}

impl AccidentalSign {
    pub const DOUBLE_FLAT: Self = Self { offset: -2 };
    pub const FLAT: Self = Self { offset: -1 };
    pub const NATURAL: Self = Self { offset: 0 };
    pub const SHARP: Self = Self { offset: 1 };
    pub const DOUBLE_SHARP: Self = Self { offset: 2 };

    pub fn as_semitone_offset(self) -> Semitone {
        Semitone(self.offset)
    }

    pub fn from_semitone_offset(offset: Semitone) -> Self {
        Self { offset: offset.0 }
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offset = self.offset;

        if offset == 0 {
            write!(f, "♮")
        } else {
            let num_double = offset.abs() / 2;
            let add_single = offset.abs() % 2 == 1;

            let (d, s) = if offset > 0 {
                ("𝄪", "♯")
            } else {
                ("𝄫", "♭")
            };

            let single = if add_single { s } else { "" };
            let double = d.repeat(num_double as _);

            write!(f, "{single}{double}")
        }
    }
}

impl From<Semitone> for AccidentalSign {
    fn from(value: Semitone) -> Self {
        Self::from_semitone_offset(value)
    }
}

impl From<AccidentalSign> for Semitone {
    fn from(value: AccidentalSign) -> Self {
        value.as_semitone_offset()
    }
}

/// Spelling preference for pitches.
///
/// `Spelling` represents whether to prefer sharps or flats spelling a
/// [`PitchClass`](crate::pitch::PitchClass) as a [`Pitch`](crate::pitch::Pitch).
///
/// For usage, see [`PitchClass::spell_with`](crate::pitch::PitchClass::spell_with)
/// or [`Pitch::respell_with`](crate::pitch::Pitch::respell_with).
/// # Examples
/// ```rust
/// # use music_theory::prelude::*;
/// // D#/Eb spelled with sharps becomes D♯
/// assert_eq!(PitchClass::Ds.spell_with(Spelling::Sharps), Pitch::D_SHARP);
///
/// // ... and spelled with flats becomes Eb
/// assert_eq!(PitchClass::Ds.spell_with(Spelling::Flats), Pitch::E_FLAT);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Spelling {
    /// Prefer sharp accidentals (#).
    Sharps,
    /// Prefer flat accidentals (b).
    Flats,
}

impl Spelling {
    /// Gets spelling from an accidental sign.
    ///
    /// Returns `Some(Spelling::Sharps)` for sharps, `Some(Spelling::Flats)` for flats,
    /// and `None` for natural.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(
    ///     Spelling::of_accidental(AccidentalSign::SHARP),
    ///     Some(Spelling::Sharps)
    /// );
    ///
    /// assert_eq!(
    ///     Spelling::of_accidental(AccidentalSign::DOUBLE_FLAT),
    ///     Some(Spelling::Flats)
    /// );
    ///
    /// assert_eq!(Spelling::of_accidental(AccidentalSign::NATURAL), None);
    /// ```
    pub const fn of_accidental(acc: AccidentalSign) -> Option<Self> {
        match acc.offset {
            ..0 => Some(Self::Flats),
            0 => None,
            1.. => Some(Self::Sharps)
        }
    }

    /// Returns the opposite spelling preference.
    ///
    /// Converts [`Spelling::Sharps`] to [`Spelling::Flats`] and vice versa.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Spelling::Sharps.flip(), Spelling::Flats);
    /// assert_eq!(Spelling::Flats.flip(), Spelling::Sharps);
    /// ```
    pub const fn flip(self) -> Self {
        match self {
            Self::Sharps => Self::Flats,
            Self::Flats => Self::Sharps,
        }
    }

    /// Returns `true` if the spelling preference is [`Spelling::Sharps`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert!(Spelling::Sharps.is_sharps());
    /// assert!(!Spelling::Flats.is_sharps());
    /// ```
    pub const fn is_sharps(self) -> bool {
        matches!(self, Self::Sharps)
    }

    /// Returns `true` if the spelling preference is [`Spelling::Flats`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert!(Spelling::Flats.is_flats());
    /// assert!(!Spelling::Sharps.is_flats());
    /// ```
    pub const fn is_flats(self) -> bool {
        matches!(self, Self::Flats)
    }
}