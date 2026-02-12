use std::iter;
use crate::chord::ChordShape;
use crate::{Interval, Note, Pitch};
use crate::interval::Number;
use crate::harmony::Key;
use crate::pitch::Spelling;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PitchChord {
    // TODO: should the root & shape fields be public? What the invariant that must be held?
    root: Pitch,
    shape: ChordShape,
    bass: Pitch,
}

impl PitchChord {
    pub fn new(root: Pitch, shape: ChordShape) -> Self {
        Self::with_bass(root, shape, root)
    }

    pub fn with_inversion(root: Pitch, shape: ChordShape, inversion: u8) -> Option<Self> {
        let bass_interval = *shape.intervals().get(inversion as usize)?;
        let bass = root + bass_interval;

        Some(Self::with_bass(root, shape, bass))
    }

    pub fn with_bass(root: Pitch, shape: ChordShape, bass: Pitch) -> Self {
        Self { root, shape, bass }
    }

    pub fn root(&self) -> Pitch {
        self.root
    }

    pub fn shape(&self) -> &ChordShape {
        &self.shape
    }

    pub fn bass(&self) -> Pitch {
        self.bass
    }

    pub fn pitches(&self) -> Vec<Pitch> {
        let mut chord_tones: Vec<Pitch> = self.shape.intervals().iter()
            .map(|&ivl| self.root + ivl)
            .collect();

        if let Some(bass_idx) = chord_tones.iter().position(|&p| p == self.bass) {
            chord_tones.rotate_left(bass_idx);
        } else {
            chord_tones.insert(0, self.bass);
        }
        
        chord_tones
    }

    pub fn notes(&self, bass_octave: i16) -> Vec<Note> {
        self.pitches().into_iter()
            .scan((bass_octave, None), |(octave, prev_note), pitch| {
                let mut note = Note::new(pitch, *octave);

                if let Some(prev) = prev_note && *prev >= note {
                    *octave += 1;
                    note.octave = *octave;
                }

                *prev_note = Some(note);
                Some(note)
            })
            .collect()
    }

    // TODO: should this take ownership?
    pub fn transpose(&self, interval: Interval) -> Self {
        Self {
            root: self.root + interval,
            shape: self.shape.clone(),
            bass: self.bass + interval,
        }
    }

    pub fn respell_with(&self, spelling: Spelling) -> Self {
        let root = self.root.respell_with(spelling);

        let transposition = self.root.distance_to(root);

        self.transpose(transposition)
    }

    // TODO: might this method be confused with other methods?
    pub fn respell_in_key(&self, key: Key) -> Self {
        let root = self.root.respell_in_key(key);

        let transposition = self.root.distance_to(root);

        self.transpose(transposition)
    }

    pub fn inversion(&self) -> u8 {
        self.shape.intervals().iter()
            .position(|&ivl| self.root + ivl == self.bass)
            .unwrap_or(0) as _
    }

    pub fn is_slash(&self) -> bool {
        self.shape.intervals().iter().all(|&ivl| self.root + ivl != self.bass)
    }

    pub fn len(&self) -> usize {
        self.shape.len() + self.is_slash() as usize
    }

    pub fn pitch_at(&self, degree: Number) -> Option<Pitch> {
        self.shape.interval_of(degree).map(|ivl| self.root + ivl)
    }

    pub fn third(&self) -> Option<Pitch> {
        self.pitch_at(Number::THIRD)
    }

    pub fn fifth(&self) -> Option<Pitch> {
        self.pitch_at(Number::FIFTH)
    }

    pub fn seventh(&self) -> Option<Pitch> {
        self.pitch_at(Number::SEVENTH)
    }

    pub fn ninth(&self) -> Option<Pitch> {
        self.pitch_at(Number::NINTH)
    }

    pub fn eleventh(&self) -> Option<Pitch> {
        self.pitch_at(Number::ELEVENTH)
    }

    pub fn thirteenth(&self) -> Option<Pitch> {
        self.pitch_at(Number::THIRTEENTH)
    }

    pub fn is_triad(&self) -> bool {
        self.shape.is_triad()
    }

    pub fn is_seventh(&self) -> bool {
        self.shape.is_seventh()
    }

    pub fn is_extended(&self) -> bool {
        self.shape.is_extended()
    }

    pub fn contains_triad(&self) -> bool {
        self.shape.contains_triad()
    }

    pub fn is_major(&self) -> bool {
        self.shape.is_major()
    }

    pub fn is_minor(&self) -> bool {
        self.shape.is_minor()
    }

    pub fn is_diminished(&self) -> bool {
        self.shape.is_diminished()
    }

    pub fn is_augmented(&self) -> bool {
        self.shape.is_augmented()
    }

    pub fn is_suspended(&self) -> bool {
        self.shape.is_suspended()
    }

    pub fn is_dominant(&self) -> bool {
        self.shape.is_dominant()
    }

    pub fn is_consonant(&self) -> bool {
        self.shape.is_consonant()
    }

    fn pitches_naive(&self) -> impl Iterator<Item=Pitch> + Clone {
        self.shape.intervals()
            .iter()
            .map(|&ivl| self.root + ivl)
            .chain(iter::once(self.bass))
    }

    pub fn contains(&self, pitch: Pitch) -> bool {
        self.pitches_naive().any(|p| p == pitch)
    }
}