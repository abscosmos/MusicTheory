use std::cmp::{self, Ordering};

/// Trait for ordering musical objects enharmonically.
///
/// Provides a comparison that ignores spelling differences and compares
/// based on chromatic position. For intervals, this compares by semitone size.
/// For pitches, this compares by their position in the chromatic scale.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// use music_theory::enharmonic::EnharmonicOrd as _;
/// use std::cmp::Ordering;
///
/// // C# and Db occupy the same chromatic position
/// assert_eq!(
///     Pitch::C_SHARP.cmp_enharmonic(&Pitch::D_FLAT),
///     Ordering::Equal
/// );
///
/// // C is chromatically lower than D
/// assert_eq!(
///     Pitch::C.cmp_enharmonic(&Pitch::D),
///     Ordering::Less
/// );
///
/// // Compare intervals by chromatic distance
/// assert_eq!(
///     Interval::MAJOR_THIRD.cmp_enharmonic(&Interval::DIMINISHED_FOURTH),
///     Ordering::Equal
/// );
/// ```
pub trait EnharmonicOrd {
    /// Compares two musical objects enharmonically.
    ///
    /// Returns [`Ordering::Equal`] if the objects are enharmonically equivalent,
    /// [`Ordering::Less`] if `self` is lower than `rhs`, and [`Ordering::Greater`]
    /// if `self` is higher than `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(
    ///     Pitch::C_SHARP.cmp_enharmonic(&Pitch::D_FLAT),
    ///     Ordering::Equal
    /// );
    ///
    /// assert_eq!(
    ///     Pitch::C.cmp_enharmonic(&Pitch::E),
    ///     Ordering::Less
    /// );
    ///
    /// assert_eq!(
    ///     Interval::PERFECT_FIFTH.cmp_enharmonic(&Interval::MAJOR_THIRD),
    ///     Ordering::Greater
    /// );
    /// ```
    fn cmp_enharmonic(&self, other: &Self) -> Ordering;

    /// Tests enharmonically less than (for `self` and `other`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert!(Pitch::C.lt_enharmonic(&Pitch::D));
    /// ```
    fn lt_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_lt()
    }

    /// Tests enharmonically less than or equal to (for `self` and `other`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert!(Pitch::C.le_enharmonic(&Pitch::C_SHARP));
    /// assert!(Pitch::C_SHARP.le_enharmonic(&Pitch::D_FLAT));
    /// ```
    fn le_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_le()
    }

    /// Tests enharmonically greater than (for `self` and `other`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert!(Pitch::E.gt_enharmonic(&Pitch::C));
    /// ```
    fn gt_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_gt()
    }

    /// Tests enharmonically greater than or equal to (for `self` and `other`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert!(Pitch::D.ge_enharmonic(&Pitch::C_SHARP));
    /// assert!(Pitch::C_SHARP.ge_enharmonic(&Pitch::D_FLAT));
    /// ```
    fn ge_enharmonic(&self, other: &Self) -> bool {
        self.cmp_enharmonic(other).is_ge()
    }

    /// Compares and returns the maximum of two values enharmonically.
    ///
    /// Returns the second argument if the comparison determines them to be enharmonically equal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert_eq!(Pitch::C.max_enharmonic(Pitch::E), Pitch::E);
    /// ```
    fn max_enharmonic(self, other: Self) -> Self
    where Self: Sized
    {
        cmp::max_by(self, other, EnharmonicOrd::cmp_enharmonic)
    }

    /// Compares and returns the minimum of two values enharmonically.
    ///
    /// Returns the first argument if the comparison determines them to be enharmonically equal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert_eq!(Pitch::C.min_enharmonic(Pitch::E), Pitch::C);
    /// ```
    fn min_enharmonic(self, other: Self) -> Self
    where Self: Sized
    {
        cmp::min_by(self, other, EnharmonicOrd::cmp_enharmonic)
    }

    /// Restrict a value to a certain interval enharmonically.
    ///
    /// Returns `max` if `self` is enharmonically greater than `max`, and `min` if `self` is
    /// enharmonically less than `min`. Otherwise this returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max` enharmonically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// use music_theory::enharmonic::EnharmonicOrd as _;
    ///
    /// assert_eq!(Pitch::C.clamp_enharmonic(Pitch::E, Pitch::G), Pitch::E);
    /// assert_eq!(Pitch::F.clamp_enharmonic(Pitch::E, Pitch::G), Pitch::F);
    /// assert_eq!(Pitch::B.clamp_enharmonic(Pitch::E, Pitch::G), Pitch::G);
    /// ```
    fn clamp_enharmonic(self, min: Self, max: Self) -> Self
    where Self: Sized
    {
        assert!(
            min.le_enharmonic(&max),
            "min must be less than max!"
        );

        if self.lt_enharmonic(&min) {
            min
        } else if self.gt_enharmonic(&max) {
            max
        } else {
            self
        }
    }
}