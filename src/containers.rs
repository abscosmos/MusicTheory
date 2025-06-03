use crate::clef::PitchClef;
use crate::duration::Duration;
use crate::key::Key;
use crate::note::Note;

#[derive(Default, Debug, Clone)]
pub struct Freeform {
    elements: Vec<(Duration, ContainerElement)>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContainerElement {
    Note {
        note: Note,
        duration: Duration,
        accidental: AccidentalDisplay,
    },
    Rest {
        duration: Duration,
        implicit: bool,
    },
    KeySignature(Key),
    Clef(PitchClef),
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum AccidentalDisplay {
    #[default]
    Normal,
    Courtesy,
    Implied,
}

