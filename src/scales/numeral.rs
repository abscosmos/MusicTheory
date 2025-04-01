pub trait Numeral<const N: usize>: Copy {
    fn as_num(self) -> u8;

    fn from_num(num: u8) -> Option<Self> where Self: Sized;
}

#[repr(u8)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, strum_macros::FromRepr)]
pub enum Numeral1 {
    I = 1,
}

impl Numeral<1> for Numeral1 {
    fn as_num(self) -> u8 {
        self as _
    }

    fn from_num(num: u8) -> Option<Self> where Self: Sized {
        Self::from_repr(num)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, strum_macros::FromRepr)]
pub enum Numeral2 {
    I = 1,
    II,
}

impl Numeral<2> for Numeral2 {
    fn as_num(self) -> u8 {
        self as _
    }

    fn from_num(num: u8) -> Option<Self> where Self: Sized {
        Self::from_repr(num)
    }
}