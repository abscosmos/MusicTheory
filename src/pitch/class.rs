use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use strum_macros::{EnumIter, FromRepr};
use crate::enharmonic::EnharmonicEq;
use crate::interval::Interval;
use crate::pitch::{Pitch, Letter, AccidentalSign, PitchFromStrError, Spelling};
use crate::prelude::Key;
use crate::semitone::Semitone;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, FromRepr, EnumIter, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PitchClass {
    C = 0,  /* C /B# */
    Cs, /* C#/D♭ */
    D,
    Ds, /* D#/E♭ */
    E,  /* E /F♭ */
    F,  /* F /E♭ */
    Fs, /* F#/G♭ */
    G,
    Gs, /* G#/A♭ */
    A,
    As, /* A#/B♭ */
    B,  /* B /C♭ */
}

impl PitchClass {
    pub fn transpose(&self, interval: Interval) -> Self {
        *self + interval.semitones()
    }

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

    pub fn accidental(self) -> AccidentalSign {
        use PitchClass as PC;

        match self {
            PC::Cs | PC::Ds | PC::Fs | PC::Gs | PC::As => AccidentalSign::SHARP,
            _ => AccidentalSign::NATURAL,
        }
    }

    #[inline(always)]
    pub const fn from_chroma(chroma: u8) -> Option<Self> {
        Self::from_repr(chroma)
    }

    pub fn chroma(self) -> u8 {
        self as u8
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

    /// Spells pitch class in the context of a musical key.
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
    /// // C# major: C♯, D♯, *E♯*, F♯, G♯, A♯, B♯
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

impl EnharmonicEq for PitchClass {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl Add<Semitone> for PitchClass {
    type Output = PitchClass;

    fn add(self, rhs: Semitone) -> Self::Output {
        let pitch = (self as u8 as i16 + rhs.0)
            .rem_euclid(12)
            .try_into()
            .expect("must be between [0,11] since did % 12");

        Self::from_chroma(pitch)
            .expect("must be between [0,11] since did % 12")
    }
}

impl Sub<Semitone> for PitchClass {
    type Output = PitchClass;

    fn sub(self, rhs: Semitone) -> Self::Output {
        self + (-rhs)
    }
}

impl FromStr for PitchClass {
    type Err = PitchFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pitch::from_str(s)?.into())
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pitch = Pitch::from(*self);
        write!(f, "{pitch}")
    }
}

impl Add<Interval> for PitchClass {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Interval> for PitchClass {
    type Output = Self;

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
