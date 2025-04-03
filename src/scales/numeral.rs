use mt_macros::{numeral};

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait Numeral<const N: usize>: Copy {
    fn as_num(self) -> u8;

    fn from_num(num: u8) -> Option<Self> where Self: Sized;
}

#[numeral(1)]
pub enum Numeral1 {}

#[numeral(2)]
pub enum Numeral2 {}

#[numeral(3)]
pub enum Numeral3 {}

#[numeral(4)]
pub enum Numeral4 {}

#[numeral(5)]
pub enum Numeral5 {}

#[numeral(6)]
pub enum Numeral6 {}

#[numeral(7)]
pub enum Numeral7 {}

#[numeral(8)]
pub enum Numeral8 {}

#[numeral(9)]
pub enum Numeral9 {}

#[numeral(10)]
pub enum Numeral10 {}

#[numeral(11)]
pub enum Numeral11 {}

#[numeral(12)]
pub enum Numeral12 {}