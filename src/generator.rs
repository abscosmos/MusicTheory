use std::iter::{FusedIterator, Take};
use crate::note::Note;
use crate::pitch::{Pitch, PitchClass};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NoteGenerator {
    // doing it from a "midi like" representation is implementation detail
    current: i32,
    reverse: bool,
}

impl NoteGenerator {
    const MIN_NOTE: Note = Note { pitch: Pitch::C, octave: i16::MIN };
    const MAX_NOTE: Note = Note { pitch: Pitch::B, octave: i16::MAX };

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

    pub fn range(start: Note, end: Note) -> Take<Self> {
        let start_repr = Self::note_to_repr(start);
        let end_repr = Self::note_to_repr(end);

        let count = (end_repr - start_repr).unsigned_abs() as _;

        if start_repr <= end_repr {
            Self::new(start).take(count)
        } else {
            Self::reversed(start).take(count)
        }
    }

    pub fn range_inclusive(start: Note, end: Note) -> Take<Self> {
        let start_repr = Self::note_to_repr(start);
        let end_repr = Self::note_to_repr(end);

        let count = (end_repr - start_repr).unsigned_abs() + 1;

        if start_repr <= end_repr {
            Self::new(start).take(count as _)
        } else {
            Self::reversed(start).take(count as _)
        }
    }

    pub fn take_until_overflow(self) -> Take<Self> {
        let Self { current, reverse } = self;

        let current = Self::repr_to_note(current);

        if !reverse {
            Self::range_inclusive(current, Self::MAX_NOTE)
        } else {
            Self::range_inclusive(current, Self::MIN_NOTE)
        }
    }

    pub fn full_note_range(reverse: bool) -> Take<Self> {
        if !reverse {
            Self::range_inclusive(Self::MIN_NOTE, Self::MAX_NOTE)
        } else {
            Self::range_inclusive(Self::MAX_NOTE, Self::MIN_NOTE)
        }
    }

    pub fn peek(&self) -> Note {
        Self::repr_to_note(self.current)
    }

    // this is very similar to Note::from_midi with types changed,
    // and using a different note for 0
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

    #[inline]
    fn note_to_repr(note: Note) -> i32 {
        note.octave as i32 * 12 + note.pitch.as_pitch_class() as i32
    }
}

impl Iterator for NoteGenerator {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        let note = Self::repr_to_note(self.current);

        if self.reverse {
            self.current -= 1;
        } else {
            self.current += 1;
        }

        Some(note)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }

    fn count(self) -> usize where Self: Sized {
        panic!("NoteGenerator is infinite, cannot count");
    }

    fn last(self) -> Option<Self::Item> where Self: Sized {
        panic!("NoteGenerator is infinite, has no last element");
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let offset = usize::clamp(i32::MIN as _, i32::MAX as _, n) as _;

        self.current = if self.reverse {
            self.current.saturating_sub(offset)
        } else {
            self.current.saturating_add(offset)
        };

        self.next()
    }
}

impl FusedIterator for NoteGenerator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth() {
        let note_gen = NoteGenerator::new(Note::MIDDLE_C);

        assert_eq!(
            note_gen.clone().next(),
            note_gen.clone().nth(0),
        )
    }
}