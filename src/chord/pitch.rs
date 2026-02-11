use crate::chord::ChordShape;
use crate::{Interval, Pitch};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PitchChord {
    root: Pitch,
    shape: ChordShape,
    bass: Option<Interval>,
}