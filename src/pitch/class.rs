use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use strum_macros::{EnumIter, FromRepr};
use crate::enharmonic::{self, EnharmonicEq, EnharmonicOrd, WithoutSpelling};
use crate::interval::Interval;
use crate::pitch::{Pitch, Letter, AccidentalSign, PitchFromStrError, Spelling};
use crate::prelude::Key;
use crate::semitone::Semitones;

/// A pitch class representing one of the twelve chromatic pitches.
///
/// Pitch classes represent the twelve pitches in an octave, without spelling or octave information.
/// For example, the pitches [C#](Pitch::C_SHARP) and [Db](Pitch::D_FLAT) are both pitch class [Cs](PitchClass::Cs).
///
/// Since pitch classes are spelling agnostic, to convert a pitch class to a specific spelling,
/// use [`spell_with`](Self::spell_with) or [`spell_in_key`](Self::spell_in_key).
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// // Pitch classes can be transposed by intervals
/// assert_eq!(PitchClass::C + Interval::MAJOR_THIRD, PitchClass::E);
///
/// // ... or by semitones
/// assert_eq!(PitchClass::F + Semitones(1), PitchClass::Fs);
///
/// // Spell pitch classes with sharps or flats
/// assert_eq!(PitchClass::Cs.spell_with(Spelling::Sharps), Pitch::C_SHARP);
/// assert_eq!(PitchClass::Cs.spell_with(Spelling::Flats), Pitch::D_FLAT);
///
/// assert_eq!(
///     Pitch::F_SHARP.as_pitch_class(),
///     Pitch::G_FLAT.as_pitch_class(),
///     "both are PitchClass::Fs",
/// );
/// # assert_eq!(Pitch::G_FLAT.as_pitch_class(), PitchClass::Fs)
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, FromRepr, EnumIter, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PitchClass {
    /// C / B#
    C = 0,
    /// C# / Db
    Cs,
    /// D
    D,
    /// D# / Eb
    Ds,
    /// E / Fb
    E,
    /// F / E#
    F,
    /// F# / Gb
    Fs,
    /// G
    G,
    /// G# / Ab
    Gs,
    /// A
    A,
    /// A# / Bb
    As,
    /// B / Cb
    B,
}

impl PitchClass {
    /// Transposes this pitch class by the given interval.
    ///
    /// This is equivalent to using the [+ operator](Add::add).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// let a = PitchClass::A;
    /// assert_eq!(a.transpose(Interval::MAJOR_THIRD), PitchClass::Cs);
    /// assert_eq!(a.transpose(Interval::PERFECT_FIFTH), PitchClass::E);
    ///
    /// // Equivalent to the '+' operator
    /// assert_eq!(PitchClass::D + Interval::MINOR_THIRD, PitchClass::F);
    /// ```
    pub fn transpose(&self, interval: Interval) -> Self {
        *self + interval.semitones()
    }

    /// Returns the letter of this pitch class.
    ///
    /// For pitch classes with accidentals, returns the letter used in the sharp spelling.
    /// For example, `Cs` returns [`Letter::C`], not [`Letter::D`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::Ds.letter(), Letter::D);
    /// assert_eq!(PitchClass::E.letter(), Letter::E);
    /// ```
    pub fn letter(self) -> Letter {
        use PitchClass as PC;
        use Letter as L;

        match self {
            PC::C | PC::Cs => L::C,
            PC::D | PC::Ds => L::D,
            PC::E => L::E,
            PC::F | PC::Fs => L::F,
            PC::G | PC::Gs => L::G,
            PC::A | PC::As => L::A,
            PC::B => L::B,
        }
    }

    /// Returns the accidental of this pitch class.
    ///
    /// Pitch classes without accidentals return [`AccidentalSign::NATURAL`], and those with
    /// accidentals always return [`AccidentalSign::SHARP`].
    ///
    /// [`AccidentalSign::FLAT`] is never returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::A.accidental(), AccidentalSign::NATURAL);
    /// assert_eq!(PitchClass::Fs.accidental(), AccidentalSign::SHARP);
    /// ```
    pub fn accidental(self) -> AccidentalSign {
        use PitchClass as PC;

        match self {
            PC::Cs | PC::Ds | PC::Fs | PC::Gs | PC::As => AccidentalSign::SHARP,
            _ => AccidentalSign::NATURAL,
        }
    }

