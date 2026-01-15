use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::harmony::Key;
use crate::interval::Interval;
use crate::pitch::{Pitch, PitchClass, Spelling};
use crate::semitone::Semitone;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Note {
    pub pitch: Pitch,
    pub octave: i16,
}

impl Note {
    pub const MIDDLE_C: Self = Self { pitch: Pitch::C, octave: 4 };
    pub const A4: Self = Self { pitch: Pitch::A, octave: 4 };
    
    pub fn new(pitch: Pitch, octave: i16) -> Self {
        Self { pitch, octave }
    }
    
    pub fn semitones_to(self, other: Self) -> Semitone {
        let lhs = self.pitch.semitones_offset_from_c() + Semitone(self.octave * 12);

        let rhs = other.pitch.semitones_offset_from_c() + Semitone(other.octave * 12);

        rhs - lhs
    }

    pub fn distance_to(self, other: Self) -> Interval {
        Interval::between_notes(self, other)
    }
    
    // TODO: this includes spelling, when it probably shouldn't
    pub fn from_frequency_hz(hz: f32) -> Option<Self> {
        if hz <= 0.0 || !hz.is_finite() {
            return None;
        }

        let semitones_from_a4 = 12.0 * (hz / 440.0).log2();

        if !semitones_from_a4.is_finite() {
            return None;
        }

        let semitones_from_c0 = semitones_from_a4.round() as i16 + 9 + 4 * 12;

        let octave = semitones_from_c0.div_euclid(12);

        let pitch = semitones_from_c0.rem_euclid(12)
            .try_into()
            .expect("i32::rem_euclid(12) must be within [0,12)");

        let pitch = PitchClass::from_chroma(pitch)
            .expect("i32::rem_euclid(12) must be within [0,12)");

        Some( Self { pitch: Pitch::from(pitch), octave } )
    }

    pub fn as_frequency_hz(self) -> f32 {
        let semitones_from_a4 = Self::A4.semitones_to(self);

        440.0 * 2.0_f32.powf(semitones_from_a4.0 as f32 / 12.0)
    }

    pub fn transpose(self, interval: Interval) -> Self {
        let unchecked = Self {
            pitch: self.pitch + interval,
            octave: self.octave,
        };

        let edit = self.semitones_to(unchecked) - interval.semitones();

        Self {
            octave: unchecked.octave - edit.0.div_euclid(12),
            .. unchecked
        }
    }

    /// Only for use with spelling methods.
    #[inline(always)]
    fn respelled_as(self, respelled: Pitch) -> Self {
        assert!(
            self.pitch.eq_enharmonic(&respelled),
            "should only be called with enharmonic notes!",
        );

        let unchecked = Self { pitch: respelled, ..self };

        let octave_offset = {
            let semis = self.semitones_to(unchecked).0;

            debug_assert_eq!(
                semis % 12, 0,
                "should always be multiple of octave",
            );

            semis.div_euclid(12)
        };

        Self {
            octave: unchecked.octave - octave_offset,
            .. unchecked
        }
    }

