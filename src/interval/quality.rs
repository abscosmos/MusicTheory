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
/// If importing this type conflicts with other types, consider aliasing it:
/// ```
/// use music_theory::interval::Quality as IntervalQuality;
/// # let _ = IntervalQuality::Major;
/// ```
///
/// # Examples
///
/// ```
/// # use music_theory::interval::Quality;
/// // Standard qualities
/// let major = Quality::Major;
/// let perfect = Quality::Perfect;
///
/// // Multiple augmentations/diminishments
/// let doubly_aug = Quality::Augmented(2.try_into().unwrap());
/// assert_eq!(doubly_aug.shorthand(), "AA");
///
/// assert_eq!(Quality::Major.inverted(), Quality::Minor);
/// assert_eq!(Quality::AUGMENTED.inverted(), Quality::DIMINISHED);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Quality {
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

impl Quality {
    /// Returns `true` if this quality is augmented (of any degree).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::interval::Quality;
    /// assert!(Quality::AUGMENTED.is_augmented());
    /// assert!(!Quality::Major.is_augmented());
    ///
    /// let doubly_aug = Quality::Augmented(2.try_into().unwrap());
    /// assert!(doubly_aug.is_augmented());
    /// ```
    pub fn is_augmented(self) -> bool {
        matches!(self, Self::Augmented(_))
    }

    /// Returns `true` if this quality is diminished (of any degree).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::interval::Quality;
    /// assert!(Quality::DIMINISHED.is_diminished());
    /// assert!(!Quality::Minor.is_diminished());
    ///
    /// let doubly_dim = Quality::Diminished(2.try_into().unwrap());
    /// assert!(doubly_dim.is_diminished());
    /// ```
    pub fn is_diminished(self) -> bool {
        matches!(self, Self::Diminished(_))
    }

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
    /// # use music_theory::interval::Quality;
    /// assert_eq!(Quality::Major.shorthand(), "M");
    /// assert_eq!(Quality::Minor.shorthand(), "m");
    ///
    /// // Multiple augmentations/diminishments repeat the letter
    /// let doubly_dim = Quality::Diminished(2.try_into().unwrap());
    /// assert_eq!(doubly_dim.shorthand(), "dd");
    /// ```
    pub fn shorthand(self) -> String {
        match self {
            Quality::Perfect => "P".to_owned(),
            Quality::Major => "M".to_owned(),
            Quality::Minor => "m".to_owned(),
            Quality::Diminished(n) => "d".repeat(n.get() as _),
            Quality::Augmented(n) => "A".repeat(n.get() as _),
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
    /// # use music_theory::interval::Quality;
    /// assert_eq!(Quality::Perfect.inverted(), Quality::Perfect);
    /// assert_eq!(Quality::Major.inverted(), Quality::Minor);
    /// assert_eq!(Quality::DIMINISHED.inverted(), Quality::AUGMENTED);
    ///
    /// // Degree is preserved
    /// let doubly_aug = Quality::Augmented(2.try_into().unwrap());
    /// let doubly_dim = Quality::Diminished(2.try_into().unwrap());
    /// assert_eq!(doubly_aug.inverted(), doubly_dim);
    /// ```
    pub fn inverted(self) -> Self {
        match self {
            Quality::Perfect => Quality::Perfect,
            Quality::Major => Quality::Minor,
            Quality::Minor => Quality::Major,
            Quality::Diminished(n) => Quality::Augmented(n),
            Quality::Augmented(n) => Quality::Diminished(n),
        }
    }
}

/// Error returned when parsing an [`Quality`] from [`&str`](prim@str) fails.
///
/// # Examples
/// ```
/// # use music_theory::interval::Quality;
/// assert!("M".parse::<Quality>().is_ok());
/// assert!("X".parse::<Quality>().is_err());
/// ```
#[derive(Debug, thiserror::Error, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("The provided &str could not be converted into a IntervalQuality")]
pub struct ParseIntervalQualityErr;

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self).shorthand())
    }
}

impl FromStr for Quality {
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

    const FOUR: NonZeroU16 = NonZeroU16::new(4).expect("nonzero");
    const SIX: NonZeroU16 = NonZeroU16::new(6).expect("nonzero");
    
    #[test]
    fn shorthand() {
        assert_eq!(Quality::Perfect.shorthand(), "P");
        assert_eq!(Quality::Minor.shorthand(), "m");
        assert_eq!(Quality::Major.shorthand(), "M");
        assert_eq!(Quality::AUGMENTED.shorthand(), "A");
        assert_eq!(Quality::DIMINISHED.shorthand(), "d");
        assert_eq!(Quality::Diminished(FOUR).shorthand(), "dddd");
        assert_eq!(Quality::Augmented(SIX).shorthand(), "AAAAAA");
    }

    #[test]
    fn from_str() {
        assert_eq!("P".parse(), Ok(Quality::Perfect));
        assert_eq!("M".parse(), Ok(Quality::Major));
        assert_eq!("m".parse(), Ok(Quality::Minor));
        assert_eq!("A".parse(), Ok(Quality::AUGMENTED));
        assert_eq!("d".parse(), Ok(Quality::DIMINISHED));
        assert_eq!("dddddd".parse(), Ok(Quality::Diminished(SIX)));
        assert_eq!("AAAA".parse(), Ok(Quality::Augmented(FOUR)));

        assert_eq!("".parse::<Quality>(), Err(ParseIntervalQualityErr));
        assert_eq!("c".parse::<Quality>(), Err(ParseIntervalQualityErr));
        assert_eq!("MM".parse::<Quality>(), Err(ParseIntervalQualityErr));
    }
    
    #[test]
    fn display() {
        assert_eq!(Quality::Major.to_string(), "M");
        assert_eq!(format!("{}", Quality::DIMINISHED), "d");
    }
    
    #[test]
    fn inverted() {
        assert_eq!(Quality::Perfect.inverted(), Quality::Perfect);
        assert_eq!(Quality::Major.inverted(), Quality::Minor);
        assert_eq!(Quality::Minor.inverted(), Quality::Major);
        
        assert_eq!(Quality::DIMINISHED.inverted(), Quality::AUGMENTED);
        assert_eq!(Quality::AUGMENTED.inverted(), Quality::DIMINISHED);
        
        assert_eq!(Quality::Augmented(FOUR).inverted(), Quality::Diminished(FOUR));
        assert_eq!(Quality::Diminished(SIX).inverted(), Quality::Augmented(SIX));

        assert_eq!(Quality::Major.inverted().inverted(), Quality::Major);
        assert_eq!(Quality::Minor.inverted().inverted(), Quality::Minor);
        assert_eq!(Quality::Perfect.inverted().inverted(), Quality::Perfect);
        assert_eq!(Quality::DIMINISHED.inverted().inverted(), Quality::DIMINISHED);
        assert_eq!(Quality::AUGMENTED.inverted().inverted(), Quality::AUGMENTED);
    }
}