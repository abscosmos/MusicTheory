use std::cmp::Ordering;
use std::fmt;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::interval::quality::IntervalQuality;
use crate::interval::size::IntervalSize;
use crate::semitone::Semitone;

pub mod quality;
pub mod size;
pub mod helper;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Interval {
    quality: IntervalQuality,
    size: IntervalSize,
}

impl Interval {
    pub fn from_quality_and_size(quality: IntervalQuality, size: IntervalSize) -> Option<Interval> {
        use IntervalQuality as Q;

        let unchecked = Self { quality, size };

        match quality {
            Q::Perfect => size.is_perfect().then_some(unchecked),

            Q::Major | Q::Minor => (!size.is_perfect()).then_some(unchecked),

            Q::Diminished => (size as u8 > 1).then_some(unchecked),

            Q::DoublyDiminished => (size as u8 > 2).then_some(unchecked),

            Q::Augmented | Q::DoublyAugmented => Some(unchecked),
        }
    }

    pub fn size(&self) -> IntervalSize {
        self.size
    }

    pub fn quality(&self) -> IntervalQuality {
        self.quality
    }

    pub fn semitones(&self) -> Semitone {
        use IntervalSize as S;
        use IntervalQuality as Q;

        let base_semitones = match self.size {
            S::Unison => 0,
            S::Second => 2,
            S::Third => 4,
            S::Fourth => 5,
            S::Fifth => 7,
            S::Sixth => 9,
            S::Seventh => 11,
            S::Octave => 12,

            S::Ninth => 14,
            S::Tenth => 16,
            S::Eleventh => 17,
            S::Twelfth => 19,
            S::Thirteenth => 21,
            S::Fourteenth => 23,
            S::Fifteenth => 24,
        };

        let quality_adjustment = match self.quality {
            Q::Perfect => 0,
            Q::Major => 0,
            Q::Minor => -1,

            Q::Diminished => match self.size {
                S::Fourth | S::Fifth | S::Octave |
                S::Eleventh | S::Twelfth | S::Fifteenth => -1,
                _ => -2
            },

            Q::DoublyDiminished => match self.size {
                S::Fourth | S::Fifth | S::Octave |
                S::Eleventh | S::Twelfth | S::Fifteenth => -2,
                _ => -3
            }

            Q::Augmented => 1,
            Q::DoublyAugmented => 2,
        };

        Semitone(base_semitones + quality_adjustment)
    }

    pub fn shorthand(&self) -> String {
        format!("{}{}", self.quality.shorthand(), self.size.shorthand())
    }

    // TODO: there's probably a better way to do this
    pub fn from_semitones_preferred(semitones: Semitone) -> Option<Self> {
        use crate::interval as ivl;

        match semitones.0 {
            0 => Some(ivl!(Perfect Unison)),
            1 => Some(ivl!(Minor Second)),
            2 => Some(ivl!(Major Second)),
            3 => Some(ivl!(Minor Third)),
            4 => Some(ivl!(Major Third)),
            5 => Some(ivl!(Perfect Fourth)),
            6 => Some(ivl!(Diminished Fifth)),
            7 => Some(ivl!(Perfect Fifth)),
            8 => Some(ivl!(Minor Sixth)),
            9 => Some(ivl!(Major Sixth)),
            10 => Some(ivl!(Minor Seventh)),
            11 => Some(ivl!(Major Seventh)),
            12 => Some(ivl!(Perfect Octave)),

            13 => Some(ivl!(Minor Ninth)),
            14 => Some(ivl!(Major Ninth)),
            15 => Some(ivl!(Minor Tenth)),
            16 => Some(ivl!(Major Tenth)),
            17 => Some(ivl!(Perfect Eleventh)),
            18 => Some(ivl!(Diminished Twelfth)),
            19 => Some(ivl!(Perfect Twelfth)),
            20 => Some(ivl!(Minor Thirteenth)),
            21 => Some(ivl!(Major Thirteenth)),
            22 => Some(ivl!(Minor Fourteenth)),
            23 => Some(ivl!(Major Fourteenth)),
            24 => Some(ivl!(Perfect Fifteenth)),

            _ => None,
        }
    }

    // TODO: this method breaks the invariance of the Interval (can create invalid diminished intervals)
    pub fn inverted(&self) -> Self {
        Interval {
            size: self.size.inverted(),
            quality: self.quality.inverted(),
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