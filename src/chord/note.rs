use crate::chord::pitch::PitchChord;
use crate::Note;

pub struct NoteChord {
    pub notes: Vec<Note>,
}

impl NoteChord {
    // TODO: the existence of this method (and pub notes field) allows empty chords; is this valid?
    pub fn new(notes: impl IntoIterator<Item=Note>) -> Self {
        Self { notes: notes.into_iter().collect() }
    }

    pub fn from_pitch_chord(chord: &PitchChord, bass_octave: i16) -> Self {
        Self::new(chord.notes(bass_octave))
    }
}