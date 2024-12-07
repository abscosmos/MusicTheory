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