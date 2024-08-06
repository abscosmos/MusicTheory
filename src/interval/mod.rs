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
        use IntervalSize as S;
        use IntervalQuality as Q;

        // TODO: size::can_be_perfect(), etc.
        match (quality, size) {
            (Q::Perfect, S::Unison) |
            (Q::Perfect, S::Fourth) |
            (Q::Perfect, S::Fifth) |
            (Q::Perfect, S::Octave) |
            (Q::Perfect, S::Eleventh) |
            (Q::Perfect, S::Twelfth) |
            (Q::Perfect, S::Fifteenth) |

            (Q::Major, S::Second) |
            (Q::Major, S::Third) |
            (Q::Major, S::Sixth) |
            (Q::Major, S::Seventh) |
            (Q::Major, S::Ninth) |
            (Q::Major, S::Tenth) |
            (Q::Major, S::Thirteenth) |
            (Q::Major, S::Fourteenth) |

            (Q::Minor, S::Second) |
            (Q::Minor, S::Third) |
            (Q::Minor, S::Sixth) |
            (Q::Minor, S::Seventh) |
            (Q::Minor, S::Ninth) |
            (Q::Minor, S::Tenth) |
            (Q::Minor, S::Thirteenth) |
            (Q::Minor, S::Fourteenth) |

            (Q::Diminished, S::Second) |
            (Q::Diminished, S::Third) |
            (Q::Diminished, S::Fourth) |
            (Q::Diminished, S::Fifth) |
            (Q::Diminished, S::Sixth) |
            (Q::Diminished, S::Seventh) |
            (Q::Diminished, S::Octave) |
            (Q::Diminished, S::Ninth) |
            (Q::Diminished, S::Tenth) |
            (Q::Diminished, S::Eleventh) |
            (Q::Diminished, S::Twelfth) |
            (Q::Diminished, S::Thirteenth) |
            (Q::Diminished, S::Fourteenth) |
            (Q::Diminished, S::Fifteenth) |

            (Q::Augmented, _) |

            (Q::DoublyDiminished, S::Third) |
            (Q::DoublyDiminished, S::Fourth) |
            (Q::DoublyDiminished, S::Fifth) |
            (Q::DoublyDiminished, S::Sixth) |
            (Q::DoublyDiminished, S::Seventh) |
            (Q::DoublyDiminished, S::Octave) |
            (Q::DoublyDiminished, S::Ninth) |
            (Q::DoublyDiminished, S::Tenth) |
            (Q::DoublyDiminished, S::Eleventh) |
            (Q::DoublyDiminished, S::Twelfth) |
            (Q::DoublyDiminished, S::Thirteenth) |
            (Q::DoublyDiminished, S::Fourteenth) |
            (Q::DoublyDiminished, S::Fifteenth) |

            (Q::DoublyAugmented, _) => Some(Self { size, quality }),

            _ => None
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
}

impl EnharmonicEq for Interval {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.semitones() == rhs.semitones()
    }
}