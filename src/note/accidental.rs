use std::fmt;
use crate::semitone::Semitone;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AccidentalSign {
    Natural, // ♮
    Sharp, // #
    Flat, // ♭
    DoubleSharp, // 𝄪
    DoubleFlat, // 𝄫
}

impl AccidentalSign {
    pub fn as_char(&self) -> char {
        use AccidentalSign as AS;

        match self {
            AS::Natural => '♮',
            AS::Sharp => '#',
            AS::Flat => '♭',
            AS::DoubleSharp => '𝄪',
            AS::DoubleFlat => '𝄫'
        }
    }

    pub fn as_offset(&self) -> Semitone {
        use AccidentalSign as AS;

        match self {
            AS::Natural => Semitone(0),
            AS::Sharp => Semitone(1),
            AS::Flat => Semitone(-1),
            AS::DoubleSharp => Semitone(2),
            AS::DoubleFlat => Semitone(-2),
        }
    }

    pub fn from_offset(semitones: Semitone) -> Option<Self> {
        match semitones {
            Semitone(-2) => Some(Self::DoubleFlat),
            Semitone(-1) => Some(Self::Flat),
            Semitone(0) => Some(Self::Natural),
            Semitone(1) => Some(Self::Sharp),
            Semitone(2) => Some(Self::DoubleSharp),
            _ => None,
        }
    }
}

impl fmt::Display for AccidentalSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl From<AccidentalSign> for char {
    fn from(value: AccidentalSign) -> Self {
        value.as_char()
    }
}