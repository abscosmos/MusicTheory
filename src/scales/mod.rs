use std::array;
use std::ops::Add;
use crate::interval::Interval;

pub mod heptatonic;
pub mod pentatonic;
mod define;
pub mod rework;

use define::define_scale;

const T: Interval = Interval::MAJOR_SECOND;
const S: Interval = Interval::MINOR_SECOND;
const TS: Interval = Interval::MINOR_THIRD;
const TT: Interval = Interval::MAJOR_THIRD;
const A2: Interval = Interval::AUGMENTED_SECOND;

pub trait ScaleModes<const LEN: usize>: Sized {
    const RELATIVE_INTERVALS: [Interval; LEN];

    fn build_from<T: Add<Interval, Output = T> + Clone>(&self, root: T) -> [T; LEN] {
        let mode = self.number() as usize;

        let mut curr = root;

        array::from_fn(|i| {
            let ret = curr.clone();

            curr = curr.clone() + Self::RELATIVE_INTERVALS[(i + mode - 1) % LEN];

            ret
        })
    }

    fn intervals(&self) -> [Interval; LEN] {
        self.build_from(Interval::PERFECT_UNISON)
    }

    fn number(&self) -> u8;

    fn from_number(number: u8) -> Option<Self>;
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
