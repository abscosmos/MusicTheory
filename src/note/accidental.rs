use std::fmt;
use crate::semitone::Semitone;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AccidentalSign {
    pub offset: i16,
}

impl AccidentalSign {
    pub const DOUBLE_FLAT: Self = Self { offset: -2 };
    pub const FLAT: Self = Self { offset: -1 };
    pub const NATURAL: Self = Self { offset: 0 };
    pub const SHARP: Self = Self { offset: 1 };
    pub const DOUBLE_SHARP: Self = Self { offset: 2 };

    pub fn as_semitone_offset(&self) -> Semitone {
        Semitone(self.offset)
    }

    pub fn from_semitone_offset(offset: Semitone) -> Self {
        Self { offset: offset.0 }
    }
}

impl fmt::Debug for AccidentalSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num = match self.offset.abs() {
            0 | 1 => "".to_owned(),
            2 => "Double".to_owned(),
            3 => "Triple".to_owned(),
            4 => "Quadruple".to_owned(),
            5 => "Quintuple".to_owned(),
            n => format!("({n}x)"),
        };

        let ty = match self.offset.signum() {
            0 => "Natural",
            1 => "Sharp",
            -1 => "Flat",
            _ => unreachable!(".signum() only returns -1, 0, 1")
        };

        write!(f, "{num}{ty}")
    }
}

impl fmt::Display for AccidentalSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offset = self.offset;

        if offset == 0 {
            write!(f, "♮")
        } else {
            let num_double = offset.abs() / 2;
            let add_single = offset.abs() % 2 == 1;

            let (d, s) = if offset > 0 {
                ("𝄪", "♯")
            } else {
                ("𝄫", "♭")
            };

            let single = if add_single { s } else { "" };
            let double = d.repeat(num_double as _);

            write!(f, "{single}{double}")
        }
    }
}

impl From<Semitone> for AccidentalSign {
    fn from(value: Semitone) -> Self {
        Self::from_semitone_offset(value)
    }
}

impl From<AccidentalSign> for Semitone {
    fn from(value: AccidentalSign) -> Self {
        value.as_semitone_offset()
    }
}