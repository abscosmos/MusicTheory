use std::ops::RangeInclusive;
use crate::key::Key;
use crate::note::Note;
use crate::pitch::Pitch;
use crate::voice_leading::check::{check_voice_leading, score_single, score_window};
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};

pub fn brute_force_search(
    progression: &[RomanChord],
    starting_voicing: Option<Voicing>,
    key: Key,
) -> Vec<(u16, Vec<Voicing>)> {
    if progression.len() == 0 {
        return vec![];
    }

    let mut all_voicings: Vec<Vec<Voicing>> = progression
        .iter()
        .map(|chord| generate_voicings_for_chord(*chord, key))
        .collect();

    if let Some(start) = starting_voicing {
        all_voicings[0] = vec![start];
    }

    let total_permutations: usize = all_voicings
        .iter()
        .map(|v| v.len())
        .product();

    dbg!(total_permutations);

    if total_permutations == 0 || all_voicings.is_empty() {
        return vec![];
    }

    let mut results: Vec<Vec<Voicing>> = vec![vec![]];

    for chord_voicings in all_voicings {
        let mut new_results = Vec::with_capacity(results.len() * chord_voicings.len());

        for seq in &results {
            for voicing in &chord_voicings {
                let mut new_seq = seq.clone();
                new_seq.push(*voicing);
                new_results.push(new_seq);
            }
        }

        results = new_results;
    }

    assert_eq!(
        results.len(), total_permutations,
        "should've generated all, exactly"
    );

    let mut results = results.into_iter()
        .filter_map(|v| match check_voice_leading(key, progression, &v) {
            Ok(score) => Some((score, v)),
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    results.sort_by_key(|(score, _)| *score);

    results
}

pub fn generate_voice_leadings(
    progression: &[RomanChord],
    key: Key,
    starting_voicing: Option<Voicing>,
) -> Vec<(u16, Vec<Voicing>)> {
    if progression.is_empty() {
        return vec![];
    }

    let mut results = Vec::new();

    let first_chord = progression[0];
    let first_voicings = if let Some(start) = starting_voicing {
        vec![start]
    } else {
        generate_voicings_for_chord(first_chord, key)
    };

    for first_voicing in first_voicings {
        let first_score = match score_single(first_voicing, first_chord, key) {
            Ok(score) => score,
            Err(_) => continue,
        };

        let mut current_solution = vec![first_voicing];
        backtrack(
            &mut current_solution,
            first_score,
            progression,
            key,
            1,
            &mut results,
        );
    }

    results.sort_by_key(|(score, _)| *score);

    results
}

fn backtrack(
    current_solution: &mut Vec<Voicing>,
    current_score: u16,
    progression: &[RomanChord],
    key: Key,
    chord_index: usize,
    results: &mut Vec<(u16, Vec<Voicing>)>,
) {
    if chord_index >= progression.len() {
        results.push((current_score, current_solution.clone()));
        return;
    }

    let current_chord = progression[chord_index];
    let previous_voicing = *current_solution.last().unwrap();
    let previous_chord = progression[chord_index - 1];

    let candidate_voicings = generate_voicings_for_chord(current_chord, key);

    for voicing in candidate_voicings {
        let voicing_score = score_single(voicing, current_chord, key).expect("already checked!");

        let window_score = match score_window(previous_voicing, voicing, previous_chord, current_chord, key) {
            Ok(score) => score,
            Err(_) => continue,
        };

        let new_score = current_score + voicing_score + window_score;

        current_solution.push(voicing);

        backtrack(current_solution, new_score, progression, key, chord_index + 1, results);

        current_solution.pop();
    }
}

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