    /// Creates a pitch class from a chroma, in `[0, 11]`.
    ///
    /// Returns `None` if the chroma value is greater than 11.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::from_chroma(4), Some(PitchClass::E));
    /// assert_eq!(PitchClass::from_chroma(11), Some(PitchClass::B));
    /// assert_eq!(PitchClass::from_chroma(12), None);
    /// ```
    #[inline(always)]
    pub const fn from_chroma(chroma: u8) -> Option<Self> {
        Self::from_repr(chroma)
    }

    /// Returns the chroma value of this pitch class, in `[0, 11]`.
    ///
    /// [`PitchClass::C`] has chroma `0`, and counts up until [`PitchClass::B`] with chroma `11`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::C.chroma(), 0);
    /// assert_eq!(PitchClass::Cs.chroma(), 1);
    /// assert_eq!(PitchClass::B.chroma(), 11);
    /// ```
    pub fn chroma(self) -> u8 {
        self as u8
    }

    /// Returns how many semitones `rhs` is from `self`. Always positive, in `[0,11]`.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::C.semitones_to(PitchClass::E), Semitones(4));
    /// assert_eq!(PitchClass::Fs.semitones_to(PitchClass::B), Semitones(5));
    /// // Wraps around the octave
    /// assert_eq!(PitchClass::B.semitones_to(PitchClass::D), Semitones(3));
    /// ```
    pub fn semitones_to(self, rhs: Self) -> Semitones {
        Semitones((rhs.chroma() as i16 - self.chroma() as i16).rem_euclid(12))
    }

    /// Returns the pitch class spelled with either [sharps](Spelling::Sharps) or [flats](Spelling::Flats).
    ///
    /// Pitch classes have no spelling, and can either be spelled with sharps or flats.
    /// Naturals are always spelled as naturals.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Spell a pitch class with flats
    /// assert_eq!(PitchClass::Cs.spell_with(Spelling::Flats), Pitch::D_FLAT);
    /// // ... or with sharps
    /// assert_eq!(PitchClass::Cs.spell_with(Spelling::Sharps), Pitch::C_SHARP);
    ///
    /// // Natural pitch classes are always spelled as naturals
    /// assert_eq!(PitchClass::C.spell_with(Spelling::Flats), Pitch::C);
    /// ```
    pub fn spell_with(self, spelling: Spelling) -> Pitch {
        if self.accidental() == AccidentalSign::NATURAL || spelling == Spelling::Sharps {
            self.into()
        } else {
            let base = Self::from_chroma(self as u8 + 1)
                .expect("must be <= 11")
                .letter();

            Pitch::from_letter_and_accidental(base, AccidentalSign::FLAT)
        }
    }

    /// Spells this pitch class in the context of a musical key.
    ///
    /// This method checks if the pitch class appears in the key's scale and returns
    /// the proper spelling used in that scale. For chromatic notes not in the scale,
    /// it falls back to the key's spelling preference (sharps or flats).
    ///
    /// For pitch classes within the key, returns the spelling used in that key's scale.
    /// For pitch classes not in the key, spells according to the key's preference: keys with
    /// sharps (g# minor) spell with sharps, keys with flats (Ab major) spell chromatics with flats.
    /// Keys with no alterations (C major, A minor) default to sharps for chromatic notes.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Cb major: Cb, Db, Eb, *Fb*, Gb, Ab, Bb
    /// let cb_major = Key::major(Pitch::C_FLAT);
    /// assert_eq!(PitchClass::E.spell_in_key(cb_major), Pitch::F_FLAT);
    ///
    /// // C# major: C#, D#, *E#*, F#, G#, A#, B#
    /// let cs_major = Key::major(Pitch::C_SHARP);
    /// assert_eq!(PitchClass::F.spell_in_key(cs_major), Pitch::E_SHARP);
    ///
    /// // Chromatic notes use the key's spelling preference
    /// let gb_major = Key::major(Pitch::G_FLAT);
    /// assert_eq!(PitchClass::Ds.spell_in_key(gb_major), Pitch::E_FLAT);
    /// assert_eq!(PitchClass::Ds.spell_in_key(cs_major), Pitch::D_SHARP);
    ///
    /// // C major has no alterations; chromatics default to sharps
    /// assert_eq!(PitchClass::Cs.spell_in_key(Key::major(Pitch::C)), Pitch::C_SHARP);
    /// ```
    pub fn spell_in_key(self, key: Key) -> Pitch {
        if let Some(pitch) = key.scale_experimental()
            .build_default()
            .into_iter()
            .find(|p| p.as_pitch_class() == self)
        {
            return pitch;
        }

        self.spell_with(key.spelling().unwrap_or_default())
    }
}

