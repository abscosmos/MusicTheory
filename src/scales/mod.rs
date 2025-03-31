use std::fmt;
use crate::interval::Interval;

mod old;
pub mod typed_scale;
pub mod exact_scale;
pub mod sized_scale;
pub mod dyn_scale;
mod build_from;

pub(crate) use build_from::*;

// TODO: proper derives for all scale items

const T: Interval = Interval::MAJOR_SECOND;
const S: Interval = Interval::MINOR_SECOND;
const TS: Interval = Interval::MINOR_THIRD;
const TT: Interval = Interval::MAJOR_THIRD;
const A2: Interval = Interval::AUGMENTED_SECOND;


#[repr(u8)]
#[derive(Copy, Clone, Debug, strum_macros::FromRepr)]
pub enum BaseMode7 {
    I = 1,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

impl BaseMode<7> for BaseMode7 {
    fn as_num(&self) -> u8 {
        *self as _
    }

    fn from_num(num: u8) -> Option<Self> where Self: Sized {
        Self::from_repr(num)
    }
}

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait BaseMode<const N: usize>: Copy {
    fn as_num(&self) -> u8;

    fn from_num(num: u8) -> Option<Self> where Self: Sized;
}

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait ScaleDefinition<const N: usize>: fmt::Debug {
    type Mode: ScaleMode<N> + fmt::Debug;
    const INTERVALS: [Interval; N];
}

#[derive(Debug)]
pub struct DiatonicScaleDef;

impl ScaleDefinition<7> for DiatonicScaleDef {
    type Mode = DiatonicMode;
    const INTERVALS: [Interval; 7] = [T, T, S, T, T, T, S];
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, strum_macros::FromRepr)]
pub enum DiatonicMode {
    I = 1,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

impl ScaleMode<7> for DiatonicMode {
    type Base = BaseMode7;

    fn as_num(self) -> u8 {
        self as _
    }

    fn from_num(num: u8) -> Option<Self> where Self: Sized {
        Self::from_repr(num)
    }
}

pub trait ScaleIntervals {

}

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait ScaleMode<const N: usize>: Copy { // from base mode
    type Base: BaseMode<N>;
    
    fn as_num(self) -> u8;

    fn from_num(num: u8) -> Option<Self> where Self: Sized;
    
    fn as_base(self) -> Self::Base {
        Self::Base::from_num(self.as_num())
            .expect("the base mode type should be constructable for all N in [1, N]")
    }
}
