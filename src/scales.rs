use std::array;
use std::ops::Add;
use crate::interval::Interval;

const T: Interval = Interval::MAJOR_SECOND;
const S: Interval = Interval::MINOR_SECOND;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum DiatonicMode {
    Ionian = 1,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl DiatonicMode {
    pub const MAJOR: Self = Self::Ionian;
    pub const NATURAL_MINOR: Self = Self::Aeolian;
    
    const INTERVALS: [Interval; 7] = [T, T, S, T, T, T, S];

    // TODO: do we like this name?
    pub fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; 7] {
        let mode = self.number() as usize;
        
        let mut curr = root;
        
        array::from_fn(|i| {
            let ret = curr.clone();
            
            curr = curr.clone() + Self::INTERVALS[(i + mode - 1) % Self::INTERVALS.len()];
            
            ret
        })
    }
    
    pub fn intervals(&self) -> [Interval; 7] {
        self.build_from(Interval::PERFECT_UNISON)
    }
    
    pub fn number(&self) -> u8 {
        *self as _
    }
}

#[cfg(test)]
mod tests {
    use crate::pitch::Pitch;
    use crate::scales::DiatonicMode;

    #[test]
    fn intervals() {
        let ivls = DiatonicMode::Locrian.intervals();
        
        assert_eq!(ivls, DiatonicMode::Locrian.build_from(Pitch::A).map(|p| Pitch::A.distance_to(&p)));
        
        assert_eq!(DiatonicMode::Locrian.build_from(Pitch::A), ivls.map(|i| Pitch::A + i))
    }
}