impl WithoutSpelling for PitchClass {
    type Unspelled = Self;

    fn without_spelling(self) -> Self::Unspelled {
        self
    }
}

impl EnharmonicEq for PitchClass {
    fn eq_enharmonic(&self, other: &Self) -> bool {
        enharmonic::defer_without_spelling::eq(self, other)
    }
}

impl EnharmonicOrd for PitchClass {
    fn cmp_enharmonic(&self, other: &Self) -> Ordering {
        enharmonic::defer_without_spelling::cmp(self, other)
    }
}

impl Add<Semitones> for PitchClass {
    type Output = PitchClass;

    /// Transposes a pitch class by a number of semitones.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::C + Semitones(4), PitchClass::E);
    /// assert_eq!(PitchClass::G + Semitones(5), PitchClass::C);
    /// ```
    fn add(self, rhs: Semitones) -> Self::Output {
        let pitch = (self as u8 as i16 + rhs.0)
            .rem_euclid(12)
            .try_into()
            .expect("must be between [0,11] since did % 12");

        Self::from_chroma(pitch)
            .expect("must be between [0,11] since did % 12")
    }
}

impl Sub<Semitones> for PitchClass {
    type Output = PitchClass;

    /// Transposes a pitch class down by a number of semitones.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::E - Semitones(4), PitchClass::C);
    /// assert_eq!(PitchClass::D - Semitones(3), PitchClass::B);
    /// ```
    fn sub(self, rhs: Semitones) -> Self::Output {
        self + (-rhs)
    }
}

impl FromStr for PitchClass {
    type Err = PitchFromStrError;

    /// Parses a pitch class from a [`&str`](prim@str).
    ///
    /// Accepts pitch notation with optional accidentals (e.g., "C", "F#", "Bb").
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!("F#".parse::<PitchClass>(), Ok(PitchClass::Fs));
    /// assert_eq!("Db".parse::<PitchClass>(), Ok(PitchClass::Cs));
    ///
    /// // Octave numbers aren't allowed
    /// assert_eq!("C4".parse::<PitchClass>(), Err(PitchFromStrError));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pitch::from_str(s)?.into())
    }
}

impl fmt::Display for PitchClass {
    /// Formats the pitch class using its default sharp spelling.
    ///
    /// Natural pitch classes display as their letter, chromatic pitch classes
    /// display with a sharp symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::C.to_string(), "C");
    /// assert_eq!(PitchClass::Cs.to_string(), "C♯");
    /// assert_eq!(PitchClass::E.to_string(), "E");
    /// assert_eq!(PitchClass::Fs.to_string(), "F♯");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pitch = Pitch::from(*self);
        write!(f, "{pitch}")
    }
}

impl Add<Interval> for PitchClass {
    type Output = Self;

