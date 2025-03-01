use std::cmp::Ordering;
use std::fmt;
use std::num::{NonZeroU16, ParseIntError};
use std::ops::Neg;
use std::str::FromStr;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::interval::number::IntervalNumber;
use crate::interval::quality::{IntervalQuality, ParseIntervalQualityErr};
use crate::note::Note;
use crate::pitch::Pitch;
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
         Self::new(quality, number).filter(|ivl| !ivl.is_subzero())
    }

    pub fn between_notes(lhs: Note, rhs: Note) -> Self {
        // make lhs the higher note and rhs the lower note
        let (lhs, rhs) = match lhs.cmp_enharmonic(&rhs) {
            Ordering::Equal => return Self::PERFECT_UNISON,
            Ordering::Less => (lhs, rhs),
            Ordering::Greater => (rhs, lhs),
        };
        
        todo!()
    }
    
    pub fn pitch_semitones_between_helper(lhs: Pitch, rhs: Pitch) -> i16 {
        let base = lhs.semitones_to(rhs).0;

        if lhs.letter().offset_between(rhs.letter()) == 6 && base == 0 {
            base + 12
        } else {
            base
        }
    }
    
    // TODO: fails from c -> B#, since aug 7 is 12 semitones; C -> B# are 0 semitones apart 
    pub fn between_pitches(lhs: Pitch, rhs: Pitch) -> Self {
        let lhs_letter = lhs.letter();
        let rhs_letter = rhs.letter();

        let number = lhs_letter.offset_between(rhs_letter) + 1;
        
        let number = IntervalNumber::new(number as _)
            .expect("can't be zero since offset_between returns [0, 6], and adding one");

        let number_adj = if lhs_letter == rhs_letter && lhs > rhs { IntervalNumber::OCTAVE } else { number };

        let quality = match Self::pitch_semitones_between_helper(lhs, rhs) - number_adj.base_semitones_with_octave_unsigned() {
            -1 if number.is_perfect() => IntervalQuality::DIMINISHED,
            -1 => IntervalQuality::Minor,
            0 if number.is_perfect() => IntervalQuality::Perfect,
            0 => IntervalQuality::Major,
            n @ 1.. => IntervalQuality::Augmented((n as u16).try_into().expect("can't be zero")),
            n @ ..-1 => IntervalQuality::Diminished(NonZeroU16::new(-n as u16 - 1).expect("shouldn't be zero, as the first arm should've caught that")),
        };
        
        Interval::new(quality, number).expect("should be valid")
    }

    pub fn is_subzero(&self) -> bool {
        self.semitones().0.is_negative()
    }

    pub fn quality(&self) -> IntervalQuality {
        self.quality
    }

    pub fn number(&self) -> IntervalNumber {
        self.number
    }

    pub fn semitones(&self) -> Semitone {
        let base_oct_semitones = self.number.base_semitones_with_octave_unsigned();

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

        let unsigned = base_oct_semitones + quality_adjust;

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
        match self.inverted() {
            ivl if !ivl.is_subzero() => Some(ivl),
            _ => None,
        }
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

        let oct = (semi.abs() - 1) / 12;

        let unsigned = base_num + oct * 7;

        let number = IntervalNumber::new(unsigned * semi.signum())
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

    pub fn abs(&self) -> Self {
        self.with_direction(true)
    }

    pub fn add(&self, rhs: Self) -> Self {
        let ln = self.number.number();
        let rn = rhs.number.number();

        let offset = {
            let ls = ln.signum();
            let rs = rn.signum();
            let ss = (ln + rn).signum();

            -ls * rs * ss + (ss == 0) as i16
        };

        let num = IntervalNumber::new(ln + rn + offset)
            .expect("nonzero");

        let distance = self.semitones().0 + rhs.semitones().0;

        let num_sign = num.number().signum();

        let difference = distance - num.base_semitones_with_octave_unsigned() * num_sign;

        let perfect = num.is_perfect();

        use IntervalQuality as IQ;

        let quality = match difference {
            0 if perfect => IQ::Perfect,
            0 if !perfect => IQ::Major,
            -1 if !perfect && num_sign == 1 => IQ::Minor,
            -1 if !perfect && num_sign == -1 => IQ::AUGMENTED,
            diff => match diff * num_sign {
                -1 if !perfect => IQ::Minor,
                n if n > 0 => IQ::Augmented(NonZeroU16::new(n as u16).expect("zero was handled already")),
                n if n < 0 && perfect => IQ::Diminished(NonZeroU16::new(-n as u16).expect("nonzero")),
                n if n < 0 && !perfect => IQ::Diminished(NonZeroU16::new(-(n + 1) as _).expect("nonzero")),
                _ => unreachable!("all cases covered"),
            }
        };

        Self::new(quality, num).expect("valid quality")
    }

    pub fn subtract(&self, rhs: Self) -> Self {
        Self::add(self, -rhs)
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

#[derive(Debug, thiserror::Error, Eq, PartialEq, Clone)]
pub enum ParseIntervalError {
    #[error("The input was in an invalid format")]
    InvalidFormat,
    #[error("The interval wasn't a valid interval")]
    InvalidInterval,
    #[error(transparent)]
    QualityErr(#[from] ParseIntervalQualityErr),
    #[error("Failed to parse number: {0}")]
    NumberErr(#[from] ParseIntError),
}

impl FromStr for Interval {
    type Err = ParseIntervalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars()
            .last()
            .ok_or(ParseIntervalError::InvalidFormat)?
            .is_ascii_digit()
        {
            let leading_negative = s.starts_with('-');

            let start = leading_negative as usize;

            let s = &s[start..];

            let split = s.find(|c: char| c.is_ascii_digit() || c == '-')
                .ok_or(ParseIntervalError::InvalidFormat)?;

            let (quality_str, num_str) = s.split_at(split);

            let quality = quality_str.parse()?;

            let number = num_str.parse()?;

            let ivl = Self::new(quality, number).ok_or(ParseIntervalError::InvalidInterval)?;

            if leading_negative {
                Ok(-ivl)
            } else {
                Ok(ivl)
            }
        } else {
            let split = s.find(|c: char| !c.is_ascii_digit() && c != '-')
                .ok_or(ParseIntervalError::InvalidFormat)?;

            let (num_str, quality_str) = s.split_at(split);

            Ok(
                Self::new(quality_str.parse()?, num_str.parse()?)
                    .ok_or(ParseIntervalError::InvalidInterval)?
            )
        }
    }
}


#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;
    use super::*;
    use Interval as I;
    use IntervalQuality as IQ;
    use IntervalNumber as IN;
    use crate::accidental::AccidentalSign;
    use crate::letter::Letter;

    const FOUR: NonZeroU16 = NonZeroU16::new(4).expect("nonzero");
    const SIX: NonZeroU16 = NonZeroU16::new(6).expect("nonzero");

    // helper fns
    fn semi(ivl: I) -> i16 {
        ivl.semitones().0
    }

    fn ivl(q: IQ, sz: i16) -> I {
        I::new(q, IN::new(sz).expect("nonzero")).expect("valid interval")
    }

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
    fn from_str() {
        assert_eq!("P1".parse(), Ok(I::PERFECT_UNISON));
        assert_eq!("-M7".parse(), Ok(-I::MAJOR_SEVENTH));
        assert_eq!("m-13".parse(), Ok(-I::MINOR_THIRTEENTH));
        assert_eq!("A6".parse(), Ok(I::AUGMENTED_SIXTH));
        assert_eq!("d15".parse(), Ok(I::DIMINISHED_FIFTEENTH));

        assert_eq!("dddd-5".parse(), Ok(I::new(IQ::Diminished(FOUR), -IN::FIFTH).expect("valid interval")));
        assert_eq!("-AAAAAA2".parse(), Ok(I::new(IQ::Augmented(SIX), -IN::SECOND).expect("valid interval")));

        assert_eq!("1P".parse(), Ok(I::PERFECT_UNISON));
        assert_eq!("-7M".parse(), Ok(-I::MAJOR_SEVENTH));
        assert_eq!("-13m".parse(), Ok(-I::MINOR_THIRTEENTH));
        assert_eq!("A6".parse(), Ok(I::AUGMENTED_SIXTH));
        assert_eq!("d15".parse(), Ok(I::DIMINISHED_FIFTEENTH));

        assert_eq!("-5dddd".parse(), Ok(I::new(IQ::Diminished(FOUR), -IN::FIFTH).expect("valid interval")));
        assert_eq!("-2AAAAAA".parse(), Ok(I::new(IQ::Augmented(SIX), -IN::SECOND).expect("valid interval")));

        assert_eq!("".parse::<I>(), Err(ParseIntervalError::InvalidFormat));
        assert_eq!("P3".parse::<I>(), Err(ParseIntervalError::InvalidInterval));
        assert_eq!("q3".parse::<I>(), Err(ParseIntervalError::QualityErr(ParseIntervalQualityErr)));
        assert!(matches!("m0".parse::<I>(), Err(ParseIntervalError::NumberErr(..))));
    }

    #[test]
    fn subzero() {
        assert!(I::strict_non_subzero(IQ::DIMINISHED, IN::UNISON).is_none());
        assert!(I::new(IQ::DIMINISHED, IN::UNISON).expect("valid interval").inverted().inverted_strict_non_subzero().is_none());
        
        for num in 2..15 {
            let num = IN::new(num).expect("nonzero");
            assert!(I::strict_non_subzero(IQ::DIMINISHED, num).is_some());
            assert!(I::strict_non_subzero(IQ::DIMINISHED, num).expect("non subzero").inverted_strict_non_subzero().is_some());
        }

        let doubly_diminished = IQ::Diminished(NonZeroU16::new(2).expect("nonzero"));
        
        assert!(I::strict_non_subzero(doubly_diminished, IN::UNISON).is_none());
        assert!(I::new(doubly_diminished, IN::UNISON).expect("valid interval").inverted().inverted_strict_non_subzero().is_none());
        assert!(I::strict_non_subzero(doubly_diminished, IN::SECOND).is_none());
        assert!(I::new(doubly_diminished, IN::SECOND).expect("valid interval").inverted().inverted_strict_non_subzero().is_none());

        for num in 3..15 {
            let num = IN::new(num).expect("nonzero");
            assert!(I::strict_non_subzero(doubly_diminished, num).is_some());
            assert!(I::strict_non_subzero(doubly_diminished, num).expect("non subzero").inverted_strict_non_subzero().is_some());
        }

        assert!(I::new(IQ::DIMINISHED, IN::UNISON).expect("valid quality").is_subzero());
        assert!(!I::new(IQ::DIMINISHED, IN::SECOND).expect("valid quality").is_subzero());
    }
    
    #[test]
    fn semitones_constants() {
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

    #[test]
    fn semitones_negative() {
        assert_eq!(semi(-I::PERFECT_UNISON), 0);
        assert_eq!(semi(-I::DIMINISHED_SECOND), 0);

        assert_eq!(semi(-I::MINOR_SECOND), -1);
        assert_eq!(semi(-I::AUGMENTED_UNISON), -1);

        assert_eq!(semi(-I::MAJOR_SEVENTH), -11);
        assert_eq!(semi(-I::DIMINISHED_OCTAVE), -11);

        assert_eq!(semi(-I::PERFECT_OCTAVE), -12);
        assert_eq!(semi(-I::AUGMENTED_SEVENTH), -12);
        assert_eq!(semi(-I::DIMINISHED_NINTH), -12);

        assert_eq!(semi(-I::MAJOR_FOURTEENTH), -23);
        assert_eq!(semi(-I::DIMINISHED_FIFTEENTH), -23);

        assert_eq!(semi(-I::PERFECT_FIFTEENTH), -24);
        assert_eq!(semi(-I::AUGMENTED_FOURTEENTH), -24);
    }

    #[test]
    fn semitones_aug_dim() {
        fn dim(adj: u16, sz: i16) -> I {
            I::new(
                IQ::Diminished(NonZeroU16::new(adj).expect("nonzero")),
                IN::new(sz).expect("nonzero")
            ).expect("valid interval")
        }

        fn aug(adj: u16, sz: i16) -> I {
            I::new(
                IQ::Augmented(NonZeroU16::new(adj).expect("nonzero")),
                IN::new(sz).expect("nonzero")
            ).expect("valid interval")
        }

        assert_eq!(semi(dim(7, -80)), -128);
        assert_eq!(semi(dim(6, 4)), -1); // subzero
        assert_eq!(semi(dim(5, -45)), -70);
        assert_eq!(semi(dim(4, 30)), 45);
        assert_eq!(semi(dim(3, -75)), -124);
        assert_eq!(semi(dim(2, 6)), 6);

        assert_eq!(semi(aug(2, -38)), -66);
        assert_eq!(semi(aug(3, -11)), -20);
        assert_eq!(semi(aug(4, 59)), 104);
        assert_eq!(semi(aug(5, 25)), 46);
        assert_eq!(semi(aug(6, -53)), -95);
        assert_eq!(semi(aug(7, 34)), 64);
    }

    #[test]
    fn semitones_general() {
        assert_eq!(semi(ivl(IQ::Perfect, -39)), -65);
        assert_eq!(semi(ivl(IQ::Major, 31)), 52);
        assert_eq!(semi(ivl(IQ::Minor, -76)), -128);
        assert_eq!(semi(ivl(IQ::Perfect, 40)), 67);
        assert_eq!(semi(ivl(IQ::Major, 17)), 28);
        assert_eq!(semi(ivl(IQ::Minor, -77)), -130);
        assert_eq!(semi(ivl(IQ::Perfect, -19)), -31);
        assert_eq!(semi(ivl(IQ::Major, 48)), 81);
        assert_eq!(semi(ivl(IQ::Minor, 21)), 34);
    }

    #[test]
    fn from_semitones() {
        assert_eq!(
            (0..=12)
                .map(|s| Interval::from_semitones_preferred(Semitone(s)))
                .collect::<Vec<_>>(),
            [
                I::PERFECT_UNISON, I::MINOR_SECOND, I::MAJOR_SECOND,
                I::MINOR_THIRD, I::MAJOR_THIRD, I::PERFECT_FOURTH,
                I::DIMINISHED_FIFTH, I::PERFECT_FIFTH, I::MINOR_SIXTH,
                I::MAJOR_SIXTH, I::MINOR_SEVENTH, I::MAJOR_SEVENTH,
                I::PERFECT_OCTAVE
            ]
        );

        assert_eq!(Interval::from_semitones_preferred(Semitone(76)), ivl(IQ::Major, 45));
        assert_eq!(Interval::from_semitones_preferred(Semitone(21)), ivl(IQ::Major, 13));
        assert_eq!(Interval::from_semitones_preferred(Semitone(-31)), ivl(IQ::Perfect, -19));
        assert_eq!(Interval::from_semitones_preferred(Semitone(58)), ivl(IQ::Minor, 35));
        assert_eq!(Interval::from_semitones_preferred(Semitone(14)), ivl(IQ::Major, 9));
        assert_eq!(Interval::from_semitones_preferred(Semitone(-27)), ivl(IQ::Minor, -17));
        assert_eq!(Interval::from_semitones_preferred(Semitone(-17)), ivl(IQ::Perfect, -11));
        assert_eq!(Interval::from_semitones_preferred(Semitone(16)), ivl(IQ::Major, 10));
        assert_eq!(Interval::from_semitones_preferred(Semitone(-66)), ivl(IQ::DIMINISHED, -40));
        assert_eq!(Interval::from_semitones_preferred(Semitone(72)), ivl(IQ::Perfect, 43));
    }

    #[test]
    fn to_from_semitones_inverse() {
        for semis in -75..75 {
            assert_eq!(semi(I::from_semitones_preferred(Semitone(semis))), semis);
        }
    }

    #[test]
    fn shorthand_display() {
        assert_eq!(I::PERFECT_FIFTEENTH.shorthand(), "P15");
        assert_eq!(I::PERFECT_FIFTEENTH.to_string(), "P15");
    }

    #[test]
    fn inverted() {
        assert_eq!(I::PERFECT_UNISON.inverted(), I::PERFECT_UNISON);
        assert_eq!(I::DIMINISHED_SECOND.inverted(), I::AUGMENTED_SEVENTH);
        assert_eq!(-I::MINOR_THIRD.inverted(), -I::MAJOR_SIXTH);
        assert_eq!(I::DIMINISHED_FOURTH.inverted(), I::AUGMENTED_FIFTH);
        assert_eq!(-I::PERFECT_FIFTH.inverted(), -I::PERFECT_FOURTH);
        assert_eq!(I::AUGMENTED_SIXTH.inverted(), I::DIMINISHED_THIRD);
        assert_eq!(I::MAJOR_SEVENTH.inverted(), I::MINOR_SECOND);
        assert_eq!(-I::DIMINISHED_OCTAVE.inverted(), -I::AUGMENTED_OCTAVE);
        assert_eq!(I::PERFECT_OCTAVE.inverted(), I::PERFECT_OCTAVE);
        
        assert_eq!(-I::MINOR_TENTH.inverted(), -I::MAJOR_THIRTEENTH);
        assert_eq!(I::AUGMENTED_ELEVENTH.inverted(), I::DIMINISHED_TWELFTH);
        assert_eq!(I::PERFECT_TWELFTH.inverted(), I::PERFECT_ELEVENTH);
        assert_eq!(-I::DIMINISHED_THIRTEENTH.inverted(), -I::AUGMENTED_TENTH);
        assert_eq!(-I::MAJOR_FOURTEENTH.inverted(), -I::MINOR_NINTH);
        assert_eq!(I::PERFECT_FIFTEENTH.inverted(), I::PERFECT_FIFTEENTH);

        assert_eq!(ivl(IQ::Perfect, -39).inverted(), ivl(IQ::Perfect, -40));
        assert_eq!(ivl(IQ::Major, 31).inverted(), ivl(IQ::Minor, 34));
        assert_eq!(ivl(IQ::Minor, -76).inverted(), ivl(IQ::Major, -73));
        assert_eq!(ivl(IQ::Perfect, 40).inverted(), ivl(IQ::Perfect, 39));
        assert_eq!(ivl(IQ::Major, 17).inverted(), ivl(IQ::Minor, 20));
        assert_eq!(ivl(IQ::Minor, -77).inverted(), ivl(IQ::Major, -72));
        assert_eq!(ivl(IQ::Perfect, -19).inverted(), ivl(IQ::Perfect, -18));
        assert_eq!(ivl(IQ::Major, 48).inverted(), ivl(IQ::Minor, 45));
        assert_eq!(ivl(IQ::Minor, 21).inverted(), ivl(IQ::Major, 16));
    }

    #[test]
    fn double_inversion() {
        for &ivl in I::ALL_CONSTS {
            assert_eq!(ivl.inverted().inverted(), ivl);
        }
    }

    #[test]
    fn direction() {
        assert!(I::MAJOR_NINTH.is_ascending());
        assert!(I::DIMINISHED_TWELFTH.is_ascending());

        assert!(!(-I::MINOR_SEVENTH).is_ascending());
        assert!(!(-I::AUGMENTED_FOURTEENTH).is_ascending());

        assert_eq!(I::MINOR_SEVENTH.with_direction(true), I::MINOR_SEVENTH);
        assert_eq!(I::MAJOR_SECOND.with_direction(false), -I::MAJOR_SECOND);

        assert_eq!((-I::AUGMENTED_FIFTH).with_direction(true), I::AUGMENTED_FIFTH);
        assert_eq!((-I::PERFECT_ELEVENTH).with_direction(false), -I::PERFECT_ELEVENTH);
    }

    #[test]
    fn eq_ord_enharmonic() {
        assert!(I::MAJOR_SIXTH.eq_enharmonic(&I::DIMINISHED_SEVENTH));
        assert!(I::AUGMENTED_THIRTEENTH.eq_enharmonic(&I::MINOR_FOURTEENTH));

        assert!(!I::MINOR_THIRD.eq_enharmonic(&I::DIMINISHED_FOURTH));
        assert!(!I::PERFECT_TWELFTH.eq_enharmonic(&I::AUGMENTED_TWELFTH));

        assert_eq!(I::AUGMENTED_FOURTH.cmp_enharmonic(&I::PERFECT_FIFTH), Ordering::Less);
        assert_eq!(I::MAJOR_NINTH.cmp_enharmonic(&I::DIMINISHED_TENTH), Ordering::Equal);
        assert_eq!(I::PERFECT_FIFTEENTH.cmp_enharmonic(&I::DIMINISHED_FIFTEENTH), Ordering::Greater);
    }

    #[test]
    fn add_subtract() {
        use IntervalQuality as IQ;
        use IntervalNumber as IN;

        let mut qualities = vec![IQ::Perfect, IQ::Major, IQ::Minor];
        qualities.extend((1..=4).map(|n| IQ::Diminished(NonZeroU16::new(n).expect("nonzero"))));
        qualities.extend((1..=4).map(|n| IQ::Augmented(NonZeroU16::new(n).expect("nonzero"))));

        let mut numbers = Vec::with_capacity(100);
        numbers.extend((1..=24).map(|n| IN::new(n).expect("nonzero")));
        numbers.extend((-24..=-1).map(|n| IN::new(n).expect("nonzero")));

        let intervals = qualities.iter()
            .flat_map(|iq|
                numbers.iter().filter_map(
                    |num| I::new(*iq, *num)
                ))
            .collect::<Vec<_>>();

        for lhs in &intervals {
            for rhs in &intervals {
                let add = lhs.add(*rhs);
                let sub = lhs.subtract(*rhs);

                assert_eq!(Note::MIDDLE_C.transpose(lhs).transpose(rhs), Note::MIDDLE_C.transpose(&add), "lhs: {lhs}, rhs: {rhs} add: {add}");
                assert_eq!(Note::MIDDLE_C.transpose(lhs).transpose(&-(*rhs)), Note::MIDDLE_C.transpose(&sub), "lhs: {lhs}, rhs: {rhs} add: {sub}");
            }
        }
    }

    #[test]
    fn neg() {
        assert_eq!((-I::DIMINISHED_FOURTEENTH).shorthand(), "d-14");
        assert_eq!(-(-I::MAJOR_SEVENTH), I::MAJOR_SEVENTH);
    }
    
    #[test]
    fn test_aug_seventh() {
        let between = Interval::between_pitches(Pitch::C, Pitch::B_SHARP);

        assert_eq!(Pitch::C.transpose(&between), Pitch::B_SHARP, "{between}");
        
        let between = Interval::between_pitches(Pitch::G, Pitch::F_DOUBLE_SHARP);
        
        assert_eq!(Pitch::G.transpose(&between), Pitch::F_DOUBLE_SHARP, "{between}");

        let between = Interval::between_pitches(Pitch::G_DOUBLE_FLAT, Pitch::F);

        assert_eq!(Pitch::G_DOUBLE_FLAT.transpose(&between), Pitch::F, "{between}");
        
        let g_quadruple_flat = Pitch::from_letter_and_accidental(Letter::G, AccidentalSign { offset: -4 });

        let between = Interval::between_pitches(g_quadruple_flat, Pitch::F_DOUBLE_FLAT);

        assert_eq!(Pitch::G_DOUBLE_FLAT.transpose(&between), Pitch::F, "{between}");
    }

    #[test]
    fn between_pitches_transpose_inverses() {
        for ivl in &Interval::ALL_CONSTS[..23] {
            for start in Pitch::ALL_CONSTS {
                let end = start.transpose(ivl);
                
                assert_eq!(
                    start.semitones_to(end), ivl.semitones(),
                    "{start} -> {end} should span {} semitones", ivl.semitones().0
                );

                let between = Interval::between_pitches(*start, end);
                
                assert_eq!(
                    between, *ivl,
                    "between_pitches returns {between} instead of applied {ivl}, ({start} -> {end})"
                );
                
                let neg_between = Interval::between_pitches(end, *start);

                let inv = ivl.inverted();

                assert_eq!(
                    neg_between, inv,
                    "neg_between_pitches returns {neg_between} instead of applied {inv}, ({end} -> {start})"
                );
            }
        }
    }
}