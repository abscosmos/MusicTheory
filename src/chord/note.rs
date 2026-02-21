use std::cmp::Ordering;
use std::hash::Hash;
use rustc_hash::FxHashSet;
use strum::IntoEnumIterator;
use crate::chord::pitch::PitchChord;
use crate::{Interval, Letter, Note, Pitch};
use crate::chord::{root, ChordShape};
use crate::chord::letter_set::LetterSet;
use crate::set::PitchClassSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteChord {
    notes: Box<[Note]>,
    pitch_chord: PitchChord,
}

impl NoteChord {
    pub fn new(notes: impl IntoIterator<Item=Note>) -> Option<Self> {
        let notes = Self::sort_dedup(notes);

        if notes.is_empty() {
            return None;
        }
        
        let root = root::find_root(notes.iter().map(|n| n.pitch))
            .expect("not empty");

        let pitch_chord = Self::make_pitch_chord_inner(&notes, root);

        Some(Self { notes, pitch_chord })
    }

    pub fn with_root(notes: impl IntoIterator<Item=Note>, root: Pitch) -> Option<Self> {
        let notes = Self::sort_dedup(notes);

        if notes.iter().any(|n| n.pitch == root) {
            let pitch_chord = Self::make_pitch_chord_inner(&notes, root);

            Some(Self { notes, pitch_chord })
        } else {
            None
        }
    }

    fn sort_dedup(notes: impl IntoIterator<Item=Note>) -> Box<[Note]> {
        let mut notes = notes.into_iter().collect::<Vec<_>>();

        notes.sort();
        notes.dedup();

        notes.into_boxed_slice()
    }

