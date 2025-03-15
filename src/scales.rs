use std::array;
use crate::interval::Interval;

pub trait Scale: Sized {
    type IntervalSet; 
    
    fn intervals() -> Self::IntervalSet;
}

pub struct ChromaticScale;

impl Scale for ChromaticScale {
    type IntervalSet = [Interval; 12];

    fn intervals() -> Self::IntervalSet {
        let p1 = Interval::PERFECT_UNISON;
        
        let a1 = Interval::AUGMENTED_UNISON;
        let m2 = Interval::MINOR_SECOND;
        
        let intervals = [p1, a1, m2, a1, m2, m2, a1, m2, a1, m2, a1, m2];
        
        let mut acc = p1;
        
        array::from_fn(|i| {
            acc = acc.add(intervals[i]);
            
            acc
        })
    }
}
