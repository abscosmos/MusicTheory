use std::fmt;
use std::num::{NonZeroI16, ParseIntError};
use std::ops::Neg;
use std::str::FromStr;

/// The diatonic size of an interval, such as "third" or "fifth".
///
/// Interval numbers are positive for ascending intervals, and negative for descending intervals.
/// The smallest number is a [unison](Self::UNISON), since unisons are the additive inverse for intervals.
///
/// For convenience, constants like [`Self::THIRD`] are available.
///
/// If importing this type conflicts with other types, consider aliasing it:
/// ```
/// use music_theory::interval::Number as IntervalNumber;
/// # let _ = IntervalNumber::THIRD;
/// ```
///
/// # Examples
///
/// ```
/// # use music_theory::interval::Number;
/// // Create using constants
/// let fifth = Number::FIFTH;
/// assert_eq!(fifth.get(), 5);
///
/// // ... or dynamically
/// let ninth = Number::new(9).unwrap();
/// assert_eq!(ninth.as_simple(), Number::SECOND);
///
/// // Negative for descending intervals
/// let desc_fourth = Number::new(-4).unwrap();
/// assert_eq!(desc_fourth, -Number::FOURTH);
/// assert!(!desc_fourth.is_ascending());
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Number(pub NonZeroI16);

impl Number {
    /// Creates a new `IntervalNumber`.
    ///
    /// Returns `None` if the number is zero, as interval numbers cannot be zero.
    /// Negative numbers represent descending intervals.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::interval::Number;
    /// assert_eq!(Number::new(3), Some(Number::THIRD));
    /// assert_eq!(Number::new(-5), Some(-Number::FIFTH));
    /// // Zero is invalid
    /// assert_eq!(Number::new(0), None);
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
    /// # use music_theory::interval::Number;
    /// assert_eq!(Number::FIFTH.get(), 5);
    /// assert_eq!((-Number::THIRD).get(), -3);
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
    /// # use music_theory::interval::Number;
    /// assert_eq!(Number::SEVENTH.shorthand(), 7);
    /// ```
    pub fn shorthand(self) -> i16 {
        self.get()
    }

    /// Reduces a compound interval to its simple form.
    ///
    /// Simplified intervals are in `[1, 8]`, so compound intervals (9ths, 10ths, etc.)
    /// are reduced by removing complete octaves. The direction is preserved.
    /// Octaves and multiples of octaves reduce to an octave, *not a unison!*
    ///
    /// # Examples
    /// ```
    /// # use music_theory::interval::Number;
    /// // Simple intervals remain unchanged
    /// assert_eq!(Number::THIRD.as_simple(), Number::THIRD);
    ///
    /// // Compound intervals reduce to their simple form
    /// assert_eq!(Number::NINTH.as_simple(), Number::SECOND);
    /// assert_eq!(Number::THIRTEENTH.as_simple(), Number::SIXTH);
    ///
    /// // Octaves and multiples of octaves remain as octaves
    /// assert_eq!(Number::FIFTEENTH.as_simple(), Number::OCTAVE);
    ///
    /// // Direction is preserved
    /// assert_eq!((-Number::NINTH).as_simple(), -Number::SECOND);
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
    /// # use music_theory::interval::Number;
    /// assert!(Number::FIFTH.is_perfect());
    /// assert!(Number::OCTAVE.is_perfect());
    ///
    /// assert!(!Number::THIRD.is_perfect());
    /// assert!(!Number::SIXTH.is_perfect());
    ///
    /// // Works with compound intervals based on their simple form
    /// assert!(Number::ELEVENTH.is_perfect()); // 11 -> 4
    /// assert!(!Number::NINTH.is_perfect());   // 9 -> 2
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
    /// # use music_theory::interval::Number;
    /// assert!(Number::FIFTH.is_ascending());
    /// assert!(!(-Number::THIRD).is_ascending());
    /// ```
    pub fn is_ascending(self) -> bool {
        self.get().is_positive()
    }

