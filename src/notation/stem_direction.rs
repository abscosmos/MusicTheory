#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StemDirection {
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GetStemDirectionParams {
    /// Only the first note and last note are considered (Default)
    #[default]
    EndsOnly,
    /// Only the note furthest above the middle line and furthest below the middle line are considered.
    ExtremesOnly,
    /// All notes are considered
    AllNotes,
}