use crate::interval::Interval;
use crate::key::Key;
use crate::note::Note;
use crate::pcset::PitchClassSet;
use crate::pitch::Pitch;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};

pub fn check_range(v: Voicing) -> Result<(), Voice> {
    let [s, a, t, b] = *v;

    const SOPRANO_MIN: Note = Note::new(Pitch::C, 4);
    const SOPRANO_MAX: Note = Note::new(Pitch::G, 5);
    const ALTO_MIN: Note = Note::new(Pitch::G, 3);
    const ALTO_MAX: Note = Note::new(Pitch::C, 5);
    const TENOR_MIN: Note = Note::new(Pitch::C, 3);
    const TENOR_MAX: Note = Note::new(Pitch::G, 4);
    const BASS_MIN: Note = Note::new(Pitch::E, 2);
    const BASS_MAX: Note = Note::new(Pitch::C, 4);

    if !(SOPRANO_MIN..=SOPRANO_MAX).contains(&s) {
        return Err(Voice::Soprano);
    }

    if !(ALTO_MIN..=ALTO_MAX).contains(&a) {
        return Err(Voice::Alto);
    }

    if !(TENOR_MIN..=TENOR_MAX).contains(&t) {
        return Err(Voice::Tenor);
    }

    if !(BASS_MIN..=BASS_MAX).contains(&b) {
        return Err(Voice::Bass);
    }

    Ok(())
}

// this does not check spelling, which it probably should
pub fn completely_voiced(v: Voicing, chord: RomanChord, key: Key) -> bool {
    let voicing_set = v.into_iter()
        .map(|p| p.as_pitch_class())
        .collect::<PitchClassSet>();

    let chord_pitches = chord.pitches(key);

    let full_chord = chord_pitches.iter()
        .copied()
        .map(Pitch::as_pitch_class)
        .collect::<PitchClassSet>();

    let eliminated_fifth = full_chord.with_cleared(chord_pitches[2].as_pitch_class());

    // sevenths must be fully voiced
    // also, eliminating the fifth is only valid in some cases
    voicing_set == full_chord || (chord.has_seventh() && voicing_set == eliminated_fifth)
}

pub fn check_bass_note(v: Voicing, chord: RomanChord, key: Key) -> bool {
    v[Voice::Bass].pitch == chord.bass(key)
}

pub fn check_spacing(v: Voicing) -> Result<(), (Voice, Voice)> {
    let [s, a, t, b] = *v;

    // TODO: this should maybe check diatonically
    let octave_range = Interval::PERFECT_OCTAVE.semitones();
    let octave_range = 0..=octave_range.0;

    let tenth_range = Interval::MAJOR_TENTH.semitones();
    let tenth_range = 0..=tenth_range.0;


    if !octave_range.contains(&s.semitones_to(a).0) {
        return Err((Voice::Soprano, Voice::Alto));
    }

    if !octave_range.contains(&a.semitones_to(t).0) {
        return Err((Voice::Alto, Voice::Tenor));
    }

    if !tenth_range.contains(&t.semitones_to(b).0) {
        return Err((Voice::Tenor, Voice::Bass));
    }

    assert!(
        s >= a && a >= t && t >= b,
        "voice ordering should've been caught by 0 boundary"
    );

    Ok(())
}

fn check_parallel_interval(first: Voicing, second: Voicing, interval: Interval) -> Result<(), (Voice, Voice)> {
    const NUM_VOICES: usize = 4;

    // make indexing easier
    let (first, second) = (*first, *second);

    for i in 0..NUM_VOICES {
        for j in (i + 1)..NUM_VOICES {
            let v1_first = first[i];
            let v2_first = first[j];
            let v1_second = second[i];
            let v2_second = second[j];

            if v1_first != v2_first
                && v1_first.semitones_to(v2_first) == interval.semitones()
                && v1_second.semitones_to(v2_second) == interval.semitones()
            {
                return Err((
                    Voice::from_repr(i as u8).expect("valid voice"),
                    Voice::from_repr(j as u8).expect("valid voice")
                ));
            }
        }
    }

    Ok(())
}

pub fn check_parallel_fifths(first: Voicing, second: Voicing) -> Result<(), (Voice, Voice)> {
    check_parallel_interval(first, second, Interval::PERFECT_FIFTH)
}

pub fn check_parallel_octaves(first: Voicing, second: Voicing) -> Result<(), (Voice, Voice)> {
    check_parallel_interval(first, second, Interval::PERFECT_OCTAVE)
}

