use std::ops::Deref;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum TwelveToneRowForm {
    #[default]
    Prime,
    Retrograde,
    Inversion,
    RetrogradeInversion,
}

impl TwelveToneRowForm {
    pub const ALL: [Self; 4] = [Self::Prime, Self::Retrograde, Self::Inversion, Self::RetrogradeInversion];
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TwelveToneRowNumber(pub u8);

impl TwelveToneRowNumber {
    pub const COUNT: u8 = 12;

    pub const fn new(n: u8) -> Option<Self> {
        if n < Self::COUNT {
            Some(Self(n))
        } else {
            None
        }
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

impl Deref for TwelveToneRowNumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct TwelveToneRowLabel(pub TwelveToneRowForm, pub TwelveToneRowNumber);

impl TwelveToneRowLabel {
    pub const fn new(form: TwelveToneRowForm, number: u8) -> Option<Self> {
        // using match instead of Option::map for const
        match TwelveToneRowNumber::new(number) {
            Some(n) => Some(Self(form, n)),
            None => None,
        }
    }

    pub fn number(&self) -> u8 {
        self.1.get()
    }
}