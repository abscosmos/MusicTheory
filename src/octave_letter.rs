use std::cmp::Ordering;
use std::fmt;
use crate::letter::Letter;
use crate::note::Note;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct OctaveLetter {
    pub letter: Letter,
    pub octave: i16,
}

impl OctaveLetter {
    pub fn new(letter: Letter, octave: i16) -> Self {
        Self { letter, octave }
    }
    
    pub fn offset_to(self, rhs: Self) -> i16 {
        (rhs.octave - self.octave) * 7 + rhs.letter.step() as i16 - self.letter.step() as i16
    }
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

#[cfg(test)]
mod tests {
    use crate::letter::Letter;
    use crate::octave_letter::OctaveLetter;

    #[test]
    fn test_offset() {
        let c4 = OctaveLetter::new(Letter::C, 4);
        
        assert_eq!(c4.offset_to(OctaveLetter::new(Letter::F, 5)), 10);
        assert_eq!(OctaveLetter::new(Letter::B, 3).offset_to(c4), 1);
    }
}
