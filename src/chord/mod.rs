use crate::chord::types::ChordType;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::interval::Interval;
use crate::note::pitch::Pitch;
use crate::note::pitch_class::PitchClass;

pub mod quality;
pub mod size;
pub mod types;

#[derive(Clone, Debug, Eq)]
pub struct Chord {
    intervals: Vec<Interval>,
    ty: Option<ChordType>,
    root: Pitch,
    inversion: u8,
}

impl Chord {
    pub fn from_type(chord_type: ChordType, root: Pitch, inversion: u8) -> Option<Self>{
        Self::from_intervals_inner(Some(chord_type), chord_type.intervals(), root, inversion)
    }

    pub fn from_intervals(intervals: Vec<Interval>, root: Pitch, inversion: u8) -> Option<Self> {
        Self::from_intervals_inner(None, intervals, root, inversion)
    }

    fn from_intervals_inner(ty: Option<ChordType>, mut intervals: Vec<Interval>, root: Pitch, inversion: u8) -> Option<Self> {
        if inversion as usize >= intervals.len() {
            return None;
        }

        intervals.sort_by(|a, b| a.cmp_enharmonic(b));

        Some(
            Self {
                intervals,
                ty,
                root,
                inversion,
            }
        )
    }

    pub fn chord_type(&self) -> Option<ChordType> {
        self.ty
    }

    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    pub fn pitches(&self) -> Option<Vec<Pitch>> {
        let mut pitches = self.intervals
            .iter()
            .map(|ivl| self.root.apply_interval_ascending(ivl))
            .collect::<Option<Vec<_>>>();

        pitches.as_mut()
            .map(|v| v.rotate_left(self.inversion as _));

        pitches
    }
}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.intervals == other.intervals &&
            self.root == other.root &&
            self.inversion == other.inversion
    }
}

impl EnharmonicEq for Chord {
    // TODO: is there a more efficient way of doing this?
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        if self.intervals.len() != rhs.intervals().len() {
            return false;
        }

        let Some(lhs) = self.pitches() else {
            return false;
        };

        let Some(rhs) = rhs.pitches() else {
            return false;
        };

        let to_pc = |p: Pitch| p.as_pitch_class();

        let mut lhs = lhs.into_iter().map(to_pc).collect::<Vec<_>>();
        let mut rhs = rhs.into_iter().map(to_pc).collect::<Vec<_>>();

        let sort_pc = |a: &PitchClass, b: &PitchClass| (*a as u8).cmp(&(*b as u8));

        lhs.sort_by(sort_pc);
        rhs.sort_by(sort_pc);

        rhs == lhs
    }
}