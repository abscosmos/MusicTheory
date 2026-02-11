use crate::chord::ChordShape;
use crate::{Interval, Pitch};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PitchChord {
    // TODO: should the root & shape fields be public? What the invariant that must be held?
    root: Pitch,
    shape: ChordShape,
    bass: Option<Pitch>,
}

impl PitchChord {
    pub fn new(root: Pitch, shape: ChordShape) -> Self {
        Self::with_bass(root, shape, root)
    }

    pub fn with_inversion(root: Pitch, shape: ChordShape, inversion: u8) -> Option<Self> {
        let bass = todo!("compute from inversion");

        Some(Self::with_bass(root, shape, bass))
    }

    pub fn with_bass(root: Pitch, shape: ChordShape, bass: Pitch) -> Self {
        Self { root, shape, bass: Some(bass) }
    }

    pub fn root(&self) -> Pitch {
        self.root
    }

    pub fn shape(&self) -> &ChordShape {
        &self.shape
    }
}