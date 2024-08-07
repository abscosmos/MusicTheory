use crate::enharmonic::EnharmonicEq;
use crate::interval::quality::IntervalQuality;
use crate::interval::size::IntervalSize;
use crate::semitone::Semitone;

pub mod quality;
pub mod size;

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

    // TODO: this method breaks the invariance of the Interval (can create invalid diminished intervals)
    pub fn inverted(&self) -> Self {
        Interval {
            size: self.size.inverted(),
            quality: self.quality.inverted(),
        }
    }
}

impl EnharmonicEq for Interval {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.semitones() == rhs.semitones()
    }
}