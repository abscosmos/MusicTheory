use std::fmt;
use crate::interval::Interval;

pub mod typed_scale;
pub mod exact_scale;
pub mod sized_scale;
pub mod dyn_scale;
mod build_from;
pub mod numeral;
mod define;
pub mod heptatonic;
pub mod pentatonic;
pub mod chromatic;
pub mod hexatonic;
pub mod octatonic;

pub use numeral::Numeral7;

pub(crate) use build_from::*;
use crate::scales::numeral::Numeral;
// TODO: proper derives for all scale items

const T: Interval = Interval::MAJOR_SECOND;
const S: Interval = Interval::MINOR_SECOND;
const TS: Interval = Interval::MINOR_THIRD;
const TT: Interval = Interval::MAJOR_THIRD;
const A2: Interval = Interval::AUGMENTED_SECOND;

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait ScaleDefinition<const N: usize>: fmt::Debug {
    type Mode: ScaleMode<N> + fmt::Debug;
    const INTERVALS: [Interval; N];
}

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait ScaleMode<const N: usize>: Copy { // from base mode
    type Base: Numeral<N>;
    
    fn as_num(self) -> u8;

    fn from_num(num: u8) -> Option<Self> where Self: Sized;
    
    fn as_base(self) -> Self::Base {
        <Self::Base as Numeral<N>>::from_num(self.as_num())
            .expect("the base mode type should be constructable for all N in [1, N]")
    }
}

impl<const N: usize, T: Numeral<N>> ScaleMode<N> for T {
    type Base = T;

    fn as_num(self) -> u8 {
        self.as_num()
    }

    fn from_num(num: u8) -> Option<Self> where Self: Sized {
        Self::from_num(num)
    }
}
