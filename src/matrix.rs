use crate::pcset::PitchClassSet;
use crate::pitch_class::PitchClass;

pub struct TwelveToneMatrix([PitchClass; 12]);

impl TwelveToneMatrix {
    pub fn new(prime_0: [PitchClass; 12]) -> Option<Self> {
        let pc_set = PitchClassSet::from_iter(prime_0);

        (pc_set.len() == 12).then_some(Self(prime_0))
    }
}