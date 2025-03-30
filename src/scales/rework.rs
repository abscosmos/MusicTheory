use std::array;
use std::ops::Add;
use crate::interval::Interval;

use super::{S, T, TS, TT, A2};

pub struct ScaleIntervals<const N: usize> {
    relative_intervals: [Interval; N],
}

impl<const N: usize> ScaleIntervals<N> {
    pub fn new(relative_intervals: [Interval; N]) -> Option<Self> {
        if relative_intervals.iter().copied().reduce(Add::add) == Some(Interval::PERFECT_OCTAVE) {
            Some(Self { relative_intervals })
        } else {
            None
        }
    }

    // this only checks if the semitones add up, and needs to be hardcoded to match intervals
    // TODO: redesign this function?
    const fn new_const_or_panic(relative_intervals: [Interval; N]) -> Self {
        let mut sum = 0;

        // TODO: iterating over a list in a const context is horrible
        let mut i = 0;

        while i < relative_intervals.len() {
            sum += match relative_intervals[i] {
                S => 1,
                T => 2,
                TS | A2 => 3,
                TT => 4,
                // TODO: const_panic_like :) 
                _ => panic!("one or more of the intervals hasn't been hardcoded for this function"),
            };
            
            i += 1;
        }
        
        if sum != 12 {
            panic!("intervals don't add up to 12 semitones");
        }
        
        Self { relative_intervals }
    }
    
    // TODO: mode: impl Degree<N>
    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T, mode: u8) -> [T; N] {
        assert!(mode >= 1 && mode as usize <= N, "TODO: make degree enum; mode should be in range");
        
        let mut curr = root;

        array::from_fn(|i| {
            let ret = curr.clone();

            curr = curr.clone() + self.relative_intervals[(i + mode as usize - 1) % N];

            ret
        })
    }
    
    pub fn intervals_from_root(&self, mode: u8) -> [Interval; N] {
        self.build_from(Interval::PERFECT_UNISON, mode)
    }
}