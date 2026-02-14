use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use rustc_hash::FxBuildHasher;
use crate::chord::pitch::PitchChord;
use crate::{Interval, Letter, Note, Pitch};
use crate::chord::{root, ChordShape};
use crate::chord::letter_set::LetterSet;
use crate::interval::Number;
use crate::set::PitchClassSet;

#[derive(Clone)]
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

    fn dedup_by<K: Eq + Hash>(&self, mut key: impl FnMut(Note) -> K) -> Self {
        let mut seen = HashSet::with_capacity_and_hasher(self.len(), FxBuildHasher);

        let notes = self.notes.iter()
            .filter(|n| seen.insert(key(**n)))
            .copied()
            .collect();

        Self { notes, root: self.root }
    }

    #[inline]
    pub fn dedup_by_pitch(&self) -> Self {
        self.dedup_by(|n| n.pitch)
    }

    // if not deduped by pitch, and root and
    // also, need method to undo this? after that this method can be public
    // check what the current n is, and go off of that? like find how much to rotate by
    fn with_inversion(&self, n: usize, separate_steps: bool) -> Option<Self> {
        if n == 0 {
            return Some(self.clone())
        }

        if n >= self.len() {
            return None;
        }

        let mut notes = self.notes.clone();

        notes.rotate_left(n);

        let wrap_start = notes.len() - n;

        // bump wrapped notes up one octave
        let bass = notes[0];

        for note in &mut notes[wrap_start..] {
            note.octave += 1;

            if *note < bass {
                note.octave += 1;

                debug_assert!(
                    *note >= bass,
                    "note should now be in correct place",
                );
            }
        }

        notes.sort_unstable();

        // fix multiple notes with same letter at same octave
        if separate_steps {
            for letter in Letter::iter() {
                let mut pitches = notes.iter_mut()
                    .filter(|n| n.pitch.letter() == letter);

                if let Some(lowest) = pitches.next().copied() {
                    for (i, note) in pitches.enumerate() {
                        note.octave = lowest.octave + i as i16 + 1;
                    }
                }
            }

            notes.sort_unstable();
        }

        debug_assert!(
            notes.windows(2).all(|w| w[0] < w[1]),
            "should be sorted & deduplicated after inversion",
        );

        Some(Self { notes, root: self.root })
    }

    fn closed_root_position(&self, separate_steps: bool) -> Self {
        let deduped = self.dedup_by_pitch();

        let mut notes = deduped.notes.to_vec();

        let mut closed = Vec::with_capacity(deduped.len());

        let mut letters_added = LetterSet::EMPTY;

        // 1. add root(s)
        let root = {
            let root_pos = notes.iter()
                .position(|n| n.pitch == deduped.root)
                .expect("root should exist in chord");

            notes.remove(root_pos)
        };

        closed.push(root);
        letters_added = letters_added.with_set(root.pitch.letter());

        let roots = notes.extract_if(.., |n|
            n.pitch.letter() == deduped.root.letter()
        );

        for (i, note) in roots.enumerate() {
            let octave = root.octave + 1;
            let new_octave = if separate_steps { octave + i as i16 } else { octave };

            closed.push(Note::new(note.pitch, new_octave));
        }

        // 2. find thirds
        let mut last_letter = root.pitch.letter();
        let mut last_octave = root.octave;

        loop {
            if letters_added == LetterSet::FULL {
                break;
            }

            let third_letter = Letter::from_step((last_letter.step() + 2) % 7)
                .expect("should be valid letter");

            let thirds = notes.extract_if(.., |n|
                n.pitch.letter() == third_letter
            );

            let octave = match last_letter.cmp(&third_letter) {
                Ordering::Less => last_octave,
                Ordering::Greater => last_octave + 1,
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
                last_letter = third_letter;
                last_octave = octave;
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
                bass.pitch, deduped.root,
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

        closed.sort_unstable();

        assert!(
            closed.windows(2).all(|w| w[0] < w[1]),
            "should be sorted & deduplicated",
        );

        Self { notes: closed.into_boxed_slice(), root: deduped.root }
    }

    pub fn closed_position(&self, separate_steps: bool) -> Self {
        let closed_root = self.closed_root_position(separate_steps);

        if self.root == self.bass().pitch {
            return closed_root
        }

        // 4. inversion
        // ensure state is right before reordering
        if cfg!(debug_assertions) {
            let bass_letter = self.bass().pitch.letter();

            let first_bass = self.notes.iter().find(|n| n.pitch.letter() == bass_letter)
                .expect("bass letter should exist in chord");

            assert_eq!(
                first_bass.pitch, self.bass().pitch,
                "before reordering for inversion, make sure the right note is in the bass"
            )
        }

        let bass = self.bass();

        let bass_idx = closed_root.notes.iter()
            .position(|n| n.pitch == bass.pitch)
            .expect("bass pitch must be in closed voicing");

        assert_ne!(
            bass_idx, 0,
            "bass shouldn't be root",
        );

        let mut with_inversion = closed_root.with_inversion(bass_idx, separate_steps).expect("valid inversion");

        let octave_diff = self.bass().octave - with_inversion.bass().octave;

        for note in &mut with_inversion.notes {
            note.octave += octave_diff;
        }

        assert_eq!(
            with_inversion.bass(), self.bass(),
            "bass note should match after octave adjustment",
        );

        with_inversion
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