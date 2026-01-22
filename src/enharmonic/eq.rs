/// Trait for comparing musical objects enharmonically.
///
/// Two musical objects are enharmonically equal if they represent the same
/// pitch class or chromatic distance, even if they are spelled differently.
/// For example, C# and Db are enharmonically equal because they represent
/// the same pitch class, despite having different spellings.
///
/// # Examples
///
/// ```
/// # use music_theory::{Pitch, Interval};
/// use music_theory::EnharmonicEq as _;
///
/// // Different spellings of the same pitch class
/// assert!(Pitch::C_SHARP.eq_enharmonic(&Pitch::D_FLAT));
/// assert!(Pitch::F_SHARP.eq_enharmonic(&Pitch::G_FLAT));
///
/// // Different spellings of intervals with the same chromatic distance
/// assert!(Interval::AUGMENTED_FOURTH.eq_enharmonic(&Interval::DIMINISHED_FIFTH));
/// assert!(Interval::MAJOR_THIRD.eq_enharmonic(&Interval::DIMINISHED_FOURTH));
/// ```
pub trait EnharmonicEq {
    /// Checks if two musical objects are enharmonically equivalent.
    ///
    /// Returns `true` if the objects represent the same pitch class or chromatic
    /// distance, regardless of their spelling.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::{Pitch, Interval};
    /// use music_theory::EnharmonicEq as _;
    ///
    /// assert!(Pitch::C_SHARP.eq_enharmonic(&Pitch::D_FLAT));
    /// assert!(Interval::MINOR_THIRD.eq_enharmonic(&Interval::AUGMENTED_SECOND));
    /// ```
    fn eq_enharmonic(&self, other: &Self) -> bool;

    /// Checks if two musical objects are not enharmonically equivalent.
    ///
    /// This is the logical inverse of [`eq_enharmonic`](Self::eq_enharmonic).
    /// The default implementation is almost always sufficient, and should not be overridden without very good reason.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::Pitch;
    /// use music_theory::EnharmonicEq as _;
    ///
    /// assert!(Pitch::C.ne_enharmonic(&Pitch::A));
    /// ```
    fn ne_enharmonic(&self, other: &Self) -> bool {
        !self.eq_enharmonic(other)
    }
}