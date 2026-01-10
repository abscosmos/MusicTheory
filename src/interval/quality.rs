use std::fmt;
use std::num::NonZeroU16;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IntervalQuality {
    Diminished(NonZeroU16),
    Minor,
    Perfect,
    Major,
    Augmented(NonZeroU16),
}

impl IntervalQuality {
    pub fn shorthand(self) -> String {
        match self {
            IntervalQuality::Perfect => "P".to_owned(),
            IntervalQuality::Major => "M".to_owned(),
            IntervalQuality::Minor => "m".to_owned(),
            IntervalQuality::Diminished(n) => "d".repeat(n.get() as _),
            IntervalQuality::Augmented(n) => "A".repeat(n.get() as _),
        }
    }

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