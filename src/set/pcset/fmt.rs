use std::fmt;
use crate::pitch::PitchClass;
use crate::set::PitchClassSet;

/// Helper type for displaying pitch class sets as integers (chroma values).
///
/// Obtained via [`PitchClassSet::display_chromas()`].
pub struct DisplayChromas(pub(super) PitchClassSet);

impl fmt::Display for DisplayChromas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(self.0.into_iter().map(PitchClass::chroma))
            .finish()
    }
}

impl fmt::Debug for PitchClassSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct PcWithChroma(PitchClass);

        impl fmt::Debug for PcWithChroma {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?} ({})", self.0, self.0.chroma())
            }
        }

        f.debug_set()
            .entries(self.into_iter().map(PcWithChroma))
            .finish()
    }
}

impl fmt::Display for PitchClassSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(*self).finish()
    }
}

impl fmt::Binary for PitchClassSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0b")?;
        }

        write!(f, "{:012b}", self.0)
    }
}