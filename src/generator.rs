//! Infinite chromatic note generation.
//!
//! [`NoteGenerator`] is an infinite iterator that generates [notes](Note) chromatically,
//! either ascending or descending.
//!
//! # Examples
//! ```
//! # use music_theory::prelude::*;
//! # use music_theory::generator::NoteGenerator;
//! // Create a generator starting from middle C that's ascending
//! let mut ascending = NoteGenerator::new(Note::MIDDLE_C);
//!
//! assert_eq!(ascending.next(), Some(Note::new(Pitch::C, 4)));
//! assert_eq!(ascending.next(), Some(Note::new(Pitch::C_SHARP, 4)));
//! assert_eq!(ascending.next(), Some(Note::new(Pitch::D, 4)));
//!
//! // ... or one that's descending
//! let mut descending = NoteGenerator::reversed(Note::MIDDLE_C);
//! assert_eq!(descending.next(), Some(Note::new(Pitch::C, 4)));
//! assert_eq!(descending.next(), Some(Note::new(Pitch::B, 3)));
//! assert_eq!(descending.next(), Some(Note::new(Pitch::A_SHARP, 3)));
//!
//! // Generate a specific range of notes
//! let c_major_scale_range = NoteGenerator::range_inclusive(
//!     Note::MIDDLE_C,
//!     Note::new(Pitch::C, 5),
//! );
//!
//! // [C4, C5] has 13 notes
//! assert_eq!(c_major_scale_range.count(), 13);
//! ```

use std::iter::{FusedIterator, Take};
use crate::note::Note;
use crate::pitch::{Pitch, PitchClass};

/// An infinite iterator that generates [notes](Note) chromatically.
///
/// `NoteGenerator` produces notes in chromatic order (semitone by semitone), either
/// ascending or descending. It can start from any [`Note`] and continues infinitely
/// until overflow occurs.
///
/// Notes are generated with simplified sharp-based spellings from [`PitchClass`]
/// (e.g., C#, not Db). To get different enharmonic spellings, use methods like
/// [`Pitch::bias`] on the resulting notes.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use music_theory::prelude::*;
/// # use music_theory::generator::NoteGenerator;
/// let mut generator = NoteGenerator::new(Note::MIDDLE_C);
/// assert_eq!(generator.nth(1), Some(Note::new(Pitch::C_SHARP, 4)));
/// ```
///
/// Generating notes in reverse:
/// ```
/// # use music_theory::prelude::*;
/// # use music_theory::generator::NoteGenerator;
/// let mut generator = NoteGenerator::reversed(Note::MIDDLE_C);
/// assert_eq!(generator.nth(1), Some(Note::new(Pitch::B, 3)));
/// ```
///
/// Or, just in a specific range:
/// ```
/// # use music_theory::prelude::*;
/// # use music_theory::generator::NoteGenerator;
/// // Generate one octave starting from C4
/// let mut octave = NoteGenerator::range_inclusive(
///     Note::MIDDLE_C,
///     Note::new(Pitch::C, 5),
/// );
///
/// # assert_eq!(octave.clone().count(), 13);
/// assert_eq!(octave.nth(4), Note::new(Pitch::E, 4));
/// assert_eq!(octave.last(), Note::new(Pitch::C, 5));
/// ```
///
/// # Overflow Behavior
///
/// Since `NoteGenerator` is infinite, it will eventually overflow the octave [`Note`] can hold.
/// To avoid overflow, use bounded methods:
/// - [`range`](Self::range) for half-open ranges
/// - [`range_inclusive`](Self::range_inclusive) for closed ranges
/// - [`take_until_overflow`](Self::take_until_overflow) to generate until the limit
/// - [`full_note_range`](Self::full_note_range) for the complete valid range
///
/// # Panics
///
/// The [`count`](Iterator::count) and [`last`](Iterator::last) methods panic because
/// `NoteGenerator` is infinite.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NoteGenerator {
    // doing it from a "midi like" representation is implementation detail
    current: i32,
    reverse: bool,
}

impl NoteGenerator {
    const MIN_NOTE: Note = Note { pitch: Pitch::C, octave: i16::MIN };
    const MAX_NOTE: Note = Note { pitch: Pitch::B, octave: i16::MAX };