    pub fn from_pitch_chord(chord: &PitchChord, bass_octave: i16) -> Self {
        Self::with_root(chord.notes(bass_octave), chord.root()).expect("can't be empty")
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    pub fn root(&self) -> Pitch {
        self.pitch_chord.root()
    }

    pub fn bass(&self) -> Note {
        *self.notes.first().expect("shouldn't be empty")
    }

    pub fn pitch_class_set(&self) -> PitchClassSet {
        self.notes.iter()
            .map(|n| n.pitch.as_pitch_class())
            .collect()
    }

    pub fn contains_pitch(&self, pitch: Pitch) -> bool {
        self.notes.iter().any(|n| n.pitch == pitch)
    }

    pub fn contains_note(&self, note: Note) -> bool {
        self.notes.contains(&note)
    }

    #[inline]
    pub fn dedup_by_pitch(&self) -> Self {
        Self {
            notes: dedup_by(self.notes.iter().copied(), |n| n.pitch),
            // removing duplicate pitches does not affect the underlying pitch chord
            pitch_chord: self.pitch_chord.clone(),
        }
    }

    // if not deduped by pitch, and root and
    // also, need method to undo this? after that this method can be public
    // check what the current n is, and go off of that? like find how much to rotate by
    fn with_inversion(&self, n: usize, separate_steps: bool) -> Option<Self> {
        if n == 0 {
            return Some(self.clone())
        }

        if n >= self.len() {
            return None;
        }

        let mut notes = self.notes.clone();

        notes.rotate_left(n);

        let wrap_start = notes.len() - n;

        // bump wrapped notes up one octave
        let bass = notes[0];

        for note in &mut notes[wrap_start..] {
            note.octave += 1;

            if *note < bass {
                note.octave += 1;

                debug_assert!(
                    *note >= bass,
                    "note should now be in correct place",
                );
            }
        }

        notes.sort_unstable();

        // fix multiple notes with same letter at same octave
        if separate_steps {
            for letter in Letter::iter() {
                let mut pitches = notes.iter_mut()
                    .filter(|n| n.pitch.letter() == letter);

                if let Some(lowest) = pitches.next().copied() {
                    for (i, note) in pitches.enumerate() {
                        note.octave = lowest.octave + i as i16 + 1;
                    }
                }
            }

            notes.sort_unstable();
        }

        debug_assert!(
            notes.windows(2).all(|w| w[0] < w[1]),
            "should be sorted & deduplicated after inversion",
        );

        let pitch_chord = PitchChord::with_bass(
            self.root(),
            self.pitch_chord.shape().clone(),
            bass.pitch,
        );

        Some(Self { notes, pitch_chord })
    }

    fn closed_root_position_inner(root: Pitch, notes: &[Note]) -> Box<[Note]> {
        let mut notes = dedup_by::<_, _, Vec<_>>(notes.iter().copied(), |n| n.pitch);

        let mut closed = Vec::with_capacity(notes.len());

        let mut letters_added = LetterSet::EMPTY;

        // 1. add root(s)
        let root_note = {
            let root_pos = notes.iter()
                .position(|n| n.pitch == root)
                .expect("root should exist in chord");

            notes.remove(root_pos)
        };

        closed.push(root_note);
        letters_added = letters_added.with_set(root_note.pitch.letter());

        closed.extend(
            notes
                .extract_if(.., |n| n.pitch.letter() == root.letter())
                .map(|n| Note::new(n.pitch, root_note.octave + 1))
        );

        // 2. find thirds
        let mut last_letter = root_note.pitch.letter();
        let mut last_octave = root_note.octave;

        loop {
            if letters_added == LetterSet::FULL {
                break;
            }

            let third_letter = Letter::from_step((last_letter.step() + 2) % 7)
                .expect("should be valid letter");

            let octave = match last_letter.cmp(&third_letter) {
                Ordering::Less => last_octave,
                Ordering::Greater => last_octave + 1,
                Ordering::Equal => unreachable!("letter should be two steps away"),
            };

            let added_any = {
                let before_size = closed.len();

                closed.extend(
                    notes
                        .extract_if(.., |n| n.pitch.letter() == third_letter)
                        .map(|n| Note::new(n.pitch, octave))
                );

                closed.len() != before_size
            };

            if added_any {
                letters_added = letters_added.with_set(third_letter);
                last_letter = third_letter;
                last_octave = octave;
            } else {
                break;
            }
        }

        // 3. insert leftovers

        // to cluster notes with the same letter, sort them by letter
        // which should also preserve ordering of accidentals
        debug_assert!(
            notes.is_sorted(),
            "notes should be sorted (by octave)"
        );

        let (root_letter, root_octave) = {
            let bass = *closed.first().expect("must have at least one note");

            assert_eq!(
                bass.pitch, root,
                "before reordering, the bass should be the root",
            );

            (bass.pitch.letter(), bass.octave)
        };

        notes.sort_by_key(|n| root_letter.offset_between(n.pitch.letter()));

        for notes in notes.chunk_by(|a, b| a.pitch.letter() == b.pitch.letter()) {
            let letter = notes.first()
                .expect("must be first item")
                .pitch
                .letter();

            let octave = match root_letter.cmp(&letter) {
                Ordering::Less => root_octave,
                Ordering::Greater => root_octave + 1,
                Ordering::Equal => unreachable!("letter should be root"),
            };

            closed.extend(
                notes.iter().map(|n| Note::new(n.pitch, octave))
            );
        }

        closed.into_boxed_slice()
    }

    fn separate_same_letter_by_octave(notes: &mut [Note]) {
        for letter in Letter::iter() {
            let mut iter = notes
                .iter_mut()
                .filter(|n| n.pitch.letter() == letter);

            let Some(first) = iter.next().copied() else {
                continue;
            };

            for (i, note) in iter.enumerate() {
                note.octave = first.octave + i as i16 + 1;
            }
        }

        notes.sort_unstable();

        assert!(
            notes.windows(2).all(|w| w[0] < w[1]),
            "should be sorted & deduplicated",
        );
    }

    fn collapse_same_letter(notes: &mut [Note]) {
        for letter in Letter::iter() {
            let mut iter = notes
                .iter_mut()
                .filter(|n| n.pitch.letter() == letter);

            let Some(first) = iter.next().copied() else {
                continue;
            };

            for note in iter {
                note.octave = first.octave;
            }
        }

        notes.sort_unstable();

        assert!(
            notes.windows(2).all(|w| w[0] < w[1]),
            "should be sorted & deduplicated",
        );
    }

    fn make_pitch_chord_inner(notes: &[Note], root: Pitch) -> PitchChord {
        assert!(
            !notes.is_empty(),
            "notes can't be empty!"
        );

        debug_assert!(
            notes.iter().any(|n| n.pitch == root),
            "should contain at least one pitch that's root"
        );

        let mut closed = Self::closed_root_position_inner(root, notes);
        Self::separate_same_letter_by_octave(&mut closed);

        let root_note = closed[0];

        assert_eq!(
            root, closed[0].pitch,
            "root should have correct pitch",
        );

        let ivls = closed.into_iter()
            .map(|n| root_note.distance_to(n));

        let shape = ChordShape::from_intervals(ivls)
            .expect("should be sorted, deduped, and start with P1");

        PitchChord::with_bass(root, shape, notes[0].pitch)
    }

    pub fn closed_position(&self, separate_steps: bool) -> Self {
        // TODO: there are unnecessary clones since 'with_inversion' requires a NoteChord

        // TODO: restore this optimization (uses cached shape, avoids recomputing root position)
        // once separate_steps=false is properly handled — currently the shape is always
        // computed with separate_steps=true, so it diverges when separate_steps=false
        // and there are same-letter conflicts.
        //
        // let root = *self.notes.iter()
        //     .find(|n| n.pitch == self.root())
        //     .expect("root should exist in chord");
        // let notes = self.pitch_chord.shape().intervals().iter()
        //     .map(|ivl| root + *ivl)
        //     .collect::<Box<[_]>>();
        // debug_assert_eq!(
        //     notes,
        //     Self::closed_root_position_inner(self.root(), &self.notes, true),
        // );
        // Self { notes, pitch_chord: self.pitch_chord.clone() }
        let closed_root = {
            let mut notes = Self::closed_root_position_inner(self.root(), &self.notes);

            if separate_steps {
                Self::separate_same_letter_by_octave(&mut notes);
            } else {
                notes.sort_unstable();
            }

            let pitch_chord = Self::make_pitch_chord_inner(&notes, self.root());
            Self { notes, pitch_chord }
        };

        if self.root() == self.bass().pitch {
            return closed_root;
        }

        // ensure state is right before reordering
        if cfg!(debug_assertions) {
            let bass_letter = self.bass().pitch.letter();

            let first_bass = self.notes.iter().find(|n| n.pitch.letter() == bass_letter)
                .expect("bass letter should exist in chord");

            assert_eq!(
                first_bass.pitch, self.bass().pitch,
                "before reordering for inversion, make sure the right note is in the bass"
            )
        }

        let bass = self.bass();
        let bass_idx = closed_root.notes.iter()
            .position(|n| n.pitch == bass.pitch)
            .expect("bass pitch must be in closed voicing");

        assert_ne!(
            bass_idx, 0,
            "bass shouldn't be root",
        );

        let mut with_inversion = closed_root.with_inversion(bass_idx, separate_steps).expect("valid inversion");

        let octave_diff = self.bass().octave - with_inversion.bass().octave;

        for note in &mut with_inversion.notes {
            note.octave += octave_diff;
        }

        assert_eq!(
            with_inversion.bass(), self.bass(),
            "bass note should match after octave adjustment",
        );

        with_inversion
    }

    pub fn compact_position(&self, separate_steps: bool) -> Self {
        let mut deduped = self.dedup_by_pitch();

        let bass_letter = deduped.bass().pitch.letter();
        let bass_octave = deduped.bass().octave;

        for letter in Letter::iter() {
            for (i, note) in deduped.notes.iter_mut()
                .filter(|n| n.pitch.letter() == letter)
                .enumerate()
            {
                let octave = match bass_letter.cmp(&letter) {
                    Ordering::Less | Ordering::Equal => bass_octave,
                    Ordering::Greater => bass_octave + 1,
                };

                if separate_steps {
                    note.octave = octave + i as i16;
                } else {
                    note.octave = octave;
                }
            }
        }

        deduped.notes.sort_unstable();

        deduped
    }

    pub fn transpose(&self, interval: Interval) -> Self {
        let notes = self.notes.iter()
            .map(|&n| n + interval)
            .collect::<Box<[_]>>();

        debug_assert!(
            notes.is_sorted(),
            "notes should remain sorted after transpose",
        );

        Self { notes, pitch_chord: self.pitch_chord.transpose(interval) }
    }

    pub fn as_pitch_chord(&self) -> &PitchChord {
        &self.pitch_chord
    }

    pub fn inversion(&self) -> u8 {
        self.as_pitch_chord().inversion()
    }
}

fn dedup_by<T, K: Eq + Hash, C: FromIterator<T>>(collection: impl IntoIterator<Item=T>, mut key: impl FnMut(&T) -> K) -> C {
    let mut seen = FxHashSet::default();

    collection.into_iter()
        .filter(|elem| seen.insert(key(elem)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_chord(chord: &str) -> NoteChord {
        use std::str::FromStr;

        let notes = chord.split(' ')
            .map(|note| Note::from_str(note).expect("valid note str"));

        NoteChord::new(notes).expect("valid chord")
    }

    fn test_by(cases: &[(&str, &str)], test: impl Fn(NoteChord) -> Box<[Note]>) {
        for (input_str, closed_str) in cases {
            let input = parse_chord(input_str);
            let exp = parse_chord(closed_str);

            assert_eq!(
                test(input).as_ref(), exp.notes(),
                "failed for input {input_str}, expected: {closed_str}",
            );
        }
    }

    #[test]
    fn closed_position_separate_steps() {
        let cases = [
            ("C4 E4 G4", "C4 E4 G4"),
            ("C3 G4 E6", "C3 E3 G3"),
            ("C4 E4 G4 C5", "C4 E4 G4"),
            ("C4 D4 E4 G4 Bb4", "C4 E4 G4 Bb4 D5"),
            ("E3 G3 C4", "E3 G3 C4"),
            ("C4 E5 G5 Bb5 D6", "C4 E4 G4 Bb4 D5"),
            ("E2 Bb3 D4 G4 C5", "E2 G2 Bb2 C3 D3"),
            ("G4 C5 E6 Bb6 D7", "G4 Bb4 C5 D5 E5"),
            ("Bb4 C6 E6 D7 G7", "Bb4 C5 D5 E5 G5"),
            ("D4 E5 Bb5 G6 C7", "D4 E4 G4 Bb4 C5"),
            ("C4 E4 G4 Bb4 Eb5", "C4 E4 G4 Bb4 Eb5"),
            ("C4 Eb4 G4 E5", "C4 Eb4 G4 E5"),
            ("G3 C4 Eb4 E4 Bb4", "G3 Bb3 C4 Eb4 E5"),
        ];

        test_by(&cases, |nc| nc.closed_position(true).notes);
    }

    #[test]
    fn closed_position_no_separate_steps() {
        let cases = [
            ("C4 E4 G4 Bb4 Eb5", "C4 Eb4 E4 G4 Bb4"),
        ];

        test_by(&cases, |nc| nc.closed_position(false).notes);
    }

    #[test]
    fn closed_root_position() {
        let cases = [
            ("E3 G3 C4", "C4 E4 G4"),
        ];

        test_by(&cases, |nc| {
            let mut notes = NoteChord::closed_root_position_inner(nc.root(), nc.notes());
            notes.sort_unstable();

            notes
        });
    }
}