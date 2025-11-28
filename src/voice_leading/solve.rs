use std::ops::RangeInclusive;
use crate::key::Key;
use crate::note::Note;
use crate::pitch::Pitch;
use crate::voice_leading::check::{check_voice_leading, score_single, score_window};
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};

#[doc(hidden)]
pub fn brute_force_search(
    progression: &[RomanChord],
    key: Key,
    starting_voicing: Option<Voicing>,
) -> Vec<(u16, Vec<Voicing>)> {
    if progression.len() == 0 {
        return vec![];
    }

    let mut all_voicings: Vec<Vec<Voicing>> = progression
        .iter()
        .map(|chord| {
            generate_voicings_for_chord(*chord, key)
                .into_iter()
                .map(|(_score, v)| v)
                .collect()
        })
        .collect();

    if let Some(start) = starting_voicing {
        all_voicings[0] = vec![start];
    }

    let total_permutations: usize = all_voicings
        .iter()
        .map(|v| v.len())
        .product();

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

    results.sort_unstable_by_key(|(score, _)| *score);

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

    let all_voicings: Vec<Vec<(u16, Voicing)>> = progression
        .iter()
        .map(|chord| generate_voicings_for_chord(*chord, key))
        .collect();

    let (first_scores, first_voicings): (Vec<u16>, Vec<Voicing>) = if let Some(start) = starting_voicing {
        let score = score_single(start, progression[0], key).expect("starting voicing must be valid");
        (vec![score], vec![start])
    } else {
        all_voicings[0].iter().copied().unzip()
    };

    let mut results = Vec::new();

    // index-based backtracking
    let mut current_indices = Vec::with_capacity(progression.len());

    for idx in 0..first_voicings.len() {
        current_indices.clear();
        current_indices.push(idx);

        backtrack_indexed(
            &mut current_indices,
            first_scores[idx],
            progression,
            key,
            1,
            &all_voicings,
            &first_voicings,
            &mut results,
        );
    }

    results.sort_unstable_by_key(|(score, _)| *score);

    results
}

fn backtrack_indexed(
    current_indices: &mut Vec<usize>,
    current_score: u16,
    progression: &[RomanChord],
    key: Key,
    chord_index: usize,
    all_voicings: &[Vec<(u16, Voicing)>],
    first_voicings: &[Voicing],
    results: &mut Vec<(u16, Vec<Voicing>)>,
) {
    if chord_index >= progression.len() {
        let solution: Vec<Voicing> = current_indices
            .iter()
            .enumerate()
            .map(|(i, &idx)| {
                if i == 0 {
                    first_voicings[idx]
                } else {
                    all_voicings[i][idx].1
                }
            })
            .collect();
        results.push((current_score, solution));
        return;
    }

    let current_chord = progression[chord_index];
    let previous_chord = progression[chord_index - 1];

    let prev_idx = *current_indices.last().unwrap();
    let previous_voicing = if chord_index == 1 {
        first_voicings[prev_idx]
    } else {
        all_voicings[chord_index - 1][prev_idx].1
    };

    let candidate_voicings = &all_voicings[chord_index];

    for (voicing_idx, &(voicing_score, voicing)) in candidate_voicings.iter().enumerate() {
        let Ok(window_score) = score_window(previous_voicing, voicing, previous_chord, current_chord, key) else {
            continue;
        };

        let new_score = current_score + voicing_score + window_score;

        current_indices.push(voicing_idx);

        backtrack_indexed(current_indices, new_score, progression, key, chord_index + 1, all_voicings, first_voicings, results);

        current_indices.pop();
    }
}

fn generate_voicings_for_chord(chord: RomanChord, key: Key) -> Vec<(u16, Voicing)> {
    let pitches = chord.pitches(key);

    let soprano_notes = generate_notes_in_range(&pitches, Voice::Soprano.range());

    let alto_notes = generate_notes_in_range(&pitches, Voice::Alto.range());

    let tenor_notes = generate_notes_in_range(&pitches, Voice::Tenor.range());

    let bass_notes = generate_notes_in_range(&pitches, Voice::Bass.range());

    let max_capacity = soprano_notes.len() * alto_notes.len() * tenor_notes.len() * bass_notes.len();
    let mut voicings = Vec::with_capacity(max_capacity);

    for &s in &soprano_notes {
        for &a in &alto_notes {
            for &t in &tenor_notes {
                for &b in &bass_notes {
                    let voicing = Voicing::new([s, a, t, b]);

                    if let Ok(score) = score_single(voicing, chord, key) {
                        voicings.push((score, voicing));
                    }

                }
            }
        }
    }

    voicings
}

fn generate_notes_in_range(pitches: &[Pitch], range: RangeInclusive<Note>) -> Vec<Note> {
    let (min, max) = (range.start(), range.end());

    // each pitch can appear in ~3-4 octaves typically
    let estimated_capacity = pitches.len() * 4;
    let mut notes = Vec::with_capacity(estimated_capacity);

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

#[cfg(test)]
mod tests {
    use crate::pitch::Pitch;
    use crate::key::Key;
    use crate::note::Note;
    use crate::voice_leading::roman_chord::{RomanChord, ScaleDegree};
    use crate::voice_leading::solve::{brute_force_search, generate_voice_leadings};
    use crate::voice_leading::Voicing;

    // this should take around ~30s to run in release
    #[test]
    fn backtracking_solver_finds_all() {
        use ScaleDegree as D;

        let key = Key::major(Pitch::E_FLAT);

        let progression = [
            RomanChord::diatonic_in_key(D::I, key, false),
            RomanChord::diatonic_in_key(D::V, key, false).with_inversion(1).expect("valid inversion"),
            RomanChord::diatonic_in_key(D::I, key, false),
            RomanChord::diatonic_in_key(D::IV, key, false),
            RomanChord::diatonic_in_key(D::V, key, true),
            RomanChord::diatonic_in_key(D::I, key, false),
        ];

        // without starting chord, the brute force gets issued a SIGKILL
        let starting_chord = Voicing([
            Note::new(Pitch::B_FLAT, 4),
            Note::new(Pitch::E_FLAT, 4),
            Note::new(Pitch::G, 3),
            Note::new(Pitch::E_FLAT, 3),
        ]);

        let brute = brute_force_search(&progression, key, Some(starting_chord));

        let solver = generate_voice_leadings(&progression, key, Some(starting_chord));

        assert_eq!(
            brute, solver,
            "brute force and optimized solver should produce the same results"
        );
    }
}