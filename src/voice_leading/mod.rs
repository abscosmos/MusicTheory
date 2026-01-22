use std::ops::{Deref, Index, IndexMut, RangeInclusive};
use strum_macros::{EnumIter, FromRepr};
use crate::note::Note;
use crate::pitch::Pitch;

pub mod rules;
pub mod roman_chord;
pub mod check;
pub mod solve;

pub mod debug;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Voicing(pub [Note; 4]);

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, FromRepr, EnumIter, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Voice {
    Soprano = 0,
    Alto = 1,
    Tenor = 2,
    Bass = 3,
}

impl Voice {
    pub const fn range(self) -> RangeInclusive<Note> {
        match self {
            Voice::Soprano => Note::new(Pitch::C, 4)..=Note::new(Pitch::G, 5),
            Voice::Alto => Note::new(Pitch::G, 3)..=Note::new(Pitch::D, 5),
            Voice::Tenor => Note::new(Pitch::C, 3)..=Note::new(Pitch::G, 4),
            Voice::Bass => Note::new(Pitch::E, 2)..=Note::new(Pitch::D, 4),
        }
    }
}

impl Voicing {
    pub fn new(notes: [Note; 4]) -> Self {
        Self(notes)
    }
}

impl Index<Voice> for Voicing {
    type Output = Note;

    fn index(&self, index: Voice) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Voice> for Voicing {
    fn index_mut(&mut self, index: Voice) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Deref for Voicing {
    type Target = [Note; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}