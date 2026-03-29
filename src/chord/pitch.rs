use std::iter;
use crate::chord::ChordShape;
use crate::{EnharmonicEq, Interval, Note, Pitch};
use crate::chord::known::KnownChord;
use crate::interval::Number;
use crate::harmony::Key;
use crate::pitch::Spelling;
use crate::set::{IntervalClassVector, PitchClassSet, SetClass, SetClassForm};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PitchChord {
    // TODO: should the root & shape fields be public? What the invariant that must be held?
    root: Pitch,
    shape: ChordShape,
    pub(super) bass: Pitch,
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

    pub fn in_inversion(&self) -> bool {
        self.inversion() != 0
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

    pub fn is_dominant_seventh(&self) -> bool {
        self.shape.is_dominant_seventh()
    }

    pub fn is_half_diminished(&self) -> bool {
        self.shape.is_half_diminished()
    }

    pub fn is_diminished_seventh(&self) -> bool {
        self.shape.is_diminished_seventh()
    }

    pub fn is_consonant(&self) -> bool {
        self.shape.is_consonant()
    }

    pub fn has_repeated_chord_step(&self, number: Number) -> bool {
        self.shape.has_repeated_chord_step(number)
    }

    pub fn has_any_repeated_diatonic_note(&self) -> bool {
        self.shape.has_any_repeated_diatonic_note()
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

    pub fn pitch_class_set(&self) -> PitchClassSet {
        self.pitches_naive().map(Pitch::as_pitch_class).collect()
    }

    pub fn interval_class_vector(&self) -> IntervalClassVector {
        self.pitch_class_set().interval_class_vector()
    }

    pub fn intervals_from_root(&self) -> Vec<Interval> {
        if self.root == self.bass {
            return self.shape.intervals().to_vec();
        }

        let mut intervals = self.shape.intervals().to_vec();

        if let Some(bass_idx) = intervals.iter().position(|&ivl| self.root + ivl == self.bass) {
            // inversion: all chord tones at index >= bass_idx are voiced below root
            for ivl in &mut intervals[bass_idx..] {
                let pitch = self.root + *ivl;
                *ivl = -pitch.distance_to(self.root);
            }
        } else {
            // slash chord: bass is not a chord tone, append its descending interval
            intervals.push(-self.bass.distance_to(self.root));
        }

        intervals.sort();
        intervals
    }

    pub fn known(&self) -> Option<KnownChord> {
        self.shape.known()
    }

    pub fn set_class(&self) -> SetClass {
        self.pitch_class_set().set_class()
    }

    pub fn set_class_form(&self) -> SetClassForm {
        self.pitch_class_set().set_class_form()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::known::KnownChord;
    use Interval as I;

    fn major_triad_root() -> PitchChord {
        PitchChord::new(Pitch::C, KnownChord::MajorTriad.shape())
    }

    fn major_triad_first_inv() -> PitchChord {
        PitchChord::with_inversion(Pitch::C, KnownChord::MajorTriad.shape(), 1).unwrap()
    }

    fn major_triad_second_inv() -> PitchChord {
        PitchChord::with_inversion(Pitch::C, KnownChord::MajorTriad.shape(), 2).unwrap()
    }

    fn major_seventh_first_inv() -> PitchChord {
        PitchChord::with_inversion(Pitch::C, KnownChord::MajorSeventh.shape(), 1).unwrap()
    }

    fn slash_chord() -> PitchChord {
        // Cmaj / F# — F# is not a chord tone
        PitchChord::with_bass(Pitch::C, KnownChord::MajorTriad.shape(), Pitch::F_SHARP)
    }

    #[test]
    fn root_position_unchanged() {
        assert_eq!(
            major_triad_root().intervals_from_root(),
            vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH],
        );
    }

    #[test]
    fn first_inversion_bass_negative() {
        // C/E voicing E–G–C: E and G are both below root C
        // E→C = m6, G→C = P4
        assert_eq!(
            major_triad_first_inv().intervals_from_root(),
            vec![-I::MINOR_SIXTH, -I::PERFECT_FOURTH, I::PERFECT_UNISON],
        );
    }

    #[test]
    fn second_inversion_bass_negative() {
        // C/G: G voiced below root C → descending P4 (G up to C = P4); E remains M3
        assert_eq!(
            major_triad_second_inv().intervals_from_root(),
            vec![-I::PERFECT_FOURTH, I::PERFECT_UNISON, I::MAJOR_THIRD],
        );
    }

    #[test]
    fn seventh_first_inversion() {
        // Cmaj7/E voicing E–G–B–C: E, G, B all below root C
        // E→C = m6, G→C = P4, B→C = m2
        assert_eq!(
            major_seventh_first_inv().intervals_from_root(),
            vec![-I::MINOR_SIXTH, -I::PERFECT_FOURTH, -I::MINOR_SECOND, I::PERFECT_UNISON],
        );
    }

    #[test]
    fn seventh_second_inversion() {
        // Cmaj7/G voicing G–B–C–E: G and B below root C, E above
        // G→C = P4, B→C = m2
        let chord = PitchChord::with_inversion(Pitch::C, KnownChord::MajorSeventh.shape(), 2).unwrap();
        assert_eq!(
            chord.intervals_from_root(),
            vec![-I::PERFECT_FOURTH, -I::MINOR_SECOND, I::PERFECT_UNISON, I::MAJOR_THIRD],
        );
    }

    #[test]
    fn intervals_from_root_consistent_with_pitches() {
        let cases = [
            major_triad_root(),
            major_triad_first_inv(),
            major_triad_second_inv(),
            major_seventh_first_inv(),
            slash_chord(),
        ];

        for chord in &cases {
            let pitches: std::collections::HashSet<Pitch> = chord.pitches().into_iter().collect();
            for ivl in chord.intervals_from_root() {
                let computed = chord.root() + ivl;
                assert!(
                    pitches.contains(&computed),
                    "{chord:?}: root + {ivl:?} = {computed:?}, not in pitches {pitches:?}",
                );
            }
        }
    }

    #[test]
    fn slash_chord_bass_not_chord_tone() {
        // Cmaj/F#: F# is not in the triad, voiced below root → descending d5 (F# up to C = d5)
        assert_eq!(
            slash_chord().intervals_from_root(),
            vec![-I::DIMINISHED_FIFTH, I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH],
        );
    }
}

impl EnharmonicEq for PitchChord {
    fn eq_enharmonic(&self, other: &Self) -> bool {
        let Self { root: lhs_root, shape: lhs_shape, bass: lhs_bass } = self;
        let Self { root: rhs_root, shape: rhs_shape, bass: rhs_bass } = other;

        lhs_root.eq_enharmonic(rhs_root)
            && lhs_bass.eq_enharmonic(rhs_bass)
            && lhs_shape.eq_enharmonic(rhs_shape)
    }
}