    /// Creates a new `NoteGenerator` starting from the given note.
    ///
    /// The generator will produce notes chromatically in ascending order,
    /// starting with the provided note.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let mut generator = NoteGenerator::new(Note::MIDDLE_C);
    /// assert_eq!(generator.next(), Some(Note::MIDDLE_C));
    /// assert_eq!(generator.next(), Some(Note::new(Pitch::C_SHARP, 4)));
    /// ```
    pub fn new(current: Note) -> Self {
        Self {
            current: Self::note_to_repr(current),
            reverse: false
        }
    }

    /// Reverses the direction of the generator.
    ///
    /// If the generator was ascending, it becomes descending, and vice versa.
    /// The current position remains unchanged. If you intend to call `reverse` right
    /// after constructing a new `NoteGenerator`, you may want to use [`Self::reversed`].
    ///
    /// This method may cause confusion when switching the direction in the middle of using it:
    /// ```rust
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let mut generator = NoteGenerator::new(Note::MIDDLE_C);
    /// assert_eq!(generator.next(), Some(Note::MIDDLE_C));
    /// assert_eq!(generator.next(), Some(Note::new(Pitch::C_SHARP, 4)));
    ///
    /// generator = generator.reverse();
    /// // Even though the last note was C#, and it was just reversed,
    /// // this will still return D instead of C as might be expected.
    /// // This is because after the call to 'next()' which returned C#,
    /// // the generator's position was advanced to D, and calling reverse
    /// // doesn't change that.
    /// assert_eq!(generator.next(), Some(Note::new(Pitch::D, 4)));
    /// ```
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let mut generator = NoteGenerator::new(Note::MIDDLE_C).reverse();
    /// assert_eq!(generator.next(), Some(Note::MIDDLE_C));
    /// assert_eq!(generator.next(), Some(Note::new(Pitch::B, 3)));
    /// ```
    pub fn reverse(self) -> Self {
        Self { reverse: !self.reverse, ..self }
    }

    /// Creates a new `NoteGenerator` that generates in descending order, starting from the given note.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let mut generator = NoteGenerator::reversed(Note::MIDDLE_C);
    /// assert_eq!(generator.next(), Some(Note::MIDDLE_C));
    /// assert_eq!(generator.next(), Some(Note::new(Pitch::B, 3)));
    /// ```
    pub fn reversed(current: Note) -> Self {
        Self::new(current).reverse()
    }

    /// Creates a bounded generator for a half-open range of notes `[start, end)`.
    ///
    /// If `start` is lower than `end`, generates ascending notes. Otherwise, generates
    /// descending notes.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// // Ascending range from [C4 to E4)
    /// let ascending = NoteGenerator::range(
    ///     Note::MIDDLE_C,
    ///     Note::new(Pitch::E, 4),
    /// );
    /// // [C4, E4) = { C, C#, D, D# }
    /// assert_eq!(ascending.count(), 4);
    ///
    /// // Descending range from [E4 to C4)
    /// let mut descending = NoteGenerator::range(
    ///     Note::new(Pitch::E, 4),
    ///     Note::MIDDLE_C,
    /// );
    /// assert_eq!(descending.next(), Some(Note::new(Pitch::E, 4)));
    /// ```
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

    /// Creates a bounded generator for a closed range of notes `[start, end]`.
    ///
    /// If `start` is lower than `end`, generates ascending notes. Otherwise, generates
    /// descending notes.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// // Ascending range from [C4 to E4]
    /// let ascending = NoteGenerator::range_inclusive(
    ///     Note::MIDDLE_C,
    ///     Note::new(Pitch::E, 4),
    /// );
    /// // [C4, E4] == { C, C#, D, D#, E }
    /// assert_eq!(ascending.count(), 5);
    ///
    /// // Full octave, including both endpoints
    /// let mut octave = NoteGenerator::range_inclusive(
    ///     Note::MIDDLE_C,
    ///     Note::new(Pitch::C, 5),
    /// );
    /// assert_eq!(octave.count(), 13);
    /// ```
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

