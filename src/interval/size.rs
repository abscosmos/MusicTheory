use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[repr(u8)]
pub enum IntervalSize {
    Unison = 1,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
    Thirteenth,
    Fourteenth,
    Fifteenth,
}

impl IntervalSize {
    pub fn as_simple(&self) -> Self {
        match self {
            IntervalSize::Ninth => IntervalSize::Second,
            IntervalSize::Tenth => IntervalSize::Third,
            IntervalSize::Eleventh => IntervalSize::Fourth,
            IntervalSize::Twelfth => IntervalSize::Fifth,
            IntervalSize::Thirteenth => IntervalSize::Sixth,
            IntervalSize::Fourteenth => IntervalSize::Seventh,
            IntervalSize::Fifteenth => IntervalSize::Octave,
            _ => *self,
        }
    }

    pub fn shorthand(&self) -> u8 {
        *self as _
    }

    pub fn is_perfect(&self) -> bool {
        match self.as_simple() {
            IntervalSize::Unison => true,
            IntervalSize::Fourth => true,
            IntervalSize::Fifth => true,
            IntervalSize::Octave => true,
            _ => false,
        }
    }

    pub fn inverted(&self) -> Self {
        use IntervalSize as S;

        match self {
            S::Unison => S::Unison,
            S::Second => S::Seventh,
            S::Third => S::Sixth,
            S::Fourth => S::Fifth,
            S::Fifth => S::Fourth,
            S::Sixth => S::Third,
            S::Seventh => S::Second,
            S::Octave => S::Octave,

            S::Ninth => S::Fourteenth,
            S::Tenth => S::Thirteenth,
            S::Eleventh => S::Twelfth,
            S::Twelfth => S::Eleventh,
            S::Thirteenth => S::Tenth,
            S::Fourteenth => S::Ninth,
            S::Fifteenth => S::Fifteenth,
        }
    }
}