use std::ops::{Deref, Index, IndexMut};
use strum_macros::{EnumIter, FromRepr};
use crate::note::Note;

pub mod rules;
pub mod roman_chord;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Voicing(pub [Note; 4]);

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, FromRepr, EnumIter)]
pub enum Voice {
    Soprano = 0,
    Alto = 1,
    Tenor = 2,
    Bass = 3,
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