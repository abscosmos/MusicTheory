//! Musical notes with pitch and octave.
//!
//! See documentation for [`Note`] for more information.
//!
//! # Examples
//! ```
//! # use music_theory::prelude::*;
//! let c4 = Note::new(Pitch::C, 4);
//! let g4 = c4 + Interval::PERFECT_FIFTH;
//! assert_eq!(g4, Note::new(Pitch::G, 4));
//! ```

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::harmony::Key;
use crate::interval::Interval;
use crate::pitch::{Pitch, PitchClass, Spelling};
use crate::semitone::Semitones;

/// A musical note with pitch and octave.
///
/// A `Note` combines a [`Pitch`] with an octave number (represented as an [`i16`](prim@i16)).
///
/// # Examples
///
/// You can create a `Note` from a pitch and octave:
/// ```
/// # use music_theory::prelude::*;
/// let middle_c = Note::new(Pitch::C, 4);
/// # assert_eq!(middle_c, Note::MIDDLE_C);
/// ```
///
/// Notes can be transposed by intervals:
/// ```
/// # use music_theory::prelude::*;
/// let e4 = Note::A4 + Interval::MAJOR_SIXTH;
/// assert_eq!(e4, Note::new(Pitch::F_SHARP, 5));
/// ```
///
/// And can be converted to other representations:
/// ```
/// # use music_theory::prelude::*;
/// // Converting to and from MIDI
/// assert_eq!(Note::MIDDLE_C.as_midi(), Some(60));
/// assert_eq!(Note::from_midi(69), Note::A4);
///
/// // Converting to and from frequency in 12-TET
/// assert_eq!(Note::from_frequency_hz(261.6256), Some(Note::MIDDLE_C));
/// assert_eq!(Note::A4.as_frequency_hz(), 440.0);
/// ```
///
/// [`Letter`]: crate::pitch::Letter
/// [`AccidentalSign`]: crate::pitch::AccidentalSign
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Note {
    /// The pitch (has [letter](crate::pitch::Letter) and [accidental](crate::pitch::AccidentalSign)).
    pub pitch: Pitch,
    /// The octave number.
    pub octave: i16,
}

impl Note {
    /// Middle C (C4).
    pub const MIDDLE_C: Self = Self { pitch: Pitch::C, octave: 4 };

    /// Concert A (A4), the standard tuning reference note.
    pub const A4: Self = Self { pitch: Pitch::A, octave: 4 };

    /// Creates a new `Note` from a pitch and octave number.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let c4 = Note::new(Pitch::C, 4);
    /// let eb2 = Note::new(Pitch::E_FLAT, 2);
    /// let gx6 = Note::new(Pitch::G_DOUBLE_SHARP, 6);
    /// ```
    pub fn new(pitch: Pitch, octave: i16) -> Self {
        Self { pitch, octave }
    }

    /// Returns the signed semitone distance from `self` to `other`.
    ///
    /// Returns a positive value if `other` is higher than `self`, and negative values
    /// if `other` is lower.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let e4 = Note::new(Pitch::E, 4);
    /// assert_eq!(Note::MIDDLE_C.semitones_to(e4), Semitones(4));
    ///
    /// let gf5 = Note::new(Pitch::G_FLAT, 5);
    /// assert_eq!(gf5.semitones_to(e4), Semitones(-14));
    /// ```
    pub fn semitones_to(self, other: Self) -> Semitones {
        let lhs = self.pitch.semitones_offset_from_c() + Semitones(self.octave * 12);

        let rhs = other.pitch.semitones_offset_from_c() + Semitones(other.octave * 12);

        rhs - lhs
    }

    /// Calculates the interval between `self` and `other`.
    ///
    /// If `other` is lower than `self`, the returned interval is descending.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(
    ///     Note::MIDDLE_C.distance_to(Note::new(Pitch::G, 4)),
    ///     Interval::PERFECT_FIFTH,
    /// );
    ///
    /// // Aware of enharmonic spelling, like a tritone being either an A4 or d5
    /// assert_eq!(
    ///     Note::new(Pitch::G, 4).distance_to(Note::new(Pitch::C_SHARP, 5)),
    ///     Interval::AUGMENTED_FOURTH,
    /// );
    /// assert_eq!(
    ///     Note::new(Pitch::G, 4).distance_to(Note::new(Pitch::D_FLAT, 5)),
    ///     Interval::DIMINISHED_FIFTH,
    /// );
    /// ```
    pub fn distance_to(self, other: Self) -> Interval {
        Interval::between_notes(self, other)
    }

    /// Returns the closest `Note` from a frequency in Hz, using 12-tone equal temperament.
    ///
    /// Returns `None` if the frequency is non-positive or not finite. Some [subnormal](f32::is_subnormal) floats
    /// may also return `None`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // A4 is 440 Hz
    /// assert_eq!(Note::from_frequency_hz(440.0), Some(Note::A4));
    ///
    /// // Middle C is approximately 261.6256 Hz
    /// assert_eq!(Note::from_frequency_hz(261.6256), Some(Note::MIDDLE_C));
    ///
    /// // Invalid frequencies return None
    /// assert_eq!(Note::from_frequency_hz(-100.0), None);
    /// assert_eq!(Note::from_frequency_hz(f32::INFINITY), None);
    /// ```
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