    /// Returns the same note spelled with either [sharps](Spelling::Sharps) or [flats](Spelling::Flats).
    ///
    /// If the note is already spelled with the given spelling, *it is returned unchanged*,
    /// even if it can be written in a simpler way. For example spelling `G##4` with `sharps`
    /// will return `G##4`, not `A4`. If you'd like it to return `A4` instead, consider using
    /// [`Self::simplified`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Spell a note with flats
    /// assert_eq!(
    ///     Note::new(Pitch::A_SHARP, 4).respell_with(Spelling::Flats),
    ///     Note::new(Pitch::B_FLAT, 4),
    /// );
    ///
    /// // ... or with sharps
    /// assert_eq!(
    ///     Note::new(Pitch::E_FLAT, 4).respell_with(Spelling::Sharps),
    ///     Note::new(Pitch::D_SHARP, 4),
    /// );
    ///
    /// // Does nothing if a note with sharps is called with sharps
    /// assert_eq!(
    ///     Note::new(Pitch::C_SHARP, 4).respell_with(Spelling::Sharps),
    ///     Note::new(Pitch::C_SHARP, 4),
    /// );
    ///
    /// // Octave is adjusted when respelling crosses octave boundaries
    /// assert_eq!(
    ///     Note::new(Pitch::C_FLAT, 4).respell_with(Spelling::Sharps),
    ///     Note::new(Pitch::B, 3),
    /// );
    /// assert_eq!(
    ///     Note::new(Pitch::B_SHARP, 3).respell_with(Spelling::Flats),
    ///     Note::new(Pitch::C, 4),
    /// );
    ///
    /// // This will not simplify notes if they're already spelled as intended
    /// assert_eq!(
    ///     Note::new(Pitch::G_DOUBLE_SHARP, 4).respell_with(Spelling::Sharps),
    ///     Note::new(Pitch::G_DOUBLE_SHARP, 4),
    /// );
    /// ```
    pub fn respell_with(self, spelling: Spelling) -> Self {
        self.respelled_as(self.pitch.respell_with(spelling))
    }

    /// Returns the same note with fewer accidentals.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(
    ///     Note::new(Pitch::C_FLAT, 4).simplified(),
    ///     Note::new(Pitch::B, 3),
    /// );
    /// assert_eq!(
    ///     Note::new(Pitch::F_DOUBLE_SHARP, 4).simplified(),
    ///     Note::new(Pitch::G, 4),
    /// );
    ///
    /// // Already simplified notes are not further simplified
    /// assert_eq!(
    ///     Note::new(Pitch::G_FLAT, 4).simplified(),
    ///     Note::new(Pitch::G_FLAT, 4),
    /// );
    /// assert_eq!(
    ///     Note::new(Pitch::G, 4).simplified(),
    ///     Note::new(Pitch::G, 4),
    /// );
    /// ```
    pub fn simplified(self) -> Self {
        self.respelled_as(self.pitch.simplified())
    }

    /// Returns the note's enharmonic.
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(
    ///     Note::new(Pitch::C_SHARP, 4).enharmonic(),
    ///     Note::new(Pitch::D_FLAT, 4),
    /// );
    /// assert_eq!(
    ///     Note::new(Pitch::C_FLAT, 4).enharmonic(),
    ///     Note::new(Pitch::B, 3),
    /// );
    ///
    /// // Notes that can be written with no accidentals will be written
    /// // with no accidentals. As such, notes with no accidentals will
    /// // return themselves.
    /// assert_eq!(
    ///     Note::new(Pitch::G, 4).enharmonic(),
    ///     Note::new(Pitch::G, 4),
    /// );
    /// assert_eq!(
    ///     Note::new(Pitch::B_DOUBLE_FLAT, 4).enharmonic(),
    ///     Note::new(Pitch::A, 4),
    /// );
    /// ```
    pub fn enharmonic(self) -> Self {
        self.respelled_as(self.pitch.enharmonic())
    }

    /// Respells this note according to the key signature.
    ///
    /// Corrects the spelling of notes diatonic to the key (notes that appear in the key's scale)
    /// to match the key signature. Notes not diatonic to the key preserve original spelling.
    ///
    /// If you don't want non-diatonic notes to preserve spelling, see the documentation
    /// example for this method.
    ///
    /// See [`Pitch::respell_in_key`] for more.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let cs_major = Key::major(Pitch::C_SHARP);
    ///
    /// // Diatonic notes are respelled to match the key
    /// assert_eq!(
    ///     Note::new(Pitch::C, 4).respell_in_key(cs_major),
    ///     Note::new(Pitch::B_SHARP, 3),
    /// );
    ///
    /// let d_major = Key::major(Pitch::D);
    ///
    /// // Notes that aren't diatonic preserve spelling
    /// assert_eq!(
    ///     Note::new(Pitch::B_FLAT, 4).respell_in_key(d_major),
    ///     Note::new(Pitch::B_FLAT, 4),
    /// );
    ///
    /// // ... but if you don't want this behavior, respell it
    /// assert_eq!(
    ///     Note::new(Pitch::B_FLAT, 4).respell_in_key(d_major)
    ///         .respell_with(d_major.spelling().unwrap_or_default()),
    ///     Note::new(Pitch::A_SHARP, 4),
    /// );
    /// ```
    pub fn respell_in_key(self, key: Key) -> Self {
        self.respelled_as(self.pitch.respell_in_key(key))
    }

    pub fn as_midi(self) -> Option<u8> {
        let zero = Note { pitch: Pitch::C, octave: -1 };

        zero.semitones_to(self)
            .0
            .try_into()
            .ok()
    }
    
    pub fn as_midi_strict(self) -> Option<u8> {
        self.as_midi().filter(|&m| m < 128)
    }

    pub fn from_midi(midi: u8) -> Note {
        let pitch = PitchClass::from_chroma(midi % 12)
            .expect("% 12 must be [0, 11]");
        let oct = midi / 12;

        Self {
            pitch: pitch.into(),
            octave: oct as i16 - 1,
        }
    }

    pub fn transpose_fifths(self, fifths: i16) -> Self {
        let unchecked = Self {
            pitch: self.pitch.transpose_fifths(fifths),
            octave: self.octave,
        };

        let interval_semi = Semitone(7 * fifths);

        let edit = self.semitones_to(unchecked) - interval_semi;

        Self {
            octave: unchecked.octave - edit.0.div_euclid(12),
            .. unchecked
        }
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Note {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.octave
            .cmp(&rhs.octave)
            .then(
                self.pitch.cmp(&rhs.pitch)
            )
    }
}

impl EnharmonicOrd for Note {
    fn cmp_enharmonic(&self, rhs: &Self) -> Ordering {
        let lhs = self.simplified();
        let rhs = rhs.simplified();

        lhs.octave
            .cmp(&rhs.octave)
            .then(
                lhs.pitch.as_pitch_class().cmp(&rhs.pitch.as_pitch_class())
            )
    }
}

impl EnharmonicEq for Note {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.semitones_to(*rhs).0 == 0
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch, self.octave)
    }
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        note.pitch
    }
}


impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Interval> for Note {
    type Output = Self;

    fn sub(self, rhs: Interval) -> Self::Output {
        self + (-rhs)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn midi() {
        for n in 0..=255u8 {
            assert_eq!(Some(n), Note::from_midi(n).as_midi())
        }
    }

    #[test]
    fn test_respell_with() {
        let cases = [
            (Note::new(Pitch::A_SHARP, 4), Spelling::Flats, Note::new(Pitch::B_FLAT, 4)),
            (Note::new(Pitch::E_FLAT, 4), Spelling::Sharps, Note::new(Pitch::D_SHARP, 4)),

            // already correct
            (Note::new(Pitch::C_SHARP, 4), Spelling::Sharps, Note::new(Pitch::C_SHARP, 4)),
            (Note::new(Pitch::D_FLAT, 4), Spelling::Flats, Note::new(Pitch::D_FLAT, 4)),

            // octave boundary
            (Note::new(Pitch::C_FLAT, 4), Spelling::Sharps, Note::new(Pitch::B, 3)),
            (Note::new(Pitch::B_SHARP, 3), Spelling::Flats, Note::new(Pitch::C, 4)),
            (Note::new(Pitch::C_FLAT, 5), Spelling::Sharps, Note::new(Pitch::B, 4)),
            (Note::new(Pitch::B_SHARP, 5), Spelling::Flats, Note::new(Pitch::C, 6)),

            // naturals
            (Note::new(Pitch::C, 4), Spelling::Sharps, Note::new(Pitch::C, 4)),
            (Note::new(Pitch::G, 5), Spelling::Flats, Note::new(Pitch::G, 5)),
        ];

        for (input, spelling, expected) in cases {
            assert_eq!(
                input.respell_with(spelling),
                expected,
                "{input} respelled with {spelling:?} should be {expected}"
            );
        }
    }

    #[test]
    fn test_simplified() {
        let cases = [
            // single accidentals
            (Note::new(Pitch::C_SHARP, 4), Note::new(Pitch::C_SHARP, 4)),
            (Note::new(Pitch::D_FLAT, 5), Note::new(Pitch::D_FLAT, 5)),

            // double accidentals
            (Note::new(Pitch::C_DOUBLE_SHARP, 4), Note::new(Pitch::D, 4)),
            (Note::new(Pitch::D_DOUBLE_FLAT, 4), Note::new(Pitch::C, 4)),

            // octave boundary
            (Note::new(Pitch::C_FLAT, 4), Note::new(Pitch::B, 3)),
            (Note::new(Pitch::B_SHARP, 3), Note::new(Pitch::C, 4)),
            (Note::new(Pitch::F_FLAT, 6), Note::new(Pitch::E, 6)),
            (Note::new(Pitch::E_SHARP, 2), Note::new(Pitch::F, 2)),

            // naturals
            (Note::new(Pitch::C, 4), Note::new(Pitch::C, 4)),
            (Note::new(Pitch::G, 5), Note::new(Pitch::G, 5)),
        ];

        for (input, expected) in cases {
            assert_eq!(
                input.simplified(),
                expected,
                "{input} simplified should be {expected}"
            );
        }
    }

    #[test]
    fn test_enharmonic() {
        let cases = [
            (Note::new(Pitch::C_SHARP, 4), Note::new(Pitch::D_FLAT, 4)),
            (Note::new(Pitch::D_FLAT, 4), Note::new(Pitch::C_SHARP, 4)),
            (Note::new(Pitch::F_SHARP, 5), Note::new(Pitch::G_FLAT, 5)),

            // double accidentals
            (Note::new(Pitch::A_DOUBLE_FLAT, 4), Note::new(Pitch::G, 4)),
            (Note::new(Pitch::B_DOUBLE_SHARP, 5), Note::new(Pitch::D_FLAT, 6)),

            // octave boundary
            (Note::new(Pitch::C_FLAT, 4), Note::new(Pitch::B, 3)),
            (Note::new(Pitch::B_SHARP, 3), Note::new(Pitch::C, 4)),

            // Naturals stay natural
            (Note::new(Pitch::C, 4), Note::new(Pitch::C, 4)),
            (Note::new(Pitch::G, 5), Note::new(Pitch::G, 5)),
        ];

        for (input, expected) in cases {
            assert_eq!(
                input.enharmonic(),
                expected,
                "{input} enharmonic should be {expected}"
            );
        }
    }

    #[test]
    fn test_respell_in_key() {
        let cases = [
            // diatonic notes
            (Note::new(Pitch::C, 4), Key::major(Pitch::C_SHARP), Note::new(Pitch::B_SHARP, 3)),
            (Note::new(Pitch::F, 4), Key::major(Pitch::C_SHARP), Note::new(Pitch::E_SHARP, 4)),

            (Note::new(Pitch::E, 4), Key::major(Pitch::C_FLAT), Note::new(Pitch::F_FLAT, 4)),
            (Note::new(Pitch::B, 3), Key::major(Pitch::C_FLAT), Note::new(Pitch::C_FLAT, 4)),

            (Note::new(Pitch::A_SHARP, 4), Key::major(Pitch::F), Note::new(Pitch::B_FLAT, 4)),

            // chromatic notes
            (Note::new(Pitch::C_SHARP, 4), Key::major(Pitch::F), Note::new(Pitch::C_SHARP, 4)),
            (Note::new(Pitch::E_FLAT, 5), Key::major(Pitch::D), Note::new(Pitch::E_FLAT, 5)),

            // correctly spelled
            (Note::new(Pitch::C_SHARP, 4), Key::major(Pitch::C_SHARP), Note::new(Pitch::C_SHARP, 4)),
            (Note::new(Pitch::D, 4), Key::major(Pitch::C), Note::new(Pitch::D, 4)),
        ];

        for (input, key, expected) in cases {
            assert_eq!(
                input.respell_in_key(key),
                expected,
                "{input} respelled in {key:?} should be {expected}"
            );
        }
    }
}