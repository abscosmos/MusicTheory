use crate::duration::Duration;
use crate::note::Note;

pub struct Freeform {
    elements: Vec<ContainerElement>,
}

pub enum ContainerElement {
    Note {
        // offset as duration from container start
        offset: Duration,
        note: Note,
        duration: Duration,
        accidental: AccidentalDisplay,
    },
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum AccidentalDisplay {
    #[default]
    Normal,
    Courtesy,
    Implied,
}

