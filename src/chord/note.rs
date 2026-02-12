use crate::chord::pitch::PitchChord;
use crate::{Interval, Note, Pitch};
use crate::chord::root;
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