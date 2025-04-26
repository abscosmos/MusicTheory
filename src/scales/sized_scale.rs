use std::ops::Add;
use crate::interval::Interval;
use crate::scales;
use crate::scales::dyn_scale::{DynScale, DynamicScale};
use crate::scales::numeral::Numeral;

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait SizedScale<const N: usize> {
    fn relative_intervals(&self) -> [Interval; N];

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; N] {
        scales::build_from(self.relative_intervals(), root)
    }
    
    fn to_dyn(&self) -> DynamicScale {
        DynamicScale::new(self.relative_intervals()).expect("must add up to a P8")
    }

    fn interval_between_degrees<Num: Numeral<N>>(&self, start: Num, end: Num) -> Interval {
        let n = N as u8;
        
        let start = start.as_num() - 1;
        let end = end.as_num() - 1;

        let size = (end + n - start) % n;

        let mut ivls = self.relative_intervals();
        
        ivls.rotate_left(start as usize);
        
        ivls[0..size as usize]
            .iter()
            .copied()
            .sum()
    }
}