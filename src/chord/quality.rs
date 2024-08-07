use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
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