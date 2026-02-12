use crate::chord::pitch::PitchChord;
use crate::{Note, Pitch};
use crate::chord::root;

pub struct NoteChord {
    notes: Box<[Note]>,
    root: Pitch,
}

impl NoteChord {
    pub fn new(notes: impl IntoIterator<Item=Note>) -> Option<Self> {
        let notes = notes.into_iter().collect::<Box<[_]>>();
        
        if notes.is_empty() {
            return None;
        }
        
        let root = root::find_root(notes.iter().map(|n| n.pitch))
            .expect("not empty");
        
        Some(Self { notes, root })
    }

    pub fn with_root(notes: impl IntoIterator<Item=Note>, root: Pitch) -> Option<Self> {
        let notes = notes.into_iter().collect::<Box<[_]>>();

        if notes.iter().any(|n| n.pitch == root) {
            Some(Self { notes, root })
        } else {
            None
        }
    }

    pub fn from_pitch_chord(chord: &PitchChord, bass_octave: i16) -> Self {
        Self::with_root(chord.notes(bass_octave), chord.root()).expect("can't be empty")
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }
}