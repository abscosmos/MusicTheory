use std::fmt;
use std::num::{NonZeroI16, ParseIntError};
use std::ops::Neg;
use std::str::FromStr;

/// The diatonic size of an interval, such as "third" or "fifth".
///
/// Interval numbers are positive for ascending intervals, and negative for descending interval.
/// The smallest number is a [unison](Self::UNISON), since unisons are the additive inverse for intervals.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// // Create using constants
/// let fifth = IntervalNumber::FIFTH;
/// assert_eq!(fifth.get(), 5);
///
/// // ... or dynamically
/// let ninth = IntervalNumber::new(9).unwrap();
/// assert_eq!(ninth.as_simple(), IntervalNumber::SECOND);
///
/// // Negative for descending intervals
/// let desc_fourth = IntervalNumber::new(-4).unwrap();
/// assert_eq!(desc_fourth, -IntervalNumber::FOURTH);
/// assert!(!desc_fourth.is_ascending());
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntervalNumber(pub NonZeroI16);

impl IntervalNumber {
    /// Creates a new `IntervalNumber`.
    ///
    /// Returns `None` if the number is zero, as interval numbers cannot be zero.
    /// Negative numbers represent descending intervals.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(IntervalNumber::new(3), Some(IntervalNumber::THIRD));
    /// assert_eq!(IntervalNumber::new(-5), Some(-IntervalNumber::FIFTH));
    /// // Zero is invalid
    /// assert_eq!(IntervalNumber::new(0), None);
    /// ```
    pub const fn new(number: i16) -> Option<Self> {
        // TODO: Option::map and ? operator both aren't const yet
        match NonZeroI16::new(number) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }

    /// Returns the inner value of the interval number.
    ///
    /// Positive values indicate ascending intervals, negative values indicate descending intervals.
    /// Since this value is never zero, you can get it as a [`NonZeroI16`] using `self.0` instead.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(IntervalNumber::FIFTH.get(), 5);
    /// assert_eq!((-IntervalNumber::THIRD).get(), -3);
    /// ```
    pub fn get(self) -> i16 {
        self.0.get()
    }

    /// Returns the shorthand notation for the interval number.
    ///
    /// This is identical to [`Self::get`] and returns the numeric value.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(IntervalNumber::SEVENTH.shorthand(), 7);
    /// ```
    pub fn shorthand(self) -> i16 {
        self.get()
    }

    /// Reduces a compound interval to its simple form.
    ///
    /// Simplified intervals in `[1, 8]`, and compound intervals (9ths, 10ths, etc.)
    /// are reduced by removing complete octaves. The direction is preserved.
    /// Octaves and multiples of octaves reduce to an octave, *not a unison!*
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Simple intervals remain unchanged
    /// assert_eq!(IntervalNumber::THIRD.as_simple(), IntervalNumber::THIRD);
    ///
    /// // Compound intervals reduce to their simple form
    /// assert_eq!(IntervalNumber::NINTH.as_simple(), IntervalNumber::SECOND);
    /// assert_eq!(IntervalNumber::THIRTEENTH.as_simple(), IntervalNumber::SIXTH);
    ///
    /// // Octaves and multiples of octaves remain as octaves
    /// assert_eq!(IntervalNumber::FIFTEENTH.as_simple(), IntervalNumber::OCTAVE);
    ///
    /// // Direction is preserved
    /// assert_eq!((-IntervalNumber::NINTH).as_simple(), -IntervalNumber::SECOND);
    /// ```
    pub fn as_simple(self) -> Self {
        if self.get().abs() != 1 && (self.get().abs() - 1) % 7 == 0 {
            match self.get().is_positive() {
                true => Self::OCTAVE,
                false => -Self::OCTAVE,
            }
        } else {
            let num = (self.get().abs() - 1) % 7 + 1;
            
            Self::new(num * self.get().signum())
                .expect("can't be zero")
        }
    }

    /// Returns `true` if `self` is a perfect interval number (1, 4, 5, or 8).
    ///
    /// Perfect interval numbers can have perfect, augmented, or diminished qualities,
    /// but not major or minor. The other interval numbers (2, 3, 6, 7) are major/minor
    /// and cannot be perfect.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert!(IntervalNumber::FIFTH.is_perfect());
    /// assert!(IntervalNumber::OCTAVE.is_perfect());
    ///
    /// assert!(!IntervalNumber::THIRD.is_perfect());
    /// assert!(!IntervalNumber::SIXTH.is_perfect());
    ///
    /// // Works with compound intervals based on their simple form
    /// assert!(IntervalNumber::ELEVENTH.is_perfect()); // 11 -> 4
    /// assert!(!IntervalNumber::NINTH.is_perfect());   // 9 -> 2
    /// ```
    pub fn is_perfect(self) -> bool {
        match self.as_simple().get().abs() {
            1 | 4 | 5 | 8 => true,
            2 | 3 | 6 | 7 => false,
            _ => unreachable!("abs of as_simple must be within [1,8]")
        }
    }