    /// Converts the note to its frequency in Hz, using 12-tone equal temperament.
    ///
    /// Uses A4 = 440 Hz as the reference pitch.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Note::A4.as_frequency_hz(), 440.0);
    ///
    /// // Middle C is approximately 261.6256 Hz
    /// let c4_freq = Note::MIDDLE_C.as_frequency_hz();
    /// assert!((c4_freq - 261.6256).abs() < 1e-4);
    ///
    /// let d2_freq = Note::new(Pitch::D, 2).as_frequency_hz();
    /// assert!((d2_freq - 73.4162).abs() < 1e-4);
    /// ```
    pub fn as_frequency_hz(self) -> f32 {
        let semitones_from_a4 = Self::A4.semitones_to(self);

        440.0 * 2.0_f32.powf(semitones_from_a4.0 as f32 / 12.0)
    }

    /// Transposes the note by the given interval. Has the same behavior as the [`+` operator](Add::add).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(
    ///     Note::new(Pitch::D, 4).transpose(Interval::MAJOR_THIRD),
    ///     Note::new(Pitch::F_SHARP, 4),
    /// );
    ///
    /// // Descending intervals are also supported
    /// assert_eq!(
    ///     Note::new(Pitch::B, 4).transpose(-Interval::MINOR_SIXTH),
    ///     Note::new(Pitch::D_SHARP, 4),
    /// );
    ///
    /// // Same behavior as Add::add
    /// let m2 = Interval::MINOR_SECOND;
    /// assert_eq!(Note::MIDDLE_C.transpose(m2), Note::MIDDLE_C + m2);
    /// ```
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
    ///
    /// If a note can't be written with a natural, the returned note will always have the
    /// opposite spelling as before. Notably, this means the enharmonic of `Ex4` is `Gb4`, *not* `F#4`.
    /// For the opposite behavior, see [`Self::simplified`].
    ///
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

    /// Converts the note to a MIDI note number.
    ///
    /// Returns `None` if the note is outside the range that can be represented
    /// by a `u8`.
    ///
    /// MIDI technically only supports note numbers 0-127, but this method can return values
    /// in `[0, 255]`. Use [`Self::as_midi_strict`] if you need strict MIDI compliance.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // A4 (concert pitch) is MIDI note 69
    /// assert_eq!(Note::A4.as_midi(), Some(69));
    ///
    /// // C-1 is MIDI note 0
    /// let c_minus_1 = Note::new(Pitch::C, -1);
    /// assert_eq!(c_minus_1.as_midi(), Some(0));
    ///
    /// // G22 is outside MIDI range (max is D#20)
    /// assert_eq!(Note::new(Pitch::G, 22).as_midi(), None);
    /// # assert_eq!(Note::from_midi(u8::MAX), Note::new(Pitch::D_SHARP, 20), "docs invalid");
    /// ```
    pub fn as_midi(self) -> Option<u8> {
        let zero = Note { pitch: Pitch::C, octave: -1 };

        zero.semitones_to(self)
            .0
            .try_into()
            .ok()
    }

    /// Converts the note to a MIDI note number, strictly within MIDI range (0-127).
    ///
    /// See [`Self::as_midi`] for a less strict version of this method.
    ///
    /// Returns `None` if the note is outside the valid MIDI range.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Middle C is within MIDI range
    /// assert_eq!(Note::MIDDLE_C.as_midi_strict(), Some(60));
    ///
    /// // G9 (MIDI 127) is the highest valid MIDI note
    /// let g9 = Note::new(Pitch::G, 9);
    /// assert_eq!(g9.as_midi_strict(), Some(127));
    ///
    /// // G#9 (MIDI 128) is outside the valid MIDI range
    /// let gs9 = Note::new(Pitch::G_SHARP, 9);
    /// assert_eq!(gs9.as_midi_strict(), None);
    /// ```
    pub fn as_midi_strict(self) -> Option<u8> {
        self.as_midi().filter(|&m| m < 128)
    }

    /// Creates a `Note` from a MIDI note number.
    ///
    /// The returned note will be spelled with sharps. To change this, use [`Self::respell_with`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // MIDI 60 is middle C
    /// assert_eq!(Note::from_midi(60), Note::MIDDLE_C);
    ///
    /// // MIDI 69 is A4
    /// assert_eq!(Note::from_midi(69), Note::A4);
    ///
    /// // Black keys use sharps
    /// assert_eq!(Note::from_midi(61), Note::new(Pitch::C_SHARP, 4));
    ///
    /// // Works with the full MIDI range
    /// assert_eq!(Note::from_midi(0), Note::new(Pitch::C, -1));
    /// assert_eq!(Note::from_midi(127), Note::new(Pitch::G, 9));
    /// ```
    pub fn from_midi(midi: u8) -> Note {
        let pitch = PitchClass::from_chroma(midi % 12)
            .expect("% 12 must be [0, 11]");
        let oct = midi / 12;

        Self {
            pitch: pitch.into(),
            octave: oct as i16 - 1,
        }
    }

    /// Transposes the note by a given number of fifths.
    ///
    /// This is much more efficient than [`Self::transpose`] due to internal representation of [`Pitch`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(
    ///     Note::MIDDLE_C.transpose_fifths(1),
    ///     Note::new(Pitch::G, 4),
    /// );
    ///
    /// assert_eq!(
    ///     Note::MIDDLE_C.transpose_fifths(-4),
    ///     Note::new(Pitch::A_FLAT, 1),
    /// );
    /// ```
    pub fn transpose_fifths(self, fifths: i16) -> Self {
        let unchecked = Self {
            pitch: self.pitch.transpose_fifths(fifths),
            octave: self.octave,
        };

        let interval_semi = Semitones(7 * fifths);

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