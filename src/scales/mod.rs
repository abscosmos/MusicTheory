use std::array;
use std::ops::{Add, Index};
use crate::interval::Interval;

pub mod heptatonic;
pub mod pentatonic;
mod define;

use define::define_scale;

const T: Interval = Interval::MAJOR_SECOND;
const S: Interval = Interval::MINOR_SECOND;
const TS: Interval = Interval::MINOR_THIRD;
const TT: Interval = Interval::MAJOR_THIRD;

/* TODO(generic_const_exprs): we need this trait because #![feature(generic_const_exprs)]
    isn't stable yet, and without it, the design of ScaleModes can't be:
    ```
    pub trait ScaleModes {
        const SIZE: usize;
        fn build_from<T: ...>(&self) -> [T; N];
        ...
    }
    ```
*/

pub trait ScaleArr<T>: Index<usize, Output = T> + Sized + seal::Sealed {
    const LEN: usize;
    
    fn from_fn(cb: impl FnMut(usize) -> T) -> Self;
}

impl<T, const N: usize> ScaleArr<T> for [T; N] {
    const LEN: usize = N;
    
    fn from_fn(cb: impl FnMut(usize) -> T) -> Self {
        array::from_fn(cb)
    }
}

pub trait ScaleModes {
    type ScaleArray<T>: ScaleArr<T>;
    
    fn relative_intervals(&self) -> Self::ScaleArray<Interval>;

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> Self::ScaleArray<T> {
        let mode = self.number() as usize;

        let mut curr = root;
        
        let relative_intervals = self.relative_intervals();
        
        let len = self.size();

        Self::ScaleArray::from_fn(|i| {
            let ret = curr.clone();

            #[expect(clippy::clone_on_copy, reason = "Need this clone here to satisfy type system")]
            let ivl = relative_intervals[(i + mode - 1) % len].clone();

            curr = curr.clone() + ivl;

            ret
        })
    }

    fn intervals(&self) -> Self::ScaleArray<Interval> {
        self.build_from(Interval::PERFECT_UNISON)
    }

    fn size(&self) -> usize {
        Self::ScaleArray::<()>::LEN
    }
    
    fn number(&self) -> u8;

    fn from_number(number: u8) -> Option<Self> where Self: Sized;
}

pub trait DynScaleModes {
    fn size() -> usize;

    fn intervals() -> Vec<Interval>;
}

mod seal {
    pub trait Sealed {}
    
    impl<T, const N: usize> Sealed for [T; N] {}
}

#[cfg(test)]
mod tests {
    use crate::pitch::Pitch;
    use crate::scales::heptatonic::HeptatoniaPrimaMode;
    use crate::scales::ScaleModes;

    #[test]
    fn intervals() {
        let ivls = HeptatoniaPrimaMode::LOCRIAN.intervals();
        
        assert_eq!(ivls, HeptatoniaPrimaMode::LOCRIAN.build_from(Pitch::A).map(|p| Pitch::A.distance_to(&p)));
        
        assert_eq!(HeptatoniaPrimaMode::LOCRIAN.build_from(Pitch::A), ivls.map(|i| Pitch::A + i))
    }
}
