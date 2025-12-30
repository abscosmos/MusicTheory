use crate::note::Note;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NoteGenerator {
    current: i32,
    reverse: bool,
}


