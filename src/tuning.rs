use serde::{Deserialize, Serialize};
use crate::note::Note;
use typed_floats::tf32::{StrictlyPositiveFinite, NonNaNFinite};
use crate::pitch::Pitch;
use crate::pitch_class::PitchClass;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Cents(NonNaNFinite);

impl Cents {
    pub fn new(c: f32) -> Option<Self> {
        match NonNaNFinite::new(c) {
            Ok(c) if -100.0 <= c && c <= 100.0 => Some(Self(c)),
            _ => None,
        }
    }

    pub fn get(self) -> f32 {
        self.0.get()
    }
}

pub trait Tuning {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, f32)>;

    fn note_to_freq_hz(&self, note: Note) -> Option<StrictlyPositiveFinite>;
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwelveToneEqualTemperament {
    pub a4_hz: StrictlyPositiveFinite,
}

impl TwelveToneEqualTemperament {
    pub const HZ_440: Self = Self::new(440.0).expect("440.0 is strictly positive and finite");

    pub const fn new(a4_hz: f32) -> Option<Self> {
        match StrictlyPositiveFinite::new(a4_hz) {
            Ok(a4_hz) => Some(Self { a4_hz }),
            Err(_) => None,
        }
    }
}

impl Tuning for TwelveToneEqualTemperament {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, f32)> {
        let semitones_from_a4 = 12.0 * (hz / self.a4_hz).log2().get();

        if !semitones_from_a4.is_finite() {
            return None;
        }

        let semitones_from_c0 = semitones_from_a4.round() as i16 + 9 + 4 * 12;

        let octave = semitones_from_c0.div_euclid(12);

        let pitch = semitones_from_c0.rem_euclid(12)
            .try_into()
            .expect("i32::rem_euclid(12) must be within [0,12)");

        let pitch = PitchClass::from_repr(pitch)
            .expect("i32::rem_euclid(12) must be within [0,12)");

        let cents = {
            let fract = if semitones_from_a4.trunc() == semitones_from_a4.round() {
                semitones_from_a4.fract()
            } else {
                semitones_from_a4.fract() - 1.0
            };

            fract * 100.0
        };

        Some(( Note { pitch: Pitch::from(pitch), octave }, cents ))
    }

    fn note_to_freq_hz(&self, note: Note) -> Option<StrictlyPositiveFinite> {
        let semitones_from_a4 = Note::A4.semitones_to(note);

        let hz = self.a4_hz.get() * 2.0_f32.powf(semitones_from_a4.0 as f32 / 12.0);

        StrictlyPositiveFinite::new(hz).ok()
    }
}