use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;
use strum::IntoEnumIterator;
use crate::Note;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeRule {
    ranges: [RangeInclusive<Note>; 4],
}

impl Default for RangeRule {
    fn default() -> Self {
        Self::new(
            Voice::Soprano.range(),
            Voice::Alto.range(),
            Voice::Tenor.range(),
            Voice::Bass.range(),
        )
        .expect("should be in order")
    }
}

impl RangeRule {
    pub fn new(
        soprano: RangeInclusive<Note>,
        alto: RangeInclusive<Note>,
        tenor: RangeInclusive<Note>,
        bass: RangeInclusive<Note>,
    ) -> Result<Self, (Voice, Voice)> {
        let ranges = [soprano, alto, tenor, bass];
        
        match ranges.windows(2)
            .position(|ranges_pair| {
                let [top, bottom] = &ranges_pair else {
                    unreachable!("windows(2) should have exactly two elements");
                };

                top.start() < bottom.start() || top.end() < bottom.end()
            })
        {
            None => Ok(Self { ranges }),
            Some(error) => {
                let first = Voice::from_repr(error as _).expect("must be in range");
                let second = Voice::from_repr(error as u8 + 1).expect("must be in range");
                
                Err((first, second))
            }
        }
    }
    
    #[inline]
    pub fn range(&self, v: Voice) -> RangeInclusive<Note> {
        self.ranges[v as usize].clone()
    }
    
    pub fn evaluate(&self, v: Voicing) -> Result<(), Voice> {
        for voice in Voice::iter() {
            if !self.range(voice).contains(&v[voice]) {
                return Err(voice)
            }
        }
        
        Ok(())
    }
}

impl Hash for RangeRule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ranges.clone().map(RangeInclusive::into_inner).hash(state);
    }
}