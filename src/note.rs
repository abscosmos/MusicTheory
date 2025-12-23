use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Deref, DerefMut, Sub};
use serde::{Deserialize, Serialize};
use typed_floats::tf32::StrictlyPositiveFinite;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::interval::Interval;
use crate::letter::Letter;
use crate::pitch::Pitch;
use crate::pitch_class::PitchClass;
use crate::semitone::Semitone;
use crate::tuning::{Tuning, TwelveToneEqualTemperament};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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
    
    // TODO: this function may not be correct due to Pitch::semitones_offset_from_c
    pub fn semitones_to(self, other: Self) -> Semitone {
        let lhs = self.pitch.semitones_offset_from_c() + Semitone(self.octave * 12);

        let rhs = other.pitch.semitones_offset_from_c() + Semitone(other.octave * 12);

        rhs - lhs
    }

    pub fn distance_to(self, other: Self) -> Interval {
        Interval::between_notes(self, other)
    }
    
    // TODO: this includes spelling, when it probably shouldn't
    pub fn from_frequency_hz(hz: StrictlyPositiveFinite) -> Option<Self> {
        TwelveToneEqualTemperament::HZ_440.freq_to_note(hz).map(|(n, _)| n)
    }

    /*
        TODO: frequency methods should take in a tuning struct:
            - just intonation
            - pythagorean tuning
            - meantone temperament
            - well temperament
            - equal temperament
    */ 
    pub fn as_frequency_hz(self) -> Option<StrictlyPositiveFinite> {
        TwelveToneEqualTemperament::HZ_440.note_to_freq_hz(self)
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

    pub fn bias(self, sharp: bool) -> Self {
        let base = self.pitch.bias(sharp);

        let unchecked = Self { pitch: base, ..self };

        let octave_offset = self.semitones_to(unchecked).0.div_euclid(12);

        Self {
            octave: unchecked.octave - octave_offset,
            .. unchecked
        }
    }
    
    pub fn simplified(self) -> Self {
        let base = self.pitch.simplified();

        let unchecked = Self { pitch: base, ..self };

        let octave_offset = self.semitones_to(unchecked).0.div_euclid(12);

        Self {
            octave: unchecked.octave - octave_offset,
            .. unchecked
        }
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
        let pitch = PitchClass::from_repr(midi % 12)
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

impl From<Note> for Letter {
    fn from(note: Note) -> Self {
        note.letter()
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

// TODO: reevaluate if Note should Deref[Mut] into Pitch 
impl Deref for Note {
    type Target = Pitch;

    fn deref(&self) -> &Self::Target {
        &self.pitch
    }
}

impl DerefMut for Note {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pitch
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
}