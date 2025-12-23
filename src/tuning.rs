use serde::{Deserialize, Serialize};
use crate::note::Note;
use typed_floats::tf32::StrictlyPositiveFinite;

// TODO: cents type

pub trait Tuning {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, f32)>;

    fn note_to_freq_hz(&self, note: Note) -> StrictlyPositiveFinite;
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