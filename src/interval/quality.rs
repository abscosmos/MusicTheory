use std::fmt;
use std::num::NonZeroU16;
use std::str::FromStr;

/// The quality component of an interval, such as "major", "minor", or "perfect".
///
/// # Augmented and Diminished Qualities
///
/// Intervals can be diminished or augmented multiple times (e.g., doubly augmented).
/// The [`NonZeroU16`] parameter in [`Self::Augmented`] and [`Self::Diminished`]
/// indicates how many times the quality is applied.
///
/// For convenience, [`Self::DIMINISHED`] and [`Self::AUGMENTED`] constants are available.
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// // Standard qualities
/// let major = IntervalQuality::Major;
/// let perfect = IntervalQuality::Perfect;
///
/// // Multiple augmentations/diminishments
/// let doubly_aug = IntervalQuality::Augmented(2.try_into().unwrap());
/// assert_eq!(doubly_aug.shorthand(), "AA");
///
/// assert_eq!(IntervalQuality::Major.inverted(), IntervalQuality::Minor);
/// assert_eq!(IntervalQuality::AUGMENTED.inverted(), IntervalQuality::DIMINISHED);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IntervalQuality {
    /// A diminished quality, with the count indicating how many times diminished.
    /// For example, `Diminished(2)` represents a doubly diminished interval.
    Diminished(NonZeroU16),
    /// A minor quality, used for seconds, thirds, sixths, and sevenths.
    Minor,
    /// A perfect quality, used for unisons, fourths, fifths, and octaves.
    Perfect,
    /// A major quality, used for seconds, thirds, sixths, and sevenths.
    Major,
    /// An augmented quality, with the count indicating how many times augmented.
    /// For example, `Augmented(2)` represents a doubly augmented interval.
    Augmented(NonZeroU16),
}

impl IntervalQuality {
    /// Returns the shorthand notation for the interval quality.
    ///
    /// The shorthand uses single letters:
    /// - `P` for [perfect](Self::Perfect)
    /// - `M` for [major](Self::Major)
    /// - `m` for [minor](Self::Minor)
    /// - `d` for [diminished](Self::Diminished) (repeated for multiple diminishments)
    /// - `A` for [augmented](Self::Augmented) (repeated for multiple augmentations)
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(IntervalQuality::Major.shorthand(), "M");
    /// assert_eq!(IntervalQuality::Minor.shorthand(), "m");
    ///
    /// // Multiple augmentations/diminishments repeat the letter
    /// let doubly_dim = IntervalQuality::Diminished(2.try_into().unwrap());
    /// assert_eq!(doubly_dim.shorthand(), "dd");
    /// ```
    pub fn shorthand(self) -> String {
        match self {
            IntervalQuality::Perfect => "P".to_owned(),
            IntervalQuality::Major => "M".to_owned(),
            IntervalQuality::Minor => "m".to_owned(),
            IntervalQuality::Diminished(n) => "d".repeat(n.get() as _),
            IntervalQuality::Augmented(n) => "A".repeat(n.get() as _),
        }
    }

    /// Returns the inverted interval quality.
    ///
    /// When an interval is inverted:
    /// - Perfect remains perfect
    /// - Major becomes minor (and vice versa)
    /// - Augmented becomes diminished (and vice versa)
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// use IntervalQuality as Q;
    ///
    /// assert_eq!(Q::Perfect.inverted(), Q::Perfect);
    /// assert_eq!(Q::Major.inverted(), Q::Minor);
    /// assert_eq!(Q::DIMINISHED.inverted(), Q::AUGMENTED);
    ///
    /// // Degree is preserved
    /// let doubly_aug = Q::Augmented(2.try_into().unwrap());
    /// let doubly_dim = Q::Diminished(2.try_into().unwrap());
    /// assert_eq!(doubly_aug.inverted(), doubly_dim);
    /// ```
    pub fn inverted(self) -> Self {
        use IntervalQuality as Q;

        match self {
            Q::Perfect => Q::Perfect,
            Q::Major => Q::Minor,
            Q::Minor => Q::Major,
            Q::Diminished(n) => Q::Augmented(n),
            Q::Augmented(n) => Q::Diminished(n),
        }
    }
}

