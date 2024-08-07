use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
pub enum ChordSize {
    Triad,
    Sixth,
    Seventh,
    SevenSharpEleventh,
}