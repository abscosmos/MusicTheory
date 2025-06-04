use std::mem;
use std::ops::Add;
use crate::clef::PitchClef;
use crate::duration::Duration;
use crate::key::Key;
use crate::note::Note;
use crate::pitch::Pitch;

// represents duration as offset from container start
pub type Offset = Duration;

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
        use ContainerElement as Elem;

        if self.elements.is_empty() {
            return match elem {
                Elem::Clef(_) => {
                    self.elements.push((Offset::ZERO, elem));
                    Ok(())
                },
                _ => Err(FreeformInsertError::FirstNotClef)
            };
        }
        
        assert!(
            !self.trailing_implicit_rest_exists(),
            "Trailing implicit rests should not exist",
        );

        match elem {
            Elem::Rest { implicit: true, .. } => return Err(FreeformInsertError::TrailingImplicitRest),
            Elem::Note { .. } | Elem::Rest { .. } => self.elements.push((self.next_insertion_point(), elem)),
            Elem::KeySignature(_) | Elem::Clef(_) => {
                let ins_at = self.next_insertion_point();
                
                match self.elements.iter_mut()
                    .rfind(|(offset, el)|
                        *offset == ins_at && mem::discriminant(el) == mem::discriminant(&elem)
                    )
                {
                    None => self.elements.push((ins_at, elem)),
                    Some((_, el)) => {
                        let _prev = mem::replace(el, elem);
                    },
                }
            },
        }

        assert!(
            !self.trailing_implicit_rest_exists(),
            "Trailing implicit rests should not exist",
        );
        
        Ok(())
    }
    
    pub fn insert(&mut self, offset: Offset, elem: ContainerElement) -> Result<(), FreeformInsertError> {
        /*
            - no trailing rests
            - zero duration (clef/key sig) must come before duration having; priority
                - clef
                - key signature
                - notes / rests
        */
        
        todo!()
    }

    fn next_insertion_point(&self) -> Offset {
        match self.elements.last() {
            Some((offset, elem)) => *offset + elem.duration().unwrap_or(Duration::ZERO),
            None => Offset::ZERO,
        }
    }

    fn trailing_implicit_rest_exists(&self) -> bool {
        self.elements()
            .last()
            .is_none_or(|(_, el)|
                matches!(el, ContainerElement::Rest { implicit: true, .. })
            )
    }
    
    pub fn duration(&self) -> Duration {
        let Some((last_offset, last)) = self.elements.last() else {
            return Duration::ZERO;
        };
        
        let dur_from_last = *last_offset + last.duration().unwrap_or(Duration::ZERO);
        
        let dur_sum = self.elements.iter()
            .flat_map(|(_, ce)| ce.duration())
            .fold(Duration::ZERO, Add::add);
        
        assert_eq!(
            dur_from_last, dur_sum,
            "The sum of all durations should be the same as duration from the container start to the last element's end"
        );
        
        dur_sum
    }
    
    pub fn elements(&self) -> &[(Offset, ContainerElement)] {
        &self.elements
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FreeformInsertError {
    #[error("The first element in a freeform must be a clef")]
    FirstNotClef,
    #[error("Implicit rests at freeform's end are not allowed")]
    TrailingImplicitRest,
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
    use num_rational::Ratio;
    use crate::duration::{WrittenDuration as WD};
    use super::{*, ContainerElement as CE};
    
    #[test]
    fn test_freeform_push() -> Result<(), FreeformInsertError> {
        // automatically adds clef & key
        let mut freeform = Freeform::default();
        
        let b3 = Note::new(Pitch::B, 3);
        let c4 = Note::MIDDLE_C;
        let ef4 = Note::new(Pitch::E_FLAT, 4);
        let ds5 = Note::new(Pitch::D_SHARP, 5);
        
        freeform.push(CE::note(b3, WD::HALF))?;
        freeform.push(CE::note(c4, WD::EIGHTH))?;
        freeform.push(CE::note(ds5, WD::EIGHTH))?;
        freeform.push(CE::note(b3, WD::QUARTER))?;
        freeform.push(CE::note(ef4, WD::WHOLE))?;

        assert_eq!(
            freeform.elements(),
            [
                // automatically added
                (Offset::ZERO, CE::Clef(PitchClef::TREBLE)),
                (Offset::ZERO, CE::KeySignature(Key::major(Pitch::C))),
                
                (Offset::ZERO, CE::note(b3, WD::HALF)),
                (Offset::new(Ratio::new(1,2)), CE::note(c4, WD::EIGHTH)),
                (Offset::new(Ratio::new(5,8)), CE::note(ds5, WD::EIGHTH)),
                (Offset::new(Ratio::new(3,4)), CE::note(b3, WD::QUARTER)),
                (Offset::new(Ratio::from_integer(1)), CE::note(ef4, WD::WHOLE)),
            ]
        );
        
        assert_eq!(freeform.duration(), WD::DOUBLE_WHOLE.duration());
        
        Ok(())
    }
}
