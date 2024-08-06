use crate::note::pitch::Pitch;

pub mod pitch_class;
pub mod accidental;
pub mod pitch;
pub mod letter;

pub struct Note {
    pub pitch: Pitch,
    pub octave: u8,
}