    /// Creates a bounded generator that continues until it would overflow.
    ///
    /// For ascending generators, generates from the current position to the maximum
    /// representable note. For descending generators, generates from the current
    /// position to the minimum representable note.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let until_max = NoteGenerator::new(Note::MIDDLE_C).take_until_overflow();
    /// // Since this is not infinite, it's fine to call 'last'
    /// assert_eq!(until_max.last(), Some(Note::new(Pitch::B, i16::MAX)));
    ///
    /// let until_min = NoteGenerator::reversed(Note::MIDDLE_C).take_until_overflow();
    /// assert_eq!(until_min.last(), Some(Note::new(Pitch::C, i16::MIN)));
    /// ```
    pub fn take_until_overflow(self) -> Take<Self> {
        let Self { current, reverse } = self;

        let current = Self::repr_to_note(current);

        if !reverse {
            Self::range_inclusive(current, Self::MAX_NOTE)
        } else {
            Self::range_inclusive(current, Self::MIN_NOTE)
        }
    }

    /// Creates a generator for the complete range of all valid notes.
    ///
    /// Generates from `C` at octave [`i16::MIN`] to `B` at octave [`i16::MAX`] when
    /// `reverse` is `false`, or in the opposite direction when `reverse` is `true`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let mut ascending = NoteGenerator::full_note_range(false);
    /// // First note is the minimum representable note
    /// assert_eq!(ascending.next(), Some(Note::new(Pitch::C, i16::MIN)));
    /// // ... and last is the maximum representable note
    /// assert_eq!(ascending.last(), Some(Note::new(Pitch::B, i16::MAX)));
    /// ```
    pub fn full_note_range(reverse: bool) -> Take<Self> {
        if !reverse {
            Self::range_inclusive(Self::MIN_NOTE, Self::MAX_NOTE)
        } else {
            Self::range_inclusive(Self::MAX_NOTE, Self::MIN_NOTE)
        }
    }

    /// Returns the next note that will be generated, without advancing the iterator.
    ///
    /// This is similar to calling [`Iterator::peekable`], but doesn't require the [`Peekable`](std::iter::Peekable) wrapper.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let mut generator = NoteGenerator::new(Note::MIDDLE_C);
    ///
    /// // Peek at the next note
    /// assert_eq!(generator.peek(), Note::new(Pitch::C, 4));
    /// // The iterator hasn't advanced
    /// assert_eq!(generator.peek(), Note::new(Pitch::C, 4));
    ///
    /// // Now actually consume the note
    /// assert_eq!(generator.next(), Some(Note::new(Pitch::C, 4)));
    /// assert_eq!(generator.peek(), Note::new(Pitch::C_SHARP, 4));
    /// ```
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

        assert!(
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

    /// # Panics
    ///
    /// This method always panics because `NoteGenerator` is infinite.
    /// Use bounded methods like [`range`](Self::range) or [`take`](Iterator::take) instead.
    ///
    /// # Examples
    /// ```should_panic
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let generator = NoteGenerator::new(Note::MIDDLE_C);
    /// generator.count(); // This will panic!
    /// ```
    fn count(self) -> usize where Self: Sized {
        panic!("NoteGenerator is infinite, cannot count");
    }

    /// # Panics
    ///
    /// This method always panics because `NoteGenerator` is infinite and has no last element.
    /// Use bounded methods like [`range`](Self::range) or [`take`](Iterator::take) instead.
    ///
    /// # Examples
    /// ```should_panic
    /// # use music_theory::prelude::*;
    /// # use music_theory::generator::NoteGenerator;
    /// let generator = NoteGenerator::new(Note::MIDDLE_C);
    /// generator.last(); // This will panic!
    /// ```
    fn last(self) -> Option<Self::Item> where Self: Sized {
        panic!("NoteGenerator is infinite, has no last element");
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let offset = n.clamp(i32::MIN as _, i32::MAX as _) as _;

        // this is fine, because the min/max is errors to generate notes from anyway
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

    #[test]
    #[should_panic]
    fn internal_repr_max() {
        let mut note_gen = NoteGenerator {
            current: i32::MAX,
            reverse: false,
        };

        let _ = note_gen.next();
    }
}