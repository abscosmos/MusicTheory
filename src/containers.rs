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
    Rest {
        offset: Duration,
        duration: Duration,
        implicit: bool,
    },
}

impl ContainerElement {
    pub fn offset(&self) -> Duration {
        match self {
            ContainerElement::Note { offset, .. } => *offset,
            ContainerElement::Rest { offset, .. } => *offset,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum AccidentalDisplay {
    #[default]
    Normal,
    Courtesy,
    Implied,
}

