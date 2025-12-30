use crate::note::Note;
use crate::pitch::{Pitch, PitchClass};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NoteGenerator {
    // doing it from a "midi like" representation is implementation detail
    current: i32,
    reverse: bool,
}

impl NoteGenerator {
    pub fn new(current: Note) -> Self {
        Self {
            current: Self::note_to_repr(current),
            reverse: false
        }
    }

    pub fn reverse(self) -> Self {
        Self { reverse: !self.reverse, ..self }
    }

    pub fn reversed(current: Note) -> Self {
        Self::new(current).reverse()
    }

    // this is copy Note::from_midi with types changed
    #[inline]
    fn repr_to_note(repr: i32) -> Note {
        let pitch = PitchClass::from_repr(repr.rem_euclid(12) as _)
            .expect("% 12 must be [0, 11]");

        let oct = repr.div_euclid(12);

        // it seems using #[cfg(overflow_checks)] isn't stable
        debug_assert!(
            (i16::MIN as i32..=i16::MAX as i32).contains(&oct),
            "octave must be in [-32768, 32767], got {oct}"
        );

        Note {
            pitch: pitch.into(),
            octave: oct as _,
        }
    }

    // this is copy Note::as_midi with types changed
    // TODO: an implementation that's an inverse of the above might be better instead of
    //     'semitones_to' due to the size of the repr
    #[inline]
    fn note_to_repr(note: Note) -> i32 {
        let zero = Note { pitch: Pitch::C, octave: 0 };

        zero.semitones_to(note).0 as _
    }
}