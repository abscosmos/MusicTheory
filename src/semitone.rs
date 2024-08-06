use std::ops::{Add, Neg, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Semitone(pub i16);

impl Add for Semitone {
    type Output = Semitone;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Semitone {
    type Output = Semitone;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Neg for Semitone {
    type Output = Semitone;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}