use std::ops::{Deref, Index, IndexMut, RangeInclusive};
use strum_macros::{EnumIter, FromRepr};
use crate::Interval;
use crate::note::Note;
use crate::pitch::Pitch;

pub mod rules_old;
pub mod roman_chord;
pub mod check;
pub mod solve;

pub mod debug;
pub mod rules;

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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum VoiceMotion {
    Oblique,
    Contrary,
    Similar,
    Parallel,
}

fn get_motion_between(voice_1: Voice, voice_2: Voice, first: Voicing, second: Voicing) -> VoiceMotion {
    if voice_1 == voice_2 {
        return VoiceMotion::Oblique;
    }

    let soprano_first = first[voice_1];
    let soprano_second = second[voice_1];
    let bass_first = first[voice_2];
    let bass_second = second[voice_2];

    let soprano_motion = soprano_first.distance_to(soprano_second);
    let bass_motion = bass_first.distance_to(bass_second);

    if soprano_motion == Interval::PERFECT_UNISON && bass_motion == Interval::PERFECT_UNISON {
        VoiceMotion::Oblique
    } else if soprano_motion == bass_motion {
        VoiceMotion::Parallel
    } else if soprano_motion.is_ascending() != bass_motion.is_ascending() {
        VoiceMotion::Contrary
    } else {
        VoiceMotion::Similar
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