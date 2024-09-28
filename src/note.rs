use std::fmt;
use crate::interval::Interval;
use crate::pitch::Pitch;
use crate::pitch_class::PitchClass;
use crate::placed::Placed;
use crate::semitone::Semitone;

pub type Note = Placed<Pitch>;

impl Note {
    pub const MIDDLE_C: Self = Self { base: Pitch::C, octave: 4 };
    pub const A4: Self = Self { base: Pitch::A, octave: 4 };
    
    pub fn distance_from(&self, other: &Self) -> Semitone {
        let lhs = self.base.semitones_offset_from_c() + Semitone(self.octave * 12);

        let rhs = other.base.semitones_offset_from_c() + Semitone(other.octave * 12);

        rhs - lhs
    }

    // TODO: should this return Placed<PitchClass>?
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

        let pitch: u8 = semitones_from_c0.rem_euclid(12)
            .try_into()
            .expect("i32::rem_euclid(12) must be within [0,12)");

        let pitch: PitchClass = pitch.try_into()
            .expect("i32::rem_euclid(12) must be within [0,12)");

        Some( Self { base: Pitch::from(pitch), octave } )
    }

    pub fn as_frequency_hz(&self) -> f32 {
        let semitones_from_a4 = Self::A4.distance_from(self);

        440.0 * 2.0_f32.powf(semitones_from_a4.0 as f32 / 12.0)
    }

    pub fn apply_interval_ascending(&self, interval: &Interval) -> Self {
        self.apply_interval(interval, true)
    }

    pub fn apply_interval_descending(&self, interval: &Interval) -> Self {
        self.apply_interval(interval, false)
    }

    pub fn apply_interval(&self, interval: &Interval, ascending: bool) -> Self {
        let new_pitch = self.base.apply_interval(interval, ascending);

        let unchecked = Self {
            base: new_pitch,
            octave: self.octave,
        };

        let interval_semi = if ascending {
            interval.semitones()
        } else {
            -interval.semitones()
        };

        let edit = self.distance_from(&unchecked) - interval_semi;

        Self {
            octave: unchecked.octave - edit.0.div_euclid(12),
            .. unchecked
        }
    }

    pub fn as_midi(&self) -> Option<u8> {
        let zero = Note { base: Pitch::C, octave: -1 };

        zero.distance_from(self)
            .0
            .try_into()
            .ok()
    }

    pub fn from_midi(midi: u8) -> Note {
        let pitch = PitchClass::try_from(midi % 12)
            .expect("% 12 must be [0, 11]");
        let oct = midi / 12;

        Self {
            base: pitch.into(),
            octave: oct as i16 - 1,
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.base, self.octave)
    }
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Note")
            .field("pitch", &self.base)
            .field("octave", &self.octave)
            .finish()
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