/// Error returned when parsing an [`IntervalQuality`] from [`&str`](prim@str) fails.
///
/// # Examples
/// ```
/// # use music_theory::prelude::*;
/// assert!("M".parse::<IntervalQuality>().is_ok());
/// assert!("X".parse::<IntervalQuality>().is_err());
/// ```
#[derive(Debug, thiserror::Error, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("The provided &str could not be converted into a IntervalQuality")]
pub struct ParseIntervalQualityErr;

impl fmt::Display for IntervalQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self).shorthand())
    }
}

impl FromStr for IntervalQuality {
    type Err = ParseIntervalQualityErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "P" => Ok(Self::Perfect),
            "M" => Ok(Self::Major),
            "m" => Ok(Self::Minor),

            "d" => Ok(Self::DIMINISHED),
            "A" => Ok(Self::AUGMENTED),

            "" => Err(ParseIntervalQualityErr),

            s if s.chars().all(|c| c == 'd') => Ok(
                Self::Diminished(NonZeroU16::new(s.len() as _).expect("cannot be zero"))
            ),
            s if s.chars().all(|c| c == 'A') => Ok(
                Self::Augmented(NonZeroU16::new(s.len() as _).expect("cannot be zero"))
            ),

            _ => Err(ParseIntervalQualityErr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use IntervalQuality as IQ;

    const FOUR: NonZeroU16 = NonZeroU16::new(4).expect("nonzero");
    const SIX: NonZeroU16 = NonZeroU16::new(6).expect("nonzero");
    
    #[test]
    fn shorthand() {
        assert_eq!(IQ::Perfect.shorthand(), "P");
        assert_eq!(IQ::Minor.shorthand(), "m");
        assert_eq!(IQ::Major.shorthand(), "M");
        assert_eq!(IQ::AUGMENTED.shorthand(), "A");
        assert_eq!(IQ::DIMINISHED.shorthand(), "d");
        assert_eq!(IQ::Diminished(FOUR).shorthand(), "dddd");
        assert_eq!(IQ::Augmented(SIX).shorthand(), "AAAAAA");
    }

    #[test]
    fn from_str() {
        assert_eq!("P".parse(), Ok(IQ::Perfect));
        assert_eq!("M".parse(), Ok(IQ::Major));
        assert_eq!("m".parse(), Ok(IQ::Minor));
        assert_eq!("A".parse(), Ok(IQ::AUGMENTED));
        assert_eq!("d".parse(), Ok(IQ::DIMINISHED));
        assert_eq!("dddddd".parse(), Ok(IQ::Diminished(SIX)));
        assert_eq!("AAAA".parse(), Ok(IQ::Augmented(FOUR)));

        assert_eq!("".parse::<IQ>(), Err(ParseIntervalQualityErr));
        assert_eq!("c".parse::<IQ>(), Err(ParseIntervalQualityErr));
        assert_eq!("MM".parse::<IQ>(), Err(ParseIntervalQualityErr));
    }
    
    #[test]
    fn display() {
        assert_eq!(IQ::Major.to_string(), "M");
        assert_eq!(format!("{}", IQ::DIMINISHED), "d");
    }
    
    #[test]
    fn inverted() {
        assert_eq!(IQ::Perfect.inverted(), IQ::Perfect);
        assert_eq!(IQ::Major.inverted(), IQ::Minor);
        assert_eq!(IQ::Minor.inverted(), IQ::Major);
        
        assert_eq!(IQ::DIMINISHED.inverted(), IQ::AUGMENTED);
        assert_eq!(IQ::AUGMENTED.inverted(), IQ::DIMINISHED);
        
        assert_eq!(IQ::Augmented(FOUR).inverted(), IQ::Diminished(FOUR));
        assert_eq!(IQ::Diminished(SIX).inverted(), IQ::Augmented(SIX));

        assert_eq!(IQ::Major.inverted().inverted(), IQ::Major);
        assert_eq!(IQ::Minor.inverted().inverted(), IQ::Minor);
        assert_eq!(IQ::Perfect.inverted().inverted(), IQ::Perfect);
        assert_eq!(IQ::DIMINISHED.inverted().inverted(), IQ::DIMINISHED);
        assert_eq!(IQ::AUGMENTED.inverted().inverted(), IQ::AUGMENTED);
    }
}