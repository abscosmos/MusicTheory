use std::fmt;
use std::num::NonZeroI16;
use std::ops::Neg;

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