    /// Returns `true` if this is an ascending interval (positive).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert!(IntervalNumber::FIFTH.is_ascending());
    /// assert!(!(-IntervalNumber::THIRD).is_ascending());
    /// ```
    pub fn is_ascending(self) -> bool {
        self.get().is_positive()
    }

    /// Returns the interval number with the specified direction.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let fifth = IntervalNumber::FIFTH;
    ///
    /// // Already ascending
    /// assert_eq!(fifth.with_direction(true), fifth);
    ///
    /// // Make descending
    /// assert_eq!(fifth.with_direction(false), -fifth);
    ///
    /// // Descending to ascending
    /// let desc = -IntervalNumber::THIRD;
    /// assert_eq!(desc.with_direction(true), IntervalNumber::THIRD);
    /// ```
    pub fn with_direction(self, ascending: bool) -> Self {
        if self.is_ascending() == ascending {
            self
        } else {
            -self
        }
    }

    /// Returns the number of complete octaves in this interval.
    ///
    /// The direction of the interval is ignored. For signed, use [`Self::octave_signed`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(IntervalNumber::THIRD.octave_unsigned(), 0);
    /// assert_eq!(IntervalNumber::SEVENTH.octave_unsigned(), 0);
    ///
    /// assert_eq!(IntervalNumber::OCTAVE.octave_unsigned(), 1);
    /// assert_eq!(IntervalNumber::TENTH.octave_unsigned(), 1);
    ///
    /// // Sign is ignored
    /// assert_eq!((-IntervalNumber::THIRTEENTH).octave_unsigned(), 1);
    /// ```
    pub fn octave_unsigned(self) -> i16 { // TODO: make this return u16
        (self.get().abs() - 1) / 7
    }

    /// Returns the number of complete octaves in this interval (signed).
    ///
    /// If you don't need the sign, consider [`Self::octave_unsigned`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(IntervalNumber::NINTH.octave_signed(), 1);
    /// assert_eq!(IntervalNumber::FIFTEENTH.octave_signed(), 2);
    ///
    /// // Negative for descending intervals
    /// assert_eq!((-IntervalNumber::NINTH).octave_signed(), -1);
    /// assert_eq!((-IntervalNumber::FIFTEENTH).octave_signed(), -2);
    /// ```
    pub fn octave_signed(self) -> i16 {
        self.octave_unsigned() * self.get().signum()
    }

    /// Returns the inverted interval number.
    ///
    /// Inversion flips an interval around: seconds become sevenths, thirds become sixths, etc.
    /// This takes into account compound intervals, and inverts them within the octave they're in.
    /// Unisons and octaves invert to themselves. Direction is preserved.
    ///
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Simple interval inversions
    /// assert_eq!(IntervalNumber::SECOND.inverted(), IntervalNumber::SEVENTH);
    /// assert_eq!(IntervalNumber::THIRD.inverted(), IntervalNumber::SIXTH);
    ///
    /// // Unison and octave invert to themselves
    /// assert_eq!(IntervalNumber::UNISON.inverted(), IntervalNumber::UNISON);
    /// assert_eq!(IntervalNumber::OCTAVE.inverted(), IntervalNumber::OCTAVE);
    ///
    /// // Compound intervals account for octave displacement
    /// assert_eq!(IntervalNumber::NINTH.inverted(), IntervalNumber::FOURTEENTH);
    ///
    /// // Direction is preserved
    /// assert_eq!((-IntervalNumber::THIRD).inverted(), -IntervalNumber::SIXTH);
    /// ```
    pub fn inverted(self) -> Self {
        let simple_abs = self.as_simple().get().abs();

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

        Self::new(num * self.get().signum())
            .expect("can't be zero")
    }

    /// Returns the amount of semitones a major/perfect interval of this number would have.
    pub(super) fn base_semitones_with_octave_unsigned(self) -> i16 {
        let without_octave = match self.as_simple().get().abs() {
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
        assert_eq!(IN::FIFTH.get(), 5);
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
        assert_eq!((-IN::FIFTEENTH).get(), -15);
        assert_eq!(-(-IN::NINTH), IN::NINTH);
    }
    
    #[test]
    fn display() {
        assert_eq!(format!("{}", -IN::ELEVENTH), "-11");
    }
}