use std::fmt;
use std::ops::{BitAnd, BitXor, Not};
use strum::IntoEnumIterator;
use crate::pitch_class::PitchClass;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct PitchClassSet(u16);

impl PitchClassSet {
    const MASK: u16 = 0xfff;
    
    pub fn new(set: u16) -> Option<Self> {
        (set <= Self::MASK).then_some(Self(set))
    }

    #[inline(always)]
    pub fn new_masked(set: u16) -> Self {
        Self(set & Self::MASK)
    }

    #[inline(always)]
    pub fn get(self) -> u16 {
        self.0
    }
    
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn len(self) -> u8 {
        self.0.count_ones() as _
    }
    
    #[inline(always)]
    fn index(pc: PitchClass) -> u8 {
        11 - pc.chroma()
    }
    
    pub fn from_pitch_classes(pitch_classes: &[PitchClass]) -> Self {
        pitch_classes.iter().copied().fold(
            PitchClassSet::default(),
            PitchClassSet::with_set,
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

    #[inline(always)]
    pub fn is_superset_of(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }

    #[inline(always)]
    pub fn is_subset_of(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == self.0
    }

    #[inline(always)]
    pub fn disjoint(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == 0
    }

    #[inline(always)]
    pub fn union(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    #[inline(always)]
    pub fn intersection(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    #[inline(always)]
    pub fn difference(self, rhs: Self) -> Self {
        Self(self.0 & !rhs.0)
    }

    #[inline(always)]
    pub fn symmetric_difference(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    #[inline(always)]
    pub fn complement(self) -> Self {
        Self::new_masked(!self.0)
    }
}

impl fmt::Debug for PitchClassSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PitchClassSet")
            .field(&format_args!("0b{:012b}", self.0))
            .finish()
    }
}

impl Not for PitchClassSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl BitAnd for PitchClassSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitXor for PitchClassSet {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl FromIterator<PitchClass> for PitchClassSet {
    fn from_iter<T: IntoIterator<Item = PitchClass>>(iter: T) -> Self {
        iter.into_iter().fold(
            PitchClassSet::default(),
            PitchClassSet::with_set,
        )
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
    }
    
    #[test]
    fn pitch_classes() {
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
    
    #[test]
    fn set_ops() {
        let set = PitchClassSet::new(0b000011001100).expect("only necessary bits set");
        
        assert_eq!(!set, PitchClassSet::new(0b111100110011).expect("only necessary bits set"));
        
        assert_eq!(!!set, set);
        
        let cmaj = [
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
            PitchClass::G,
            PitchClass::A,
            PitchClass::B,
        ];
        
        let cmaj_pentatonic = [
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
            PitchClass::G,
        ];
        
        let pcs_cmaj = PitchClassSet::from_pitch_classes(&cmaj);
        let pcs_cmaj_pentatonic = PitchClassSet::from_pitch_classes(&cmaj_pentatonic);
        
        assert!(pcs_cmaj.is_superset_of(pcs_cmaj_pentatonic));
        assert!(pcs_cmaj_pentatonic.is_subset_of(pcs_cmaj));
    }
}

