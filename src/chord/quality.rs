use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChordQuality {
    Major,
    Minor,
    MinorFlat,
    MinorMajor,
    Diminished,
    Augmented,
    AugmentedMajor,
    HalfDiminished,
    Dominant,
    Suspended2,
    Suspended4,
}