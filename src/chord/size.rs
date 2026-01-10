use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChordSize {
    Triad,
    Sixth,
    Seventh,
    SevenSharpEleventh,
}