use std::cmp::Ordering;
use std::fmt;
use std::ops::Neg;
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
            1 | 8 => 0,
            2 => 2,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 9,
            7 => 11,
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

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            number: -self.number,
            .. self
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

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;
    use super::*;
    use Interval as I;
    use IntervalQuality as IQ;
    use IntervalNumber as IN;

    #[test]
    fn new() {
        for num in 1..25 {
            let num = IN::new(num).expect("nonzero");

            assert!(I::new(IQ::DIMINISHED, num).is_some());
            assert!(I::new(IQ::AUGMENTED, num).is_some());

            for adj in 1..12 {
                assert!(I::new(IQ::Diminished(NonZeroU16::new(adj).expect("nonzero")), num).is_some());
                assert!(I::new(IQ::Augmented(NonZeroU16::new(adj).expect("nonzero")), num).is_some());
            }
        }

        assert!(I::new(IQ::Major, IN::THIRD).is_some());
        assert!(I::new(IQ::Major, IN::THIRTEENTH).is_some());
        assert!(I::new(IQ::Major, IN::SECOND).is_some());

        assert!(I::new(IQ::Major, IN::FOURTH).is_none());
        assert!(I::new(IQ::Major, IN::TWELFTH).is_none());
        assert!(I::new(IQ::Major, IN::OCTAVE).is_none());

        assert!(I::new(IQ::Minor, IN::SIXTH).is_some());
        assert!(I::new(IQ::Minor, IN::NINTH).is_some());
        
        assert!(I::new(IQ::Minor, IN::ELEVENTH).is_none());
        assert!(I::new(IQ::Minor, IN::UNISON).is_none());
        
        assert!(I::new(IQ::Perfect, IN::FOURTH).is_some());
        assert!(I::new(IQ::Perfect, IN::FIFTEENTH).is_some());
        
        assert!(I::new(IQ::Perfect, IN::SECOND).is_none());
        assert!(I::new(IQ::Perfect, IN::SEVENTH).is_none());
    }
    
    #[test]
    fn non_subzero() {
        assert!(I::strict_non_subzero(IQ::DIMINISHED, IN::UNISON).is_none());
        
        for num in 2..15 {
            let num = IN::new(num).expect("nonzero");
            assert!(I::strict_non_subzero(IQ::DIMINISHED, num).is_some());
        }

        let doubly_diminished = IQ::Diminished(NonZeroU16::new(2).expect("nonzero"));
        
        assert!(I::strict_non_subzero(doubly_diminished, IN::UNISON).is_none());
        assert!(I::strict_non_subzero(doubly_diminished, IN::SECOND).is_none());

        for num in 3..15 {
            let num = IN::new(num).expect("nonzero");
            assert!(I::strict_non_subzero(doubly_diminished, num).is_some());
        }
    }
    
    #[test]
    fn semitones_constants() {
        let semi = |ivl: I| ivl.semitones().0;
        
        assert_eq!(semi(I::PERFECT_UNISON), 0);
        assert_eq!(semi(I::DIMINISHED_SECOND), 0);
        
        assert_eq!(semi(I::MINOR_SECOND), 1);
        assert_eq!(semi(I::AUGMENTED_UNISON), 1);
        
        assert_eq!(semi(I::MAJOR_SECOND), 2);
        assert_eq!(semi(I::DIMINISHED_THIRD), 2);
        
        assert_eq!(semi(I::MINOR_THIRD), 3);
        assert_eq!(semi(I::AUGMENTED_SECOND), 3);
        
        assert_eq!(semi(I::MAJOR_THIRD), 4);
        assert_eq!(semi(I::DIMINISHED_FOURTH), 4);
        
        assert_eq!(semi(I::PERFECT_FOURTH), 5);
        assert_eq!(semi(I::AUGMENTED_THIRD), 5);
        
        assert_eq!(semi(I::DIMINISHED_FIFTH), 6);
        assert_eq!(semi(I::AUGMENTED_FOURTH), 6);
        
        assert_eq!(semi(I::PERFECT_FIFTH), 7);
        assert_eq!(semi(I::DIMINISHED_SIXTH), 7);
        
        assert_eq!(semi(I::MINOR_SIXTH), 8);
        assert_eq!(semi(I::AUGMENTED_FIFTH), 8);
        
        assert_eq!(semi(I::MAJOR_SIXTH), 9);
        assert_eq!(semi(I::DIMINISHED_SEVENTH), 9);
        
        assert_eq!(semi(I::MINOR_SEVENTH), 10);
        assert_eq!(semi(I::AUGMENTED_SIXTH), 10);
        
        assert_eq!(semi(I::MAJOR_SEVENTH), 11);
        assert_eq!(semi(I::DIMINISHED_OCTAVE), 11);
        
        assert_eq!(semi(I::PERFECT_OCTAVE), 12);
        assert_eq!(semi(I::AUGMENTED_SEVENTH), 12);
        assert_eq!(semi(I::DIMINISHED_NINTH), 12);
        
        assert_eq!(semi(I::MINOR_NINTH), 13);
        assert_eq!(semi(I::AUGMENTED_OCTAVE), 13);
        
        assert_eq!(semi(I::MAJOR_NINTH), 14);
        assert_eq!(semi(I::DIMINISHED_TENTH), 14);
        
        assert_eq!(semi(I::MINOR_TENTH), 15);
        assert_eq!(semi(I::AUGMENTED_NINTH), 15);
        
        assert_eq!(semi(I::MAJOR_TENTH), 16);
        assert_eq!(semi(I::DIMINISHED_ELEVENTH), 16);
        
        assert_eq!(semi(I::PERFECT_ELEVENTH), 17);
        assert_eq!(semi(I::AUGMENTED_TENTH), 17);
        
        assert_eq!(semi(I::DIMINISHED_TWELFTH), 18);
        assert_eq!(semi(I::AUGMENTED_ELEVENTH), 18);
        
        assert_eq!(semi(I::PERFECT_TWELFTH), 19);
        assert_eq!(semi(I::DIMINISHED_THIRTEENTH), 19);
        
        assert_eq!(semi(I::MINOR_THIRTEENTH), 20);
        assert_eq!(semi(I::AUGMENTED_TWELFTH), 20);
        
        assert_eq!(semi(I::MAJOR_THIRTEENTH), 21);
        assert_eq!(semi(I::DIMINISHED_FOURTEENTH), 21);
        
        assert_eq!(semi(I::MINOR_FOURTEENTH), 22);
        assert_eq!(semi(I::AUGMENTED_THIRTEENTH), 22);
        
        assert_eq!(semi(I::MAJOR_FOURTEENTH), 23);
        assert_eq!(semi(I::DIMINISHED_FIFTEENTH), 23);
        
        assert_eq!(semi(I::PERFECT_FIFTEENTH), 24);
        assert_eq!(semi(I::AUGMENTED_FOURTEENTH), 24);
    }
}