    /// Returns the interval number with the specified direction.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::interval::Number;
    /// let fifth = Number::FIFTH;
    ///
    /// // Already ascending
    /// assert_eq!(fifth.with_direction(true), fifth);
    ///
    /// // Make descending
    /// assert_eq!(fifth.with_direction(false), -fifth);
    ///
    /// // Descending to ascending
    /// let desc = -Number::THIRD;
    /// assert_eq!(desc.with_direction(true), Number::THIRD);
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
    /// # use music_theory::interval::Number;
    /// assert_eq!(Number::THIRD.octave_unsigned(), 0);
    /// assert_eq!(Number::SEVENTH.octave_unsigned(), 0);
    ///
    /// assert_eq!(Number::OCTAVE.octave_unsigned(), 1);
    /// assert_eq!(Number::TENTH.octave_unsigned(), 1);
    ///
    /// // Sign is ignored
    /// assert_eq!((-Number::THIRTEENTH).octave_unsigned(), 1);
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
    /// # use music_theory::interval::Number;
    /// assert_eq!(Number::NINTH.octave_signed(), 1);
    /// assert_eq!(Number::FIFTEENTH.octave_signed(), 2);
    ///
    /// // Negative for descending intervals
    /// assert_eq!((-Number::NINTH).octave_signed(), -1);
    /// assert_eq!((-Number::FIFTEENTH).octave_signed(), -2);
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
    /// # use music_theory::interval::Number;
    /// // Simple interval inversions
    /// assert_eq!(Number::SECOND.inverted(), Number::SEVENTH);
    /// assert_eq!(Number::THIRD.inverted(), Number::SIXTH);
    ///
    /// // Unison and octave invert to themselves
    /// assert_eq!(Number::UNISON.inverted(), Number::UNISON);
    /// assert_eq!(Number::OCTAVE.inverted(), Number::OCTAVE);
    ///
    /// // Compound intervals account for octave displacement
    /// assert_eq!(Number::NINTH.inverted(), Number::FOURTEENTH);
    ///
    /// // Direction is preserved
    /// assert_eq!((-Number::THIRD).inverted(), -Number::SIXTH);
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

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shorthand())
    }
}

impl FromStr for Number {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NonZeroI16::from_str(s).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonzero_number() {
        assert!(Number::new(5).is_some());
        assert!(Number::new(-8).is_some());
        assert!(Number::new(0).is_none());
    }
    
    #[test]
    fn number_shorthand() {
        assert_eq!(Number::FIFTH.get(), 5);
        assert_eq!(Number::FIFTH.shorthand(), 5);
    }
    
    #[test]
    fn simple_as_simple() {
        assert_eq!(Number::SIXTH.as_simple(), Number::SIXTH);
        
        assert_eq!((-Number::SEVENTH).as_simple(), -Number::SEVENTH);
    }
    
    #[test]
    fn octave_as_simple() {
        assert_eq!(Number::OCTAVE.as_simple(), Number::OCTAVE);
        
        assert_eq!((-Number::OCTAVE).as_simple(), -Number::OCTAVE);
        
        assert_eq!(Number::FIFTEENTH.as_simple(), Number::OCTAVE);

        assert_eq!((-Number::FIFTEENTH).as_simple(), -Number::OCTAVE);
        
        let fifty_seventh = Number::new(57).expect("nonzero");
        
        assert_eq!(fifty_seventh.as_simple(), Number::OCTAVE);
        assert_eq!((-fifty_seventh).as_simple(), -Number::OCTAVE);
    }
    
    #[test]
    fn negative_as_simple() {
        assert_eq!(-Number::THIRTEENTH.as_simple(), -Number::SIXTH);
        assert_eq!((-Number::THIRTEENTH).as_simple(), -Number::SIXTH);
    }
    
    #[test]
    fn general_as_simple() {
        use Number as N;
        
        assert_eq!(
            [N::NINTH, N::TENTH, N::ELEVENTH, N::TWELFTH, N::THIRTEENTH, N::FOURTEENTH, N::FIFTEENTH].map(N::as_simple),
            [N::SECOND, N::THIRD, N::FOURTH, N::FIFTH, N::SIXTH, N::SEVENTH, N::OCTAVE]
        );

        assert_eq!(
            [N::NINTH, N::TENTH, N::ELEVENTH, N::TWELFTH, N::THIRTEENTH, N::FOURTEENTH, N::FIFTEENTH].map(Neg::neg).map(N::as_simple),
            [N::SECOND, N::THIRD, N::FOURTH, N::FIFTH, N::SIXTH, N::SEVENTH, N::OCTAVE].map(Neg::neg)
        );
        
        for oct in 0..=15 {
            for base in 2..=8 {
                assert_eq!(N::new(7 * oct + base).expect("nonzero").as_simple(), N::new(base).expect("nonzero"), "{oct}, {base}, {}", 7 * oct + base);
                assert_eq!((-N::new(7 * oct + base).expect("nonzero")).as_simple(), N::new(-base).expect("nonzero"), "{oct}, {base}, {}", 7 * oct + base);
            }
        }
    }
    
