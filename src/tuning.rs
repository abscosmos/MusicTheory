use serde::{Deserialize, Serialize};
use crate::note::Note;

// TODO: cents type

pub trait Tuning {
    fn freq_to_note(&self, hz: f32) -> (Note, f32);

    fn note_to_freq_hz(&self, note: Note) -> f32;
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwelveToneEqualTemperament {
    a4_hz: f32,
}