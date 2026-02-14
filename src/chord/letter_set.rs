use crate::Letter;

/// A 7-bit set of [`Letter`] values (C through B).
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct LetterSet(u8);

impl LetterSet {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(0b1111111);

    pub const fn is_set(self, letter: Letter) -> bool {
        self.0 & Self::index(letter) != 0
    }

    pub const fn with_set(self, letter: Letter) -> Self {
        Self(self.0 | Self::index(letter))
    }

    #[inline(always)]
    const fn index(letter: Letter) -> u8 {
        1 << letter.step()
    }
}