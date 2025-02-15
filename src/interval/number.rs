use std::fmt;
use std::num::{NonZeroI16, ParseIntError};
use std::ops::Neg;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IntervalNumber(NonZeroI16);

impl IntervalNumber {
    pub const fn new(number: i16) -> Option<Self> {
        // TODO: Option::map and ? operator both aren't const yet
        match NonZeroI16::new(number) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }

    pub fn number(self) -> i16 {
        self.0.get()
    }

    pub fn shorthand(self) -> i16 {
        self.number()
    }

    pub fn as_simple(self) -> Self {
        if self.number().abs() != 1 && (self.number().abs() - 1) % 7 == 0 {
            match self.number().is_positive() {
                true => Self::OCTAVE,
                false => -Self::OCTAVE,
            }
        } else {
            let num = (self.number().abs() - 1) % 7 + 1;
            
            Self::new(num * self.number().signum())
                .expect("can't be zero")
        }
    }

    pub fn is_perfect(self) -> bool {
        match self.as_simple().number().abs() {
            1 | 4 | 5 | 8 => true,
            2 | 3 | 6 | 7 => false,
            _ => unreachable!("abs of as_simple must be within [1,8]")
        }
    }

    pub fn is_ascending(self) -> bool {
        self.number().is_positive()
    }

    pub fn with_direction(self, ascending: bool) -> Self {
        if self.is_ascending() == ascending {
            self
        } else {
            -self
        }
    }

    pub fn octave_unsigned(self) -> i16 { // TODO: make this return u16
        (self.number().abs() - 1) / 7
    }

    pub fn octave_signed(self) -> i16 {
        self.octave_unsigned() * self.number().signum()
    }

    pub fn inverted(self) -> Self {
        let simple_abs = self.as_simple().number().abs();

        let n = match simple_abs {
            1 | 8 => simple_abs,
            2..=7 => 9 - simple_abs,
            _ => unreachable!("abs of as_simple must be within [1,8]")
        };

        let num = match simple_abs {
            1..=7 => 7 * self.octave_unsigned() + n,
            8 => 7 * (self.octave_unsigned() - 1) + n,
            _ => unreachable!("abs of as_simple must be within [1,8]")
        };

        Self::new(num * self.number().signum())
            .expect("can't be zero")
    }

    pub(super) fn base_semitones_with_octave_unsigned(self) -> i16 {
        let without_octave = match self.as_simple().number().abs() {
            1 | 8 => 0,
            2 => 2,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 9,
            7 => 11,
            _ => unreachable!("abs of as_simple must be within [1,8]"),
        };

        without_octave + self.octave_unsigned() * 12
    }
}

impl Neg for IntervalNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl fmt::Display for IntervalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shorthand())
    }
}

impl FromStr for IntervalNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NonZeroI16::from_str(s).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use IntervalNumber as IN;

    #[test]
    fn nonzero_number() {
        assert!(IN::new(5).is_some());
        assert!(IN::new(-8).is_some());
        assert!(IN::new(0).is_none());
    }
    
    #[test]
    fn number_shorthand() {
        assert_eq!(IN::FIFTH.number(), 5);
        assert_eq!(IN::FIFTH.shorthand(), 5);
    }
    
    #[test]
    fn simple_as_simple() {
        assert_eq!(IN::SIXTH.as_simple(), IN::SIXTH);
        
        assert_eq!((-IN::SEVENTH).as_simple(), -IN::SEVENTH);
    }
    
    #[test]
    fn octave_as_simple() {
        assert_eq!(IN::OCTAVE.as_simple(), IN::OCTAVE);
        
        assert_eq!((-IN::OCTAVE).as_simple(), -IN::OCTAVE);
        
        assert_eq!(IN::FIFTEENTH.as_simple(), IN::OCTAVE);

        assert_eq!((-IN::FIFTEENTH).as_simple(), -IN::OCTAVE);
        
        let fifty_seventh = IN::new(57).expect("nonzero");
        
        assert_eq!(fifty_seventh.as_simple(), IN::OCTAVE);
        assert_eq!((-fifty_seventh).as_simple(), -IN::OCTAVE);
    }
    
    #[test]
    fn negative_as_simple() {
        assert_eq!(-IN::THIRTEENTH.as_simple(), -IN::SIXTH);
        assert_eq!((-IN::THIRTEENTH).as_simple(), -IN::SIXTH);
    }
    
    #[test]
    fn general_as_simple() {
        let as_simple = |x: IN| x.as_simple();
        
        assert_eq!(
            [IN::NINTH, IN::TENTH, IN::ELEVENTH, IN::TWELFTH, IN::THIRTEENTH, IN::FOURTEENTH, IN::FIFTEENTH].map(as_simple),
            [IN::SECOND, IN::THIRD, IN::FOURTH, IN::FIFTH, IN::SIXTH, IN::SEVENTH, IN::OCTAVE]
        );

        assert_eq!(
            [IN::NINTH, IN::TENTH, IN::ELEVENTH, IN::TWELFTH, IN::THIRTEENTH, IN::FOURTEENTH, IN::FIFTEENTH].map(Neg::neg).map(as_simple),
            [IN::SECOND, IN::THIRD, IN::FOURTH, IN::FIFTH, IN::SIXTH, IN::SEVENTH, IN::OCTAVE].map(Neg::neg)
        );
        
        for oct in 0..=15 {
            for base in 2..=8 {
                assert_eq!(IN::new(7 * oct + base).expect("nonzero").as_simple(), IN::new(base).expect("nonzero"), "{oct}, {base}, {}", 7 * oct + base);
                assert_eq!((-IN::new(7 * oct + base).expect("nonzero")).as_simple(), IN::new(-base).expect("nonzero"), "{oct}, {base}, {}", 7 * oct + base);
            }
        }
    }
    
    #[test]
    fn is_perfect() {
        assert!(IN::UNISON.is_perfect());
        assert!(!IN::SECOND.is_perfect());
        assert!(!IN::THIRD.is_perfect());
        assert!(IN::FOURTH.is_perfect());
        assert!(IN::FIFTH.is_perfect());
        assert!(!IN::SIXTH.is_perfect());
        assert!(!IN::SEVENTH.is_perfect());
        assert!(IN::OCTAVE.is_perfect());
        
        assert!((-IN::FOURTH).is_perfect());
        assert!(!(-IN::SEVENTH).is_perfect());
        
        assert!(IN::new(7 * 17 + 4).expect("nonzero").is_perfect());
        assert!(!IN::new(7 * 17 + 3).expect("nonzero").is_perfect());
        assert!(IN::new(7 * 13 + 5).expect("nonzero").neg().is_perfect());
        assert!(!IN::new(7 * 13 + 7).expect("nonzero").neg().is_perfect());
    }
    
    #[test]
    fn is_ascending() {
        assert!(IN::TWELFTH.is_ascending());
        assert!(!(-IN::NINTH).is_ascending());
    }
    
    #[test]
    fn with_direction() {
        assert_eq!(IN::THIRTEENTH.with_direction(true), IN::THIRTEENTH);
        assert_eq!(IN::SEVENTH.with_direction(false), -IN::SEVENTH);
        
        assert_eq!((-IN::SECOND).with_direction(false), -IN::SECOND);
        assert_eq!((-IN::OCTAVE).with_direction(true), IN::OCTAVE);
    }
    
    #[test]
    fn octave() {
        assert_eq!(IN::UNISON.octave_unsigned(), 0);
        assert_eq!(IN::SECOND.octave_unsigned(), 0);
        assert_eq!(IN::SEVENTH.octave_unsigned(), 0);
        assert_eq!(IN::OCTAVE.octave_unsigned(), 1);
        assert_eq!(IN::NINTH.octave_unsigned(), 1);
        assert_eq!(IN::FOURTEENTH.octave_unsigned(), 1);
        assert_eq!(IN::FIFTEENTH.octave_unsigned(), 2);
        
        assert_eq!(IN::FOURTEENTH.octave_signed(), 1);
        
        assert_eq!((-IN::OCTAVE).octave_unsigned(), 1);
        assert_eq!((-IN::FIFTEENTH).octave_signed(), -2);
    }
    
    #[test]
    fn inverted() {
        assert_eq!(IN::UNISON.inverted(), IN::UNISON);
        assert_eq!(IN::SECOND.inverted(), IN::SEVENTH);
        assert_eq!(IN::THIRD.inverted(), IN::SIXTH);
        assert_eq!(IN::FOURTH.inverted(), IN::FIFTH);
        assert_eq!(IN::FIFTH.inverted(), IN::FOURTH);
        assert_eq!(IN::SIXTH.inverted(), IN::THIRD);
        assert_eq!(IN::SEVENTH.inverted(), IN::SECOND);
        assert_eq!(IN::OCTAVE.inverted(), IN::OCTAVE);
        
        assert_eq!(IN::FOURTEENTH.inverted(), IN::NINTH);
        assert_eq!((-IN::TWELFTH).inverted(), -IN::ELEVENTH);
        
        assert_eq!((-IN::OCTAVE).inverted(), -IN::OCTAVE);
        assert_eq!(IN::FIFTEENTH.inverted(), IN::FIFTEENTH);
        assert_eq!((-IN::FIFTEENTH).inverted(), -IN::FIFTEENTH);

        assert_eq!(IN::THIRD.inverted().inverted(), IN::THIRD);
        assert_eq!(IN::FOURTH.inverted().inverted(), IN::FOURTH);
        assert_eq!(IN::NINTH.inverted().inverted(), IN::NINTH);
        assert_eq!(IN::FOURTEENTH.inverted().inverted(), IN::FOURTEENTH);
    }
    
    #[test]
    fn neg() {
        assert_eq!((-IN::FIFTEENTH).number(), -15);
        assert_eq!(-(-IN::NINTH), IN::NINTH);
    }
    
    #[test]
    fn display() {
        assert_eq!(format!("{}", -IN::ELEVENTH), "-11");
    }
}