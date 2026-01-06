use std::fmt;
use crate::interval::Interval;

pub mod typed_scale;
pub mod exact_scale;
pub mod sized_scale;
pub mod dyn_scale;
mod build_from;
pub mod numeral;
pub mod definition;
pub mod rooted;
pub mod analyze;

pub use numeral::Numeral7;

pub(crate) use build_from::*;
use crate::scales::numeral::Numeral;
// TODO: proper derives for all scale items

/*
TODO:
    The current implementation focuses heavily on being very strongly typed,
    but this makes it hard to develop new scale features and even harder to
    use in general. A future rework should result in only ONE Scale type,
    and perhaps another type of scale which contains extra metadata about
    how the scale was built? 
*/


// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait ScaleDefinition<const N: usize>: fmt::Debug {
    type Mode: ScaleMode<N> + fmt::Debug;
    const INTERVALS: [Interval; N];
}

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait ScaleMode<const N: usize>: Copy + Default { // from base mode
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