    #[test]
    fn is_perfect() {
        assert!(Number::UNISON.is_perfect());
        assert!(!Number::SECOND.is_perfect());
        assert!(!Number::THIRD.is_perfect());
        assert!(Number::FOURTH.is_perfect());
        assert!(Number::FIFTH.is_perfect());
        assert!(!Number::SIXTH.is_perfect());
        assert!(!Number::SEVENTH.is_perfect());
        assert!(Number::OCTAVE.is_perfect());
        
        assert!((-Number::FOURTH).is_perfect());
        assert!(!(-Number::SEVENTH).is_perfect());
        
        assert!(Number::new(7 * 17 + 4).expect("nonzero").is_perfect());
        assert!(!Number::new(7 * 17 + 3).expect("nonzero").is_perfect());
        assert!(Number::new(7 * 13 + 5).expect("nonzero").neg().is_perfect());
        assert!(!Number::new(7 * 13 + 7).expect("nonzero").neg().is_perfect());
    }
    
    #[test]
    fn is_ascending() {
        assert!(Number::TWELFTH.is_ascending());
        assert!(!(-Number::NINTH).is_ascending());
    }
    
    #[test]
    fn with_direction() {
        assert_eq!(Number::THIRTEENTH.with_direction(true), Number::THIRTEENTH);
        assert_eq!(Number::SEVENTH.with_direction(false), -Number::SEVENTH);
        
        assert_eq!((-Number::SECOND).with_direction(false), -Number::SECOND);
        assert_eq!((-Number::OCTAVE).with_direction(true), Number::OCTAVE);
    }
    
    #[test]
    fn octave() {
        assert_eq!(Number::UNISON.octave_unsigned(), 0);
        assert_eq!(Number::SECOND.octave_unsigned(), 0);
        assert_eq!(Number::SEVENTH.octave_unsigned(), 0);
        assert_eq!(Number::OCTAVE.octave_unsigned(), 1);
        assert_eq!(Number::NINTH.octave_unsigned(), 1);
        assert_eq!(Number::FOURTEENTH.octave_unsigned(), 1);
        assert_eq!(Number::FIFTEENTH.octave_unsigned(), 2);
        
        assert_eq!(Number::FOURTEENTH.octave_signed(), 1);
        
        assert_eq!((-Number::OCTAVE).octave_unsigned(), 1);
        assert_eq!((-Number::FIFTEENTH).octave_signed(), -2);
    }
    
    #[test]
    fn inverted() {
        assert_eq!(Number::UNISON.inverted(), Number::UNISON);
        assert_eq!(Number::SECOND.inverted(), Number::SEVENTH);
        assert_eq!(Number::THIRD.inverted(), Number::SIXTH);
        assert_eq!(Number::FOURTH.inverted(), Number::FIFTH);
        assert_eq!(Number::FIFTH.inverted(), Number::FOURTH);
        assert_eq!(Number::SIXTH.inverted(), Number::THIRD);
        assert_eq!(Number::SEVENTH.inverted(), Number::SECOND);
        assert_eq!(Number::OCTAVE.inverted(), Number::OCTAVE);
        
        assert_eq!(Number::FOURTEENTH.inverted(), Number::NINTH);
        assert_eq!((-Number::TWELFTH).inverted(), -Number::ELEVENTH);
        
        assert_eq!((-Number::OCTAVE).inverted(), -Number::OCTAVE);
        assert_eq!(Number::FIFTEENTH.inverted(), Number::FIFTEENTH);
        assert_eq!((-Number::FIFTEENTH).inverted(), -Number::FIFTEENTH);

        assert_eq!(Number::THIRD.inverted().inverted(), Number::THIRD);
        assert_eq!(Number::FOURTH.inverted().inverted(), Number::FOURTH);
        assert_eq!(Number::NINTH.inverted().inverted(), Number::NINTH);
        assert_eq!(Number::FOURTEENTH.inverted().inverted(), Number::FOURTEENTH);
    }
    
    #[test]
    fn neg() {
        assert_eq!((-Number::FIFTEENTH).get(), -15);
        assert_eq!(-(-Number::NINTH), Number::NINTH);
    }
    
    #[test]
    fn display() {
        assert_eq!(format!("{}", -Number::ELEVENTH), "-11");
    }
}