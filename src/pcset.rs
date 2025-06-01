use std::fmt;
use strum::IntoEnumIterator;
use crate::pitch_class::PitchClass;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct PitchClassSet(u16);

impl PitchClassSet {
    pub fn new(set: u16) -> Option<Self> {
        (set < 4096).then_some(Self(set))
    }
    
    pub fn get(self) -> u16 {
        self.0
    }
    
    pub fn len(self) -> u8 {
        self.0.count_ones() as _
    }
    
    #[inline(always)]
    fn index(pc: PitchClass) -> u8 {
        11 - pc.chroma()
    }
    
    pub fn from_pitch_classes(pitch_classes: &[PitchClass]) -> Self {
        pitch_classes.into_iter().fold(
            PitchClassSet::default(),
            |set, pc| set.with_set(*pc)
        )
    }
    
    pub fn pitch_classes(self) -> Vec<PitchClass> {
        PitchClass::iter()
            .filter(|pc| self.is_set(*pc))
            .collect()
    }
    
    pub fn is_set(self, pc: PitchClass) -> bool {
        (self.0 >> Self::index(pc)) & 1 == 1
    }
    
    #[must_use = "This method returns a new PitchClassSet instead of mutating the original"]
    pub fn with_set(self, pc: PitchClass) -> Self {
        Self(self.0 | (1 << Self::index(pc)))
    }

    #[must_use = "This method returns a new PitchClassSet instead of mutating the original"]
    pub fn with_cleared(self, pc: PitchClass) -> Self {
        Self(self.0 & !(1 << Self::index(pc)))
    }
}

impl fmt::Debug for PitchClassSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PitchClassSet")
            .field(&format_args!("0b{:012b}", self.0))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::pcset::PitchClassSet;
    use crate::pitch_class::PitchClass;

    #[test]
    fn new() {
        let cde = PitchClassSet::new(2688).expect("in range");
        
        assert_eq!(format!("{cde:?}"), "PitchClassSet(0b101010000000)");
        
        assert_eq!(cde, PitchClassSet::from_pitch_classes(&[PitchClass::C, PitchClass::D, PitchClass::E]));
        
        let pcs = [
            PitchClass::Cs,
            PitchClass::D,
            PitchClass::F,
            PitchClass::G,
            PitchClass::As,
        ];
        
        let set = PitchClassSet::from_pitch_classes(&pcs);
        
        assert_eq!(set.pitch_classes(), pcs);
        
        assert_eq!(pcs.len(), set.len() as _);
    }
}

