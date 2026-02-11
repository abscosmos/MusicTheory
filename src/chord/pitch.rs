use crate::chord::ChordShape;
use crate::{Interval, Pitch};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PitchChord {
    // TODO: should the root & shape fields be public? What the invariant that must be held?
    root: Pitch,
    shape: ChordShape,
    bass: Option<Interval>,
}