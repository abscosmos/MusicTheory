use crate::chord::interval_set::IntervalSet;
use crate::interval::Interval;
use crate::note::Note;

mod interval_set;

#[derive(Clone, Debug)]
pub struct Chord {
    // TODO: should root be a note or pitch; do we want a Placed<Chord>?
    root: Note,
    intervals: IntervalSet,
}

impl Chord {
    pub fn new(root: Note, intervals: impl IntoIterator<Item = Interval>) -> Option<Self> {
        // TODO: sort intervals first

        let intervals = intervals.into_iter().collect::<IntervalSet>();

        if intervals.is_empty() {
            return None;
        }

        Some(Self { root, intervals })
    }

    pub fn root(&self) -> Note {
        self.root
    }

    pub fn len(&self) -> usize {
        self.notes().len()
    }

    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    pub fn bass(&self) -> Note {
        let root_ivl = self.intervals.first().expect("chords must have at least one note");

        self.root.transpose(root_ivl)
    }

    pub fn notes(&self) -> Vec<Note> {
        self.intervals.iter()
            .map(|ivl| self.root.transpose(ivl))
            .collect()
    }
}

pub use ivl_sets::*;

// TODO: will be removed in the future once it's easier to build chords
pub mod ivl_sets {
    use crate::chord::interval_set::IntervalSet;
    use crate::interval::Interval as I;

    pub fn major_triad() -> IntervalSet {
        vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH]
    }

    pub fn maj_first_inversion() -> IntervalSet {
        vec![-I::PERFECT_FOURTH, I::PERFECT_UNISON, I::MAJOR_THIRD]
    }
}