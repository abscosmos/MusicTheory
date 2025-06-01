use std::cmp::Ordering;
use std::fmt;
use crate::letter::Letter;
use crate::note::Note;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct OctaveLetter {
    pub letter: Letter,
    pub octave: i16,
}

impl fmt::Display for OctaveLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Note::from(*self))
    }
}

impl From<OctaveLetter> for Note {
    fn from(oct_letter: OctaveLetter) -> Self {
        Note::new(oct_letter.letter.into(), oct_letter.octave)
    }
}

impl Ord for OctaveLetter{
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.octave.cmp(&rhs.octave).then(self.letter.cmp(&rhs.letter))
    }
}

impl PartialOrd for OctaveLetter {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

