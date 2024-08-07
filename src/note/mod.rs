use crate::note::pitch::Pitch;

pub mod pitch_class;
pub mod accidental;
pub mod pitch;
pub mod letter;

// TODO: Placed<T>
pub struct Note {
    pub pitch: Pitch,
    pub octave: i8,
}

impl Note {
    pub fn from_frequency_hz(hz: f32) -> Option<Self> {
        todo!()
    }

    pub fn as_frequency_hz(&self) -> f32 {
        todo!()
    }
}