    /// Transposes a pitch class by an interval.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::C + Interval::MAJOR_THIRD, PitchClass::E);
    /// assert_eq!(PitchClass::F + Interval::PERFECT_FIFTH, PitchClass::C);
    /// ```
    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Interval> for PitchClass {
    type Output = Self;

    /// Transposes a pitch class down by an interval.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(PitchClass::E - Interval::MAJOR_THIRD, PitchClass::C);
    /// assert_eq!(PitchClass::C - Interval::PERFECT_FIFTH, PitchClass::F);
    /// ```
    fn sub(self, rhs: Interval) -> Self::Output {
        self + (-rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_spell_with() {
        for pc in PitchClass::iter() {
            let spell_sharps = pc.spell_with(Spelling::Sharps);
            let spell_flats = pc.spell_with(Spelling::Flats);

            assert_eq!(
                spell_sharps.as_pitch_class(), pc,
                "spell with should return an enharmonic"
            );

            assert!(
                matches!(spell_sharps.accidental(), AccidentalSign::NATURAL | AccidentalSign::SHARP),
                "spelling with sharps should be natural or sharp, got {}", spell_sharps.accidental()
            );

            assert_eq!(
                spell_flats.as_pitch_class(), pc,
                "spell with should return an enharmonic"
            );

            assert!(
                matches!(spell_flats.accidental(), AccidentalSign::NATURAL | AccidentalSign::FLAT),
                "spelling with flats should be natural or flat, got {}", spell_flats.accidental()
            );
        }
    }

    #[test]
    fn test_spell_in_key() {
        let cases = [
            // sharp keys
            (Key::major(Pitch::C_SHARP), PitchClass::F, Pitch::E_SHARP),
            (Key::major(Pitch::F_SHARP), PitchClass::F, Pitch::E_SHARP),
            (Key::major(Pitch::C_SHARP), PitchClass::C, Pitch::B_SHARP),

            // flat keys
            (Key::major(Pitch::C_FLAT), PitchClass::E, Pitch::F_FLAT),
            (Key::major(Pitch::C_FLAT), PitchClass::B, Pitch::C_FLAT),
            (Key::major(Pitch::G_FLAT), PitchClass::B, Pitch::C_FLAT),
            (Key::major(Pitch::G_FLAT), PitchClass::F, Pitch::F),

            // C major naturals
            (Key::major(Pitch::C), PitchClass::C, Pitch::C),
            (Key::major(Pitch::C), PitchClass::Cs, Pitch::C_SHARP),
            (Key::major(Pitch::C), PitchClass::D, Pitch::D),
            (Key::major(Pitch::C), PitchClass::Ds, Pitch::D_SHARP),
            (Key::major(Pitch::C), PitchClass::E, Pitch::E),

            // diatonic notes in various keys
            (Key::major(Pitch::G), PitchClass::Fs, Pitch::F_SHARP),
            (Key::major(Pitch::D), PitchClass::Fs, Pitch::F_SHARP),
            (Key::major(Pitch::D), PitchClass::Cs, Pitch::C_SHARP),
            (Key::major(Pitch::F), PitchClass::As, Pitch::B_FLAT),
            (Key::major(Pitch::B_FLAT), PitchClass::As, Pitch::B_FLAT),
            (Key::major(Pitch::B_FLAT), PitchClass::Ds, Pitch::E_FLAT),

            // chromatic notes in sharp keys - should use sharps
            (Key::major(Pitch::G), PitchClass::Cs, Pitch::C_SHARP),
            (Key::major(Pitch::G), PitchClass::Ds, Pitch::D_SHARP),
            (Key::major(Pitch::D), PitchClass::Gs, Pitch::G_SHARP),

            // chromatic notes in flat keys - should use flats
            (Key::major(Pitch::F), PitchClass::Ds, Pitch::E_FLAT),
            (Key::major(Pitch::F), PitchClass::Gs, Pitch::A_FLAT),
            (Key::major(Pitch::B_FLAT), PitchClass::Cs, Pitch::D_FLAT),

            // minor keys
            (Key::minor(Pitch::A), PitchClass::A, Pitch::A),
            (Key::minor(Pitch::A), PitchClass::B, Pitch::B),
            (Key::minor(Pitch::A), PitchClass::C, Pitch::C),
            (Key::minor(Pitch::E), PitchClass::Fs, Pitch::F_SHARP),
            (Key::minor(Pitch::D), PitchClass::As, Pitch::B_FLAT),
            (Key::minor(Pitch::C_SHARP), PitchClass::Fs, Pitch::F_SHARP),
            (Key::minor(Pitch::C_SHARP), PitchClass::Cs, Pitch::C_SHARP),
            (Key::minor(Pitch::C_SHARP), PitchClass::Gs, Pitch::G_SHARP),
            (Key::minor(Pitch::C_SHARP), PitchClass::Ds, Pitch::D_SHARP),

            // G## major: G##, A##, B##, C##, D##, E##, F###
            (Key::major(Pitch::G_DOUBLE_SHARP), PitchClass::A, Pitch::G_DOUBLE_SHARP),
            (Key::major(Pitch::G_DOUBLE_SHARP), PitchClass::B, Pitch::A_DOUBLE_SHARP),
            (Key::major(Pitch::G_DOUBLE_SHARP), PitchClass::Cs, Pitch::B_DOUBLE_SHARP),
            (Key::major(Pitch::G_DOUBLE_SHARP), PitchClass::D, Pitch::C_DOUBLE_SHARP),
            (Key::major(Pitch::G_DOUBLE_SHARP), PitchClass::E, Pitch::D_DOUBLE_SHARP),
            (Key::major(Pitch::G_DOUBLE_SHARP), PitchClass::Fs, Pitch::E_DOUBLE_SHARP),
            (
                Key::major(Pitch::G_DOUBLE_SHARP),
                PitchClass::Gs,
                Pitch::from_letter_and_accidental(
                    Letter::F,
                    AccidentalSign { offset: 3 }
                )
            ),
        ];

        for (key, pc, expected) in cases {
            assert_eq!(
                pc.spell_in_key(key),
                expected,
                "{pc} should be spelled {expected} in {key:?}",
            );
        }
    }
}
