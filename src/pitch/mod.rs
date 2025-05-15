use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use regex::Regex;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::interval::Interval;
use crate::interval::IntervalQuality;
use crate::accidental::AccidentalSign;
use crate::letter::Letter;
use crate::pitch_class::PitchClass;
use crate::semitone::Semitone;

mod consts;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pitch(pub i16);

// TODO: change taking in &self -> self
impl Pitch {
    pub fn from_letter_and_accidental(letter: Letter, accidental_sign: AccidentalSign) -> Self {
        let col_offset = accidental_sign.offset;

        let pitch = letter.fifths_from_c() + 7 * col_offset;

        Self::from_fifths_from_c(pitch)
    }

    pub fn as_fifths_from_c(&self) -> i16 {
        self.0
    }

    pub fn from_fifths_from_c(fifths: i16) -> Self {
        Self(fifths)
    }

    pub fn as_pitch_class(&self) -> PitchClass {
        let fifths_plus_one = self.as_fifths_from_c() + 1;

        let semitone_offset = fifths_plus_one.div_euclid(7);

        let semitones_from_c = match fifths_plus_one.rem_euclid(7) {
            n if n % 2 == 0 => n + 5,
            n => n - 1,
        } + semitone_offset;

        let semitones_from_c = semitones_from_c
            .rem_euclid(12)
            .try_into()
            .expect("i8::rem_euclid(12) must be [0, 12)");

        PitchClass::from_repr(semitones_from_c)
            .expect("i8::rem_euclid(12) must be [0, 12)")
    }

    pub fn chroma(&self) -> u8 {
        self.as_pitch_class().chroma()
    }

    // TODO: is this right?
    pub fn semitones_to(&self, rhs: Self) -> Semitone {
        let lhs = self.as_pitch_class() as u8 as i8;
        let rhs = rhs.as_pitch_class() as u8 as i8;

        Semitone((rhs - lhs).rem_euclid(12) as _)
    }

    pub fn letter(&self) -> Letter {
        match (self.as_fifths_from_c() + 1).rem_euclid(7) {
            0 => Letter::F,
            1 => Letter::C,
            2 => Letter::G,
            3 => Letter::D,
            4 => Letter::A,
            5 => Letter::E,
            6 => Letter::B,
            _ => unreachable!("i8::rem_euclid(7) must be [0, 7)"),
        }
    }

    pub fn accidental(&self) -> AccidentalSign {
        AccidentalSign {
            offset: (self.as_fifths_from_c() + 1).div_euclid(7)
        }
    }

    pub fn semitones_offset_from_c(&self) -> Semitone {
        let fifths_plus_one = self.as_fifths_from_c() + 1;

        let n = match fifths_plus_one.rem_euclid(7) {
            0 => 5, // F
            1 => 0, // C
            2 => 7, // G
            3 => 2, // D
            4 => 9, // A
            5 => 4, // E
            6 => 11, // B
            _ => unreachable!("i8::rem_euclid(7) must be [0, 7)")
        } + fifths_plus_one.div_euclid(7);

        Semitone(n as _)
    }

    pub fn simplified(&self) -> Self {
        self.as_pitch_class().bias(self.accidental().offset > 0)
    }

    pub fn enharmonic(&self) -> Self {
        self.as_pitch_class().bias(self.accidental().offset < 0)
    }
    
    // TODO: should this function simplify if called with G## & true?
    pub fn bias(&self, sharp: bool) -> Self {
        self.as_pitch_class().bias(sharp)
    }

    pub fn transpose(&self, interval: Interval) -> Self {
        use IntervalQuality as Q;
        
        let start = match interval.number().as_simple().number().abs() {
            1 | 8 => 7,
            2 => 9,
            3 => 11,
            4 => 6,
            5 => 8,
            6 => 10,
            7 => 12,
            _ => unreachable!("a simple interval can't be bigger than a octave")
        };

        let quality_offset = match interval.quality() {
            Q::Augmented(n) => -(n.get() as i16 - 1),
            Q::Perfect | Q::Major => 1,
            Q::Minor => 2,
            
            Q::Diminished(n) => match interval.number().is_perfect() {
                true => n.get() as i16 + 1,
                false => n.get() as i16 + 2,
            }
        };

        let offset = start - 7 * quality_offset;

        let dir_offset = if interval.is_ascending() { offset } else { -offset };

        self.transpose_fifths(dir_offset)
    }
    
    pub fn distance_to(&self, rhs: &Self) -> Interval {
        Interval::between_pitches(*self, *rhs)
    }

    pub fn with_accidental(&self, accidental: AccidentalSign) -> Self {
        let letter = self.letter();

        Self::from_letter_and_accidental(letter, accidental)
    }

    pub fn with_letter(&self, letter: Letter) -> Self {
        let accidental = self.accidental();

        Self::from_letter_and_accidental(letter, accidental)
    }

    pub fn transpose_fifths(&self, fifths: i16) -> Self {
        let curr = self.as_fifths_from_c();

        Self::from_fifths_from_c(curr + fifths)
    }
}

impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = self.letter();
        let accidental = self.accidental();

        if accidental != AccidentalSign::NATURAL {
            write!(f, "{letter:?}{accidental:?}")
        } else {
            write!(f, "{letter:?}")
        }
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = self.letter();
        let accidental = self.accidental();

        if accidental != AccidentalSign::NATURAL {
            write!(f, "{letter}{accidental}")
        } else {
            write!(f, "{letter}")
        }
    }
}

impl EnharmonicEq for Pitch {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.as_pitch_class() == rhs.as_pitch_class()
    }
}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Pitch {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.letter()
            .cmp(&rhs.letter())
            .then(
                self.accidental()
                    .cmp(&rhs.accidental())
            )
    }
}

impl EnharmonicOrd for Pitch {
    fn cmp_enharmonic(&self, rhs: &Self) -> Ordering {
        self.as_pitch_class()
            .cmp(&rhs.as_pitch_class())
    }
}

impl From<PitchClass> for Pitch {
    fn from(value: PitchClass) -> Self {
        match value {
            PitchClass::C => Pitch::C,
            PitchClass::Cs => Pitch::C_SHARP,
            PitchClass::D => Pitch::D,
            PitchClass::Ds => Pitch::D_SHARP,
            PitchClass::E => Pitch::E,
            PitchClass::F => Pitch::F,
            PitchClass::Fs => Pitch::F_SHARP,
            PitchClass::G => Pitch::G,
            PitchClass::Gs => Pitch::G_SHARP,
            PitchClass::A => Pitch::A,
            PitchClass::As => Pitch::A_SHARP,
            PitchClass::B => Pitch::B,
        }
    }
}

impl From<Letter> for Pitch {
    fn from(letter: Letter) -> Self {
        Self::from_letter_and_accidental(letter, AccidentalSign::NATURAL)
    }
}

impl From<Pitch> for PitchClass {
    fn from(pitch: Pitch) -> Self {
        pitch.as_pitch_class()
    }
}

impl From<Pitch> for Letter {
    fn from(pitch: Pitch) -> Self {
        pitch.letter()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The string could not be converted to a pitch")]
pub struct PitchFromStrError;

// TODO: add support for pitches like F(25x)Flat
// TODO: change to make more like tonaljs/note's Note::name
impl FromStr for Pitch {
    type Err = PitchFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?i)^([A-G])\s?((?-i)b|(?-i)bb|(?i)sharp|♯|\+|\++|#|##|♯♯|𝄪|flat|♭|-|--|♭♭|𝄫|double\s?sharp|double\s?flat)?$")
            .expect("valid regex");

        let caps = re.captures(s)
            .ok_or(PitchFromStrError)?;

        let letter = caps.get(1)
            .ok_or(PitchFromStrError)?
            .as_str()
            .parse()
            .map_err(|_| PitchFromStrError)?;

        let accidental = caps.get(2);

        let acc = match accidental {
            None => AccidentalSign::NATURAL,
            Some(acc) => match acc
                .as_str()
                .trim()
                .to_lowercase()
                .as_str()
            {
                "+" | "#" | "♯" | "sharp" => AccidentalSign::SHARP,
                "-" | "b" | "♭" | "flat" => AccidentalSign::FLAT,
                "++" | "##" | "♯♯" | "𝄪" | "double sharp" | "doublesharp" => AccidentalSign::DOUBLE_SHARP,
                "--" | "bb" | "♭♭" | "𝄫" | "double flat" | "doubleflat" => AccidentalSign::DOUBLE_FLAT,
                _ => unreachable!("all cases should be covered"),
            }
        };

        Ok(Self::from_letter_and_accidental(letter, acc))
    }
}

impl Add<Interval> for Pitch {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose(rhs)
    }
}

impl Sub<Interval> for Pitch {
    type Output = Self;

    fn sub(self, rhs: Interval) -> Self::Output {
        self + (-rhs)
    }
}