use crate::note::Note;

pub struct Freeform {
    elements: Vec<ContainerElement>,
}

pub enum ContainerElement {
    Note {
        note: Note,
        duration: (),
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

