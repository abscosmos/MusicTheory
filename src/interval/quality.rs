use std::fmt;
use std::num::NonZeroU16;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum IntervalQuality {
    Diminished(NonZeroU16),
    Minor,
    Perfect,
    Major,
    Augmented(NonZeroU16),
}

impl IntervalQuality {
    pub fn shorthand(&self) -> String {
        match self {
            IntervalQuality::Perfect => "P".to_owned(),
            IntervalQuality::Major => "M".to_owned(),
            IntervalQuality::Minor => "m".to_owned(),
            IntervalQuality::Diminished(n) => "d".repeat(n.get() as _),
            IntervalQuality::Augmented(n) => "A".repeat(n.get() as _),
        }
    }

    pub fn inverted(&self) -> Self {
        use IntervalQuality as Q;

        match *self {
            Q::Perfect => Q::Perfect,
            Q::Major => Q::Minor,
            Q::Minor => Q::Major,
            Q::Diminished(n) => Q::Augmented(n),
            Q::Augmented(n) => Q::Diminished(n),
        }
    }
}

impl fmt::Display for IntervalQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shorthand())
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