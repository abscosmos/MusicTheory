use std::ops::RangeInclusive;
use crate::key::Key;
use crate::note::Note;
use crate::pitch::Pitch;
use crate::voice_leading::check::score_single;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};

fn generate_voicings_for_chord(chord: RomanChord, key: Key) -> Vec<Voicing> {
    let pitches = chord.pitches(key);

    let soprano_notes = generate_notes_in_range(&pitches, Voice::Soprano.range());

    let alto_notes = generate_notes_in_range(&pitches, Voice::Alto.range());

    let tenor_notes = generate_notes_in_range(&pitches, Voice::Tenor.range());

    let bass_notes = generate_notes_in_range(&pitches, Voice::Bass.range());

    let mut voicings = Vec::new();

    for &s in &soprano_notes {
        for &a in &alto_notes {
            for &t in &tenor_notes {
                for &b in &bass_notes {
                    let voicing = Voicing::new([s, a, t, b]);

                    if score_single(voicing, chord, key).is_ok() {
                        voicings.push(voicing);
                    }

                }
            }
        }
    }

    voicings
}

fn generate_notes_in_range(pitches: &[Pitch], range: RangeInclusive<Note>) -> Vec<Note> {
    let mut notes = Vec::new();

    let (min, max) = (range.start(), range.end());

    for &pitch in pitches {
        for octave in (min.octave - 1)..=(max.octave + 1) {
            let note = Note::new(pitch, octave);

            if range.contains(&note) {
                notes.push(note);
            }
        }
    }

    notes
}