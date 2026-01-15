use crate::scales::definition::heptatonic::DiatonicMode as DiatonicModeExperimental;

/// Copy of [implementation in experimental scales module][exp].
/// Intended to be used until a stable version on scales is released.
///
/// [exp]: crate::scales::definition::heptatonic::DiatonicMode
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DiatonicMode {
    #[default]
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

    pub(crate) fn as_experimental(self) -> DiatonicModeExperimental {
        DiatonicModeExperimental::from_repr(self as _).expect("implementation should be exact copy")
    }

    pub(crate) fn from_experimental(inner: DiatonicModeExperimental) -> Self {
        Self::from_repr(inner as _).expect("implementation should be exact copy")
    }
}