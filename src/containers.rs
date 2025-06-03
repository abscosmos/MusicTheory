use crate::clef::PitchClef;
use crate::duration::Duration;
use crate::key::Key;
use crate::note::Note;
use crate::pitch::Pitch;

// represents duration as offset from container start
type Offset = Duration;

#[derive(Debug, Clone)]
pub struct Freeform {
    elements: Vec<(Offset, ContainerElement)>,
}

impl Freeform {
    pub fn with_clef_and_key_signature(clef: PitchClef, key_sig: Key) -> Self {
        let elements = vec![
            (Offset::ZERO, ContainerElement::Clef(clef)),
            (Offset::ZERO, ContainerElement::KeySignature(key_sig)),
        ];
        
        Self { elements }
    }
    
    pub fn push(&mut self, elem: ContainerElement) -> Result<(), FreeformInsertError> {
        match self.elements.last() {
            None => {
                if matches!(elem, ContainerElement::Clef(_)) {
                    self.elements.push((Offset::ZERO, elem));
                } else {
                    return Err(FreeformInsertError::FirstNotClef);
                }
            }
            Some((offset, last)) => {
                // TODO: remove any implicit rests
                let insert_at = *offset + last.duration().unwrap_or(Duration::ZERO);
                self.elements.push((insert_at, elem));
            }
        }
        
        Ok(())
    }
    
    pub fn elements(&self) -> &[(Offset, ContainerElement)] {
        &self.elements
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FreeformInsertError {
    #[error("The first element in a freeform must be a clef")]
    FirstNotClef,
}

impl Default for Freeform {
    fn default() -> Self {
        Self::with_clef_and_key_signature(PitchClef::TREBLE, Key::major(Pitch::C))
    }
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

impl ContainerElement {
    pub fn note(note: Note, duration: impl Into<Duration>) -> Self {
        Self::Note { note, duration: duration.into(), accidental: AccidentalDisplay::default() }
    }
    
    pub fn rest(duration: impl Into<Duration>) -> Self {
        Self::Rest { duration: duration.into(), implicit: false }
    }
    
    pub fn duration(&self) -> Option<Duration> {
        match self {
            Self::Note { duration, .. } => Some(*duration),
            Self::Rest { duration, .. } => Some(*duration),
            Self::KeySignature(_) => None,
            Self::Clef(_) => None,
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

#[cfg(test)]
mod tests {
    use crate::duration::WrittenDuration;
    use super::{*, ContainerElement as CE};
    
    #[test]
    fn test_freeform_push() -> Result<(), FreeformInsertError> {
        // automatically adds clef & key
        let mut freeform = Freeform::default();
        
        let b3 = Note::new(Pitch::B, 3);
        let ds5 = Note::new(Pitch::D_SHARP, 5);
        
        freeform.push(CE::note(b3, WrittenDuration::HALF))?;

        assert_eq!(
            freeform.elements(),
            [
                // automatically added
                (Offset::ZERO, CE::Clef(PitchClef::TREBLE)),
                (Offset::ZERO, CE::KeySignature(Key::major(Pitch::C))),
                
                (Offset::ZERO, CE::note(b3, WrittenDuration::HALF)),
            ]
        );
        
        Ok(())
    }
}
