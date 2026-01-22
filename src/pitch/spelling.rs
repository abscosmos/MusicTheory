use crate::AccidentalSign;

/// Spelling preference for pitches.
///
/// `Spelling` represents whether to prefer sharps or flats when spelling a
/// [`PitchClass`](crate::pitch::PitchClass) as a [`Pitch`](crate::pitch::Pitch).
///
/// For usage, see [`PitchClass::spell_with`](crate::pitch::PitchClass::spell_with)
/// or [`Pitch::respell_with`](crate::pitch::Pitch::respell_with).
/// # Examples
/// ```rust
/// # use music_theory::{Pitch, PitchClass};
/// # use music_theory::pitch::Spelling;
/// // D#/Eb spelled with sharps becomes D♯
/// assert_eq!(PitchClass::Ds.spell_with(Spelling::Sharps), Pitch::D_SHARP);
///
/// // ... and spelled with flats becomes Eb
/// assert_eq!(PitchClass::Ds.spell_with(Spelling::Flats), Pitch::E_FLAT);
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Spelling {
    /// Prefer sharp accidentals (#).
    #[default]
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
    /// # use music_theory::AccidentalSign;
    /// # use music_theory::pitch::Spelling;
    /// assert_eq!(
    ///     Spelling::from_accidental(AccidentalSign::SHARP),
    ///     Some(Spelling::Sharps)
    /// );
    ///
    /// assert_eq!(
    ///     Spelling::from_accidental(AccidentalSign::DOUBLE_FLAT),
    ///     Some(Spelling::Flats)
    /// );
    ///
    /// assert_eq!(Spelling::from_accidental(AccidentalSign::NATURAL), None);
    /// ```
    pub const fn from_accidental(acc: AccidentalSign) -> Option<Self> {
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
    /// # use music_theory::pitch::Spelling;
    /// assert_eq!(Spelling::Sharps.flip(), Spelling::Flats);
    /// assert_eq!(Spelling::Flats.flip(), Spelling::Sharps);
    /// ```
    pub const fn flip(self) -> Self {
        match self {
            Self::Sharps => Self::Flats,
            Self::Flats => Self::Sharps,
        }
    }
}