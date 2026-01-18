use crate::scales::definition::heptatonic::DiatonicMode as DiatonicModeExperimental;
#[allow(unused_imports, reason = "used in documentation")]
use crate::harmony::ScaleDegree;

/// Diatonic modes, also known as the modes of the major scale.
///
/// Most commonly used are [`Ionian`](Self::Ionian) ([major](Self::MAJOR)) and
/// [`Aeolian`](Self::Aeolian) ([natural minor](Self::NATURAL_MINOR)).
///
/// Copy of [implementation in experimental scales module][exp].
/// Intended to be used until a stable version on scales is released.
///
/// [exp]: crate::scales::definition::heptatonic::DiatonicMode
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DiatonicMode {
    #[default]
    /// Ionian, the mode of the major scale.
    Ionian = 1,
    /// Dorian, a minor mode with raised [6th](ScaleDegree::VI).
    Dorian,
    /// Phrygian, a minor mode with a flattened [2nd](ScaleDegree::II).
    Phrygian,
    /// Lydian, a major mode with a raised [4th](ScaleDegree::IV).
    Lydian,
    /// Mixolydian, a major mode with a flattened [7th](ScaleDegree::VII).
    Mixolydian,
    /// Aeolian, the mode of the natural minor scale.
    Aeolian,
    /// Locrian, a diminished mode with a flattened [2nd](ScaleDegree::II), [3rd](ScaleDegree::III),
    /// [5th](ScaleDegree::V), [6th](ScaleDegree::VI) and [7th](ScaleDegree::VII).
    Locrian,
}

impl DiatonicMode {
    /// Mode of the major scale, also known as [`Ionian`](Self::Ionian).
    pub const MAJOR: Self = Self::Ionian;
    /// Mode of the natural minor scale, also known as [`Aeolian`](Self::Aeolian).
    pub const NATURAL_MINOR: Self = Self::Aeolian;

    pub(crate) fn as_experimental(self) -> DiatonicModeExperimental {
        DiatonicModeExperimental::from_repr(self as _).expect("implementation should be exact copy")
    }

    pub(crate) fn from_experimental(inner: DiatonicModeExperimental) -> Self {
        Self::from_repr(inner as _).expect("implementation should be exact copy")
    }
}