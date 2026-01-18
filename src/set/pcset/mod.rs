use crate::pitch::PitchClass;
use crate::set::IntervalClassVector;
use crate::semitone::Semitones;

mod fmt;
pub use fmt::*;

mod ops;
#[expect(unused_imports, reason = "ops module is for implementing std::ops traits")]
pub use ops::*;

mod into_iter;
pub use into_iter::*;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PitchClassSet(u16);

impl PitchClassSet {
    pub const EMPTY: Self = Self(0);
    pub const CHROMATIC_AGGREGATE: Self = Self(Self::MASK);

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

    pub fn interval_class_vector(self) -> IntervalClassVector {
        let mut icv = [0u8; 6];

        let mut remaining = self.into_iter();

        while let Some(pc1) = remaining.next() {
            // this only iterates over the remaining (which haven't yet been consumed)
            for pc2 in remaining.clone() {
                let interval = pc1.semitones_to(pc2).0;

                let ic = if interval > 6 { 12 - interval } else { interval };

                icv[(ic - 1) as usize] += 1;
            }
        }

        IntervalClassVector::new(icv).expect("all pcsets should be valid icvs")
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

    /// Transpose all pitch classes by the given number of semitones.
    ///
    /// This is equivalent to using the [+ operator](Add::add).
    ///
    /// # Examples
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let c_maj = [PitchClass::C, PitchClass::E, PitchClass::G];
    ///
    /// let c_maj_pcset = PitchClassSet::from_iter(c_maj);
    ///
    /// // Transpose up by 7 semitones to get G major
    /// let g_maj_pcset = c_maj_pcset.transpose(Semitones(7));
    ///
    /// assert_eq!(
    ///     g_maj_pcset,
    ///     PitchClassSet::from_iter(
    ///         // maps to: [G, B, D]
    ///         c_maj.map(|pc| pc + Semitones(7))
    ///     ),
    /// );
    /// ```
    #[must_use = "This method returns a new PitchClassSet instead of mutating the original"]
    pub fn transpose(self, semitones: Semitones) -> Self {
        let shift = semitones.normalize().0 as u32;

        // Rotate bits (accounting for 12-bit width, not 16)
        let rotated = (self.0 >> shift) | (self.0 << (12 - shift));

        Self::new_masked(rotated)
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

    /// Returns a helper type that displays pitch classes as their chroma values.
    ///
    /// # Example
    ///
    /// ```
    /// # use music_theory::prelude::*;
    /// # use music_theory::set::PitchClassSet;
    /// let set = PitchClassSet::from_iter([
    ///     PitchClass::C,
    ///     PitchClass::E,
    ///     PitchClass::G
    /// ]);
    ///
    /// assert_eq!(
    ///     format!("{}", set.display_chromas()),
    ///     "{0, 4, 7}"
    /// );
    /// ```
    pub fn display_chromas(self) -> DisplayChromas {
        DisplayChromas(self)
    }
}

impl FromIterator<PitchClass> for PitchClassSet {
    fn from_iter<T: IntoIterator<Item = PitchClass>>(iter: T) -> Self {
        let mut new = Self::default();
        new.extend(iter);
        new
    }
}

impl Extend<PitchClass> for PitchClassSet {
    fn extend<T: IntoIterator<Item=PitchClass>>(&mut self, iter: T) {
        *self = iter.into_iter().fold(*self, PitchClassSet::with_set);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cde = PitchClassSet::new(2688).expect("in range");

        assert_eq!(format!("{cde:?}"), "{C (0), D (2), E (4)}");

        assert_eq!(cde, PitchClassSet::from_iter([PitchClass::C, PitchClass::D, PitchClass::E]));
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
        
        let pcs_cmaj = PitchClassSet::from_iter(cmaj);
        let pcs_cmaj_pentatonic = PitchClassSet::from_iter(cmaj_pentatonic);
        
        assert!(pcs_cmaj.is_superset_of(pcs_cmaj_pentatonic));
        assert!(pcs_cmaj_pentatonic.is_subset_of(pcs_cmaj));
    }
}

