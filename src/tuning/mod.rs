use serde::{Deserialize, Serialize};
use crate::note::Note;
use typed_floats::tf32::{StrictlyPositiveFinite, NonNaNFinite};

mod twelve_tet;
pub use twelve_tet::*;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Cents(NonNaNFinite);

impl Cents {
    pub fn new(c: f32) -> Option<Self> {
        match NonNaNFinite::new(c) {
            Ok(c) if -100.0 <= c && c <= 100.0 => Some(Self(c)),
            _ => None,
        }
    }

    pub const fn get(self) -> f32 {
        self.0.get()
    }
}

pub trait Tuning {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, Cents)>;

    fn note_to_freq_hz(&self, note: Note) -> Option<StrictlyPositiveFinite>;
}