use std::fmt;
use crate::PitchClass;
use crate::set::PitchClassSet;

/// Helper type for displaying pitch class sets as integers (chroma values).
///
/// Obtained via [`PitchClassSet::display_chromas()`].
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// # use music_theory::set::PitchClassSet;
/// let set = PitchClassSet::from_iter([
///     PitchClass::C,
///     PitchClass::E,
///     PitchClass::G
/// ]);
///
/// assert_eq!(format!("{}", set.display_chromas()), "{0, 4, 7}");
/// ```
pub struct DisplayChromas(pub(super) PitchClassSet);

impl fmt::Display for DisplayChromas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(self.0.into_iter().map(PitchClass::chroma))
            .finish()
    }
}

impl fmt::Debug for PitchClassSet {
    /// Formats the pitch class set for debugging.
    ///
    /// Shows each pitch class with its name and chroma value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let set = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G
    /// ]);
    ///
    /// assert_eq!(format!("{:?}", set), "{C (0), E (4), G (7)}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct PcWithChroma(PitchClass);

        impl fmt::Debug for PcWithChroma {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?} ({})", self.0, self.0.chroma())
            }
        }

        f.debug_set()
            .entries(self.into_iter().map(PcWithChroma))
            .finish()
    }
}

impl fmt::Display for PitchClassSet {
    /// Formats the pitch class set for display.
    ///
    /// Shows pitch classes by their names in a set notation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let set = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G
    /// ]);
    ///
    /// assert_eq!(format!("{}", set), "{C, E, G}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(*self).finish()
    }
}

impl fmt::Binary for PitchClassSet {
    /// Formats the pitch class set as a 12-bit binary number.
    ///
    /// Each bit represents the presence (1) or absence (0) of a pitch class.
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let set = PitchClassSet::from_iter([PitchClass::C]);
    ///
    /// assert_eq!(format!("{:b}", set), "100000000000");
    /// assert_eq!(format!("{:#b}", set), "0b100000000000");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0b")?;
        }

        write!(f, "{:012b}", self.0)
    }
}