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
}