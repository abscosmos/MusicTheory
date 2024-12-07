use std::cmp::Ordering;
use std::fmt;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::interval::number::IntervalNumber;
use crate::interval::quality::IntervalQuality;
use crate::semitone::Semitone;

pub mod quality;
pub mod number;
pub mod consts;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Interval {
    quality: IntervalQuality,
    number: IntervalNumber,
}

impl Interval {
    pub fn new(quality: IntervalQuality, number: IntervalNumber) -> Option<Self> {
        use IntervalQuality as Q;

        let unchecked = Some(Self { quality, number });

        match quality {
            Q::Diminished(_) | Q::Augmented(_) => unchecked,
            Q::Perfect if number.is_perfect() => unchecked,
            Q::Minor | Q::Major if !number.is_perfect() => unchecked,
            _ => None,
        }
    }

    pub fn strict_non_subzero(quality: IntervalQuality, number: IntervalNumber) -> Option<Self> {
        match Self::new(quality, number)? {
            Interval { quality: IntervalQuality::Diminished(n), .. } if number.number().abs() <= n.get() as _ => None,
            ivl => Some(ivl),
        }
    }

    pub fn quality(&self) -> IntervalQuality {
        self.quality
    }

    pub fn number(&self) -> IntervalNumber {
        self.number
    }

    pub fn semitones(&self) -> Semitone {
        let base_semis = match self.number.as_simple().number().abs() {
            1 => 0,
            2 => 2,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 9,
            7 => 11,
            8 => 12,
            _ => unreachable!("abs of as_simple must be within [1,8]"),
        };

        let with_octave = base_semis + self.number.octave_unsigned() * 12;

        use IntervalQuality as Q;

        let quality_adjust = match self.quality {
            Q::Perfect | Q::Major => 0,
            Q::Minor => -1,

            Q::Diminished(n) => if self.number.is_perfect() {
                -(n.get() as i16)
            } else {
                -(n.get() as i16 + 1)
            }

            Q::Augmented(n) => n.get() as i16,
        };

        let unsigned = with_octave + quality_adjust;

        Semitone(unsigned * self.number.number().signum())
    }

    pub fn shorthand(&self) -> String {
        format!("{}{}", self.quality.shorthand(), self.number.shorthand())
    }

    pub fn inverted(&self) -> Self {
        Self::new(self.quality.inverted(), self.number.inverted())
            .expect("valid quality")
    }

    pub fn inverted_strict_non_subzero(&self) -> Option<Self> {
        let inv = self.inverted();

        Self::strict_non_subzero(inv.quality, inv.number)
    }

    pub fn from_semitones_preferred(semitones: Semitone) -> Self {
        let semi = semitones.0;

        if semi == 0 {
            return Self::PERFECT_UNISON;
        }

        let semi_adj = (semi.abs() - 1) % 12;

        use IntervalQuality as Q;

        let (quality, base_num) = match semi_adj + 1 {
            1 => (Q::Minor, 2),
            2 => (Q::Major, 2),
            3 => (Q::Minor, 3),
            4 => (Q::Major, 3),
            5 => (Q::Perfect, 4),
            6 => (Q::DIMINISHED, 5),
            7 => (Q::Perfect, 5),
            8 => (Q::Minor, 6),
            9 => (Q::Major, 6),
            10 => (Q::Minor, 7),
            11 => (Q::Major, 7),
            12 => (Q::Perfect, 8),
            _ => unreachable!("should be in range [1,12]"),
        };

        let oct = semi_adj / 12;

        let number = IntervalNumber::new(base_num + oct * 7)
            .expect("non zero");

        Self::new(quality, number)
            .expect("valid quality")
    }
    
    pub fn is_ascending(&self) -> bool {
        self.number.is_ascending()
    }
    
    pub fn with_direction(&self, ascending: bool) -> Self {
        Self {
            number: self.number.with_direction(ascending),
            .. *self
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shorthand())
    }
}

impl EnharmonicEq for Interval {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.semitones() == rhs.semitones()
    }
}

impl EnharmonicOrd for Interval {
    fn cmp_enharmonic(&self, rhs: &Self) -> Ordering {
        self.semitones().0.cmp(&rhs.semitones().0)
    }
}