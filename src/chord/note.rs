use std::cmp::Ordering;
use crate::chord::pitch::PitchChord;
use crate::{Interval, Letter, Note, Pitch};
use crate::chord::root;
use crate::chord::letter_set::LetterSet;
use crate::set::PitchClassSet;

pub struct NoteChord {
    notes: Box<[Note]>,
    root: Pitch,
}

impl NoteChord {
    pub fn new(notes: impl IntoIterator<Item=Note>) -> Option<Self> {
        let notes = Self::sort_dedup(notes);

        if notes.is_empty() {
            return None;
        }
        
        let root = root::find_root(notes.iter().map(|n| n.pitch))
            .expect("not empty");
        
        Some(Self { notes, root })
    }

    pub fn with_root(notes: impl IntoIterator<Item=Note>, root: Pitch) -> Option<Self> {
        let notes = Self::sort_dedup(notes);

        if notes.iter().any(|n| n.pitch == root) {
            Some(Self { notes, root })
        } else {
            None
        }
    }

    fn sort_dedup(notes: impl IntoIterator<Item=Note>) -> Box<[Note]> {
        let mut notes = notes.into_iter().collect::<Vec<_>>();

        notes.sort();
        notes.dedup();

        notes.into_boxed_slice()
    }

    pub fn from_pitch_chord(chord: &PitchChord, bass_octave: i16) -> Self {
        Self::with_root(chord.notes(bass_octave), chord.root()).expect("can't be empty")
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    pub fn root(&self) -> Pitch {
        self.root
    }

    pub fn bass(&self) -> Note {
        *self.notes.first().expect("shouldn't be empty")
    }

    pub fn pitch_class_set(&self) -> PitchClassSet {
        self.notes.iter()
            .map(|n| n.pitch.as_pitch_class())
            .collect()
    }

    pub fn contains_pitch(&self, pitch: Pitch) -> bool {
        self.notes.iter().any(|n| n.pitch == pitch)
    }

    pub fn contains_note(&self, note: Note) -> bool {
        self.notes.contains(&note)
    }

    pub fn closed_position(&self, separate_steps: bool) -> Self {
        // TODO: potentially make this a vecdeque
        let mut notes = self.notes.to_vec();

        let mut closed = Vec::with_capacity(self.len());

        let mut letters_added = LetterSet::EMPTY;

        // TODO: dedup same pitch different octave!!

        // 1. add root(s)
        let root = {
            let root_pos = notes.iter()
                .position(|n| n.pitch == self.root)
                .expect("root should exist in chord");

            notes.remove(root_pos)
        };

        closed.push(root);
        letters_added = letters_added.with_set(root.pitch.letter());

        let roots = notes.extract_if(.., |n|
            n.pitch.letter() == self.root.letter()
        );

        for (i, note) in roots.enumerate() {
            let octave = root.octave + 1;
            let new_octave = if separate_steps { octave + i as i16 } else { octave };

            closed.push(Note::new(note.pitch, new_octave));
        }

        // 2. find thirds
        loop {
            if letters_added == LetterSet::FULL {
                break;
            }

            let last = closed.last()
                .expect("should be at least one elem");

            let last_letter = last.pitch.letter();

            let third_letter = Letter::from_step((last_letter.step() + 2) % 7)
                .expect("should be valid letter");

            let thirds = notes.extract_if(.., |n|
                n.pitch.letter() == third_letter
            );

            let octave = match last_letter.cmp(&third_letter) {
                Ordering::Less => last.octave,
                Ordering::Greater => last.octave + 1,
                Ordering::Equal => unreachable!("letter should be two steps away"),
            };

            let mut has_any = false;

            for (i, note) in thirds.enumerate() {
                has_any = true;

                let octave = if separate_steps { octave + i as i16 } else { octave };

                closed.push(Note::new(note.pitch, octave));
            }

            if has_any {
                letters_added = letters_added.with_set(third_letter);
            } else {
                break;
            }
        }

        // 3. insert leftovers

        // to cluster notes with the same letter, sort them by letter
        // which should also preserve ordering of accidentals
        debug_assert!(
            notes.is_sorted(),
            "notes should be sorted (by octave)"
        );

        let (root_letter, root_octave) = {
            let bass = *closed.first().expect("must have at least one note");

            assert_eq!(
                bass.pitch, self.root,
                "before reordering, the bass should be the root",
            );

            (bass.pitch.letter(), bass.octave)
        };

        notes.sort_by_key(|n| root_letter.offset_between(n.pitch.letter()));

        for notes in notes.chunk_by(|a, b| a.pitch.letter() == b.pitch.letter()) {
            let letter = notes.first()
                .expect("must be first item")
                .pitch
                .letter();

            let octave = match root_letter.cmp(&letter) {
                Ordering::Less => root_octave,
                Ordering::Greater => root_octave + 1,
                Ordering::Equal => unreachable!("letter should be root"),
            };

            for (i, note) in notes.iter().enumerate() {
                let octave = if separate_steps { octave + i as i16 } else { octave };

                closed.push(Note::new(note.pitch, octave));
            }
        }

        // 4. inversion
        // ensure state is right before reordering
        if cfg!(debug_assertions) {
            let bass_letter = self.bass().pitch.letter();

            let first_bass = closed.iter().find(|n| n.pitch.letter() == bass_letter)
                .expect("bass letter should exist in chord");

            assert_eq!(
                first_bass.pitch, self.bass().pitch,
                "before reordering for inversion, make sure the right note is in the bass"
            )
        }

        todo!()
    }

    pub fn transpose(&self, interval: Interval) -> Self {
        let notes = self.notes.iter()
            .map(|&n| n + interval)
            .collect::<Box<[_]>>();

        debug_assert!(
            notes.is_sorted(),
            "notes should remain sorted after transpose",
        );

        let root = self.root + interval;

        Self { notes, root }
    }
}