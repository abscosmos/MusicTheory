use crate::chord::types::ChordType;
use crate::enharmonic::EnharmonicOrd;
use crate::interval::Interval;
use crate::pitch::Pitch;

pub mod quality;
pub mod size;
pub mod types;
mod eq;

#[derive(Debug, thiserror::Error)]
#[error("Can't have a {attempted}th inversion on a chord with only {intervals} note(s).")]
pub struct InvalidInversion {
    pub intervals: u8,
    pub attempted: u8,
}

#[derive(Clone, Debug, Eq)]
pub struct Chord {
    pub root: Pitch,
    intervals: Vec<Interval>,
    ty: Option<ChordType>,
    inversion: u8,
}

impl Chord {
    pub fn from_type(chord_type: ChordType, root: Pitch, inversion: u8) -> Result<Self, InvalidInversion> {
        Self::from_intervals_inner(Some(chord_type), chord_type.intervals(), root, inversion)
    }

    pub fn from_intervals(intervals: Vec<Interval>, root: Pitch, inversion: u8) -> Result<Self, InvalidInversion> {
        Self::from_intervals_inner(None, intervals, root, inversion)
    }

    fn from_intervals_inner(ty: Option<ChordType>, mut intervals: Vec<Interval>, root: Pitch, inversion: u8) -> Result<Self, InvalidInversion> {
        if inversion as usize >= intervals.len() {
            return Err(InvalidInversion { intervals: intervals.len() as _, attempted: inversion });
        }

        intervals.sort_by(Interval::cmp_enharmonic);

        Ok(
            Self {
                intervals,
                ty,
                root,
                inversion,
            }
        )
    }

    pub fn inversion(&self) -> u8 {
        self.inversion
    }

    pub fn set_inversion(&mut self, inversion: u8) -> Result<(), InvalidInversion> {
        if inversion as usize >= self.intervals.len() {
            Err(InvalidInversion { intervals: self.intervals.len() as _, attempted: inversion })
        } else {
            self.inversion = inversion as _;
            Ok(())
        }
    }

    pub fn chord_type(&self) -> Option<ChordType> {
        self.ty
    }

    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    pub fn successive_intervals(&self) -> Vec<Interval> {
        if self.inversion != 0 {
            todo!("Chord::successive_intervals doesn't work with inversions yet");
        }

        self.intervals.windows(2)
            .map(|window| {
                let [a, b] = window else {
                    unreachable!("set to windows of two");
                };

                *b - *a
            })
            .collect()
    }

    #[inline]
    fn pitches_iter(&self) -> impl Iterator<Item=Pitch> {
        self.intervals.iter()
            .map(|&ivl| self.root.transpose(ivl))
            .cycle()
            .skip(self.inversion as _)
            .take(self.intervals.len())
    }

    pub fn pitches(&self) -> Vec<Pitch> {
        self.pitches_iter().collect()
    }
}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.intervals == other.intervals &&
            self.root == other.root &&
            self.inversion == other.inversion
    }
}