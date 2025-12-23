use crate::note::Note;

// TODO: cents type

pub trait Tuning {
    fn freq_to_note(&self, hz: f32) -> (Note, f32);

    fn note_to_freq_hz(&self, note: Note) -> f32;
}