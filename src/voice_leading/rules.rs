use std::cmp::Ordering;
use strum::IntoEnumIterator;
use crate::interval::{Interval, IntervalQuality};
use crate::key::Key;
use crate::note::Note;
use crate::pcset::PitchClassSet;
use crate::pitch::Pitch;
use crate::prelude::IntervalNumber;
use crate::voice_leading::roman_chord::{Quality, RomanChord, ScaleDegree};
use crate::voice_leading::{Voice, Voicing};

pub fn check_range(v: Voicing) -> Result<(), Voice> {
    for voice in Voice::iter() {
        if !voice.range().contains(&v[voice]) {
            return Err(voice);
        }
    }

    Ok(())
}

// this does not check spelling, which it probably should
pub fn check_completely_voiced(v: Voicing, chord: RomanChord, key: Key) -> bool {
    let voicing_set = v.into_iter()
        .map(|p| p.as_pitch_class())
        .collect::<PitchClassSet>();

    let chord_pitches = chord.pitches(key);

    let full_chord = chord_pitches.iter()
        .copied()
        .map(Pitch::as_pitch_class)
        .collect::<PitchClassSet>();

    let eliminated_fifth = full_chord.with_cleared(chord_pitches[2].as_pitch_class());

    // duplicate rule?
    let can_eliminate_fifth = !chord.has_seventh() // TODO: not sure about these rules
        && !(chord.triad_quality == Quality::Augmented)
        && !(chord.triad_quality == Quality::Diminished);

    // sevenths must be fully voiced
    // also, eliminating the fifth is only valid in some cases
    // TODO: needs to check that the previous chord has a seventh
    voicing_set == full_chord || (can_eliminate_fifth && voicing_set == eliminated_fifth)
}

pub fn check_bass_note(v: Voicing, chord: RomanChord, key: Key) -> bool {
    v[Voice::Bass].pitch == chord.bass(key)
}

// TODO: not sure if this method is right
pub fn check_root_position_doubling(voicing: Voicing, chord: RomanChord, key: Key) -> bool {
    // sanity check it's fully voiced
    assert!(
        check_completely_voiced(voicing, chord, key),
        "chord must be completely voiced for doubling check",
    );

    if chord.inversion() != 0 || chord.has_seventh() {
        return true;
    }

    let root = chord.root_in_key(key).as_pitch_class();

    // don't double the leading tone!
    if RomanChord::mode_has_raised_leading_tone(key.mode) && chord.degree == ScaleDegree::VII {
        return true;
    }

    voicing.iter()
        .map(|n| n.as_pitch_class())
        .filter(|&p| p == root)
        .count() >= 2
}

pub fn check_leading_tone_not_doubled(v: Voicing, chord: RomanChord, key: Key) -> bool {
    // sanity check it's fully voiced
    assert!(
        check_completely_voiced(v, chord, key),
        "chord must be completely voiced for doubling check",
    );

    let voicing_pitch_classes = v.iter()
        .map(|n| n.pitch)
        .collect::<Vec<_>>();

    let chord_pitches = chord.pitches(key);

    let leading_tone = {
        let mut vii = key.scale().build_default()[6];

        if RomanChord::mode_has_raised_leading_tone(key.mode) {
            vii = vii.transpose(Interval::AUGMENTED_UNISON);
        }

        vii
    };

    let chordal_seventh = chord.has_seventh().then(|| chord_pitches[3]);

    for pc in voicing_pitch_classes.iter() {
        let count = voicing_pitch_classes.iter().filter(|&p| p == pc).count();

        if count > 1 {
            if *pc == leading_tone {
                return false;
            }

            assert!(
                chordal_seventh.is_none_or(|seventh| *pc != seventh),
                "chordal seventh doubling should've already been caught",
            );
        }
    }

    true
}

pub fn check_six_four_doubling(v: Voicing, chord: RomanChord, key: Key) -> bool {
    // sanity check the chord is voiced correctly
    assert!(
        check_completely_voiced(v, chord, key),
        "chord must be completely voiced for 6/4 doubling check",
    );

    // also, ensure the bass is correct
    assert!(
        check_bass_note(v, chord, key),
        "bass note must be correct for 6/4 doubling check",
    );

    if chord.inversion() != 2 || chord.has_seventh() {
        return true;
    }

    let bass_pc = chord.bass(key).as_pitch_class();

    let count = v.iter()
        .filter(|n| n.as_pitch_class() == bass_pc)
        .count();

    count >= 2
}

pub fn check_spacing(v: Voicing) -> Result<(), (Voice, Voice, Interval)> {
    let [s, a, t, b] = *v;

    // TODO: this should maybe check diatonically
    let octave_range = Interval::PERFECT_OCTAVE.semitones();
    let octave_range = 0..=octave_range.0;

    let tenth_range = Interval::MAJOR_TENTH.semitones();
    let tenth_range = 0..=tenth_range.0;

    let a_s = a.distance_to(s);

    if !octave_range.contains(&a_s.semitones().0) {
        return Err((Voice::Soprano, Voice::Alto, a_s));
    }

    let t_a = t.distance_to(a);

    if !octave_range.contains(&t_a.semitones().0) {
        return Err((Voice::Alto, Voice::Tenor, t_a));
    }

    let b_t = b.distance_to(t);

    if !tenth_range.contains(&b_t.semitones().0) {
        return Err((Voice::Tenor, Voice::Bass, b_t));
    }

    assert!(
        s >= a && a >= t && t >= b,
        "voice ordering should've been caught by 0 boundary"
    );

    Ok(())
}

pub fn check_parallel_interval(first: Voicing, second: Voicing, interval: Interval) -> Result<(), (Voice, Voice)> {
    fn check(v1: Note, v2: Note, interval: Interval) -> bool {
        v1.distance_to(v2).as_simple().abs().semitones() == interval.semitones()
    }

    // TODO: this double checks
    for v1 in Voice::iter() {
        for v2 in Voice::iter() {
            if v2 <= v1 {
                continue;
            }

            let v1_first = first[v1];
            let v2_first = first[v2];
            let v1_second = second[v1];
            let v2_second = second[v2];

            if v1_first != v2_first // oblique is fine
                && check(v1_first, v2_first, interval)
                && check(v1_second, v2_second, interval)
            {
                return Err((v1, v2));
            }
        }
    }

    Ok(())
}

pub fn check_unequal_fifths(first: Voicing, second: Voicing) -> Result<(), (Voice, Voice)> {
    for v1 in Voice::iter() {
        for v2 in Voice::iter() {
            if v2 <= v1 {
                continue;
            }

            let v1_first = first[v1];
            let v2_first = first[v2];
            let v1_second = second[v1];
            let v2_second = second[v2];

            let first_interval = v1_first.distance_to(v2_first).as_simple();
            let second_interval = v1_second.distance_to(v2_second).as_simple();

            let is_perfect_to_dim = first_interval == Interval::PERFECT_FIFTH
                && second_interval == Interval::DIMINISHED_FIFTH;

            let is_dim_to_perfect = first_interval == Interval::DIMINISHED_FIFTH
                && second_interval == Interval::PERFECT_FIFTH;

            if is_perfect_to_dim || is_dim_to_perfect {
                return Err((v1, v2));
            }
        }
    }

    Ok(())
}

pub fn check_direct_fifths_octaves(first: Voicing, second: Voicing) -> Result<(), Voice> {
    for voice in Voice::iter().skip(1) {
        assert_ne!(
            voice, Voice::Soprano,
            "soprano shouldn't be checked against itself"
        );

        let soprano_first = first[Voice::Soprano];
        let soprano_second = second[Voice::Soprano];
        let other_first = first[voice];
        let other_second = second[voice];

        let s_motion = soprano_first.cmp(&soprano_second);

        // only similar motion is the issue; contrary and oblique is fine
        if !(s_motion == other_first.cmp(&other_second) && s_motion != Ordering::Equal) {
            continue;
        }

        let second_interval = soprano_second.distance_to(other_second).as_simple();

        // only if arriving at a perfect fifth or octave
        if !matches!(second_interval, Interval::PERFECT_FIFTH | Interval::PERFECT_OCTAVE) {
            continue;
        }

        let soprano_motion = soprano_first.distance_to(soprano_second).as_simple().abs();

        if soprano_motion.number() != IntervalNumber::SECOND {
            return Err(voice);
        }
    }

    Ok(())
}

pub fn check_leading_tone_resolution(
    first: Voicing,
    second: Voicing,
    second_chord: RomanChord,
    key: Key,
) -> Result<(), Voice> {
    // for sanity, check the second chord is accurately voiced
    assert!(
        check_completely_voiced(second, second_chord, key),
        "second chord must be completely voiced for leading tone resolution check",
    );

    if second_chord.degree != ScaleDegree::I {
        return Ok(());
    }

    let leading_tone = {
        let mut vii = key.scale().build_default()[6];

        if RomanChord::mode_has_raised_leading_tone(key.mode) {
            vii = vii.transpose(Interval::AUGMENTED_UNISON);
        }

        vii
    };

    if leading_tone.distance_to(key.tonic) != Interval::MINOR_SECOND {
        // this mode does not have a leading tone, so it's fine by default
        return Ok(());
    }

    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        if first_note.as_pitch_class() == leading_tone.as_pitch_class() {
            if second_note.as_pitch_class() != key.tonic.as_pitch_class() {
                return Err(voice);
            }

            if first_note.semitones_to(second_note) != Interval::MINOR_SECOND.semitones() {
                return Err(voice);
            }
        }

        if first_note.as_pitch_class() == leading_tone.as_pitch_class()
            && second_note.as_pitch_class() != key.tonic.as_pitch_class()
        {
            return Err(voice);
        }
    }

    Ok(())
}

pub fn check_chordal_seventh_resolution(
    first: Voicing,
    first_chord: RomanChord,
    second: Voicing,
    key: Key,
) -> Result<(), Voice> {
    if !first_chord.has_seventh() {
        return Ok(());
    }

    let seventh = first_chord.pitches(key)[3];

    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        if first_note.as_pitch_class() == seventh.as_pitch_class()
            && !matches!(-first_note.distance_to(second_note), Interval::MAJOR_SECOND | Interval::MINOR_SECOND)
        {
            return Err(voice)
        }
    }

    Ok(())
}

pub fn check_melodic_intervals(first: Voicing, second: Voicing) -> Result<(), (Voice, Interval)> {
    use IntervalQuality as IQ;

    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        if first_note == second_note {
            continue;
        }

        let interval = first_note.distance_to(second_note);

        match interval.quality() {
            IQ::Augmented(_) => return Err((voice, interval)),
            IQ::Diminished(_) if interval.abs() != Interval::DIMINISHED_FIFTH => return Err((voice, interval)),
            // okay
            IQ::Diminished(_) if interval.abs() == Interval::DIMINISHED_FIFTH => {}
            IQ::Major | IQ::Minor | IQ::Perfect => {}
            _ => unreachable!("all cases covered"),
        }
    }

    Ok(())
}

pub fn check_eliminated_fifths(first_chord: Option<RomanChord>, second_chord: RomanChord, second_voicing: Voicing, key: Key) -> bool {
    assert!(
        check_completely_voiced(second_voicing, second_chord, key),
        "second chord must be completely voiced for eliminated fifth check",
    );

    let voicing_set = second_voicing.into_iter()
        .map(|p| p.as_pitch_class())
        .collect::<PitchClassSet>();
    
    if voicing_set.len() == second_chord.len() {
        return true;
    }

    let prev_is_root_v7 = if let Some(first_chord) = first_chord {
        first_chord.degree == ScaleDegree::V
            && first_chord.has_seventh()
            && first_chord.inversion() == 0
    } else {
        false
    };

    // valid cases
    if second_chord.degree == ScaleDegree::I
        && second_chord.inversion() == 0
        /*&& second_voicing.iter().filter(|n| n.pitch == key.tonic).count() == 3*/
        && prev_is_root_v7
    {
        return true;
    }

    // not a valid case
    false
}

enum VoiceMotion {
    Oblique,
    Contrary,
    Similar,
    Parallel,
}

fn get_motion_between(voice_1: Voice, voice_2: Voice, first: Voicing, second: Voicing) -> VoiceMotion {
    if voice_1 == voice_2 {
        return VoiceMotion::Oblique;
    }

    let soprano_first = first[voice_1];
    let soprano_second = second[voice_1];
    let bass_first = first[voice_2];
    let bass_second = second[voice_2];

    let soprano_motion = soprano_first.distance_to(soprano_second);
    let bass_motion = bass_first.distance_to(bass_second);

    if soprano_motion == Interval::PERFECT_UNISON && bass_motion == Interval::PERFECT_UNISON {
        VoiceMotion::Oblique
    } else if soprano_motion == bass_motion {
        VoiceMotion::Parallel
    } else if soprano_motion.is_ascending() != bass_motion.is_ascending() {
        VoiceMotion::Contrary
    } else {
        VoiceMotion::Similar
    }
}

pub fn score_outer_voice_motion(first: Voicing, second: Voicing) -> u16 {
    match get_motion_between(Voice::Soprano, Voice::Bass, first, second) {
        VoiceMotion::Oblique => 0,
        VoiceMotion::Contrary => 1,
        VoiceMotion::Similar => 2,
        VoiceMotion::Parallel => 4,
    }
}

pub fn score_melodic_intervals(first: Voicing, second: Voicing) -> u16 {
    let mut penalty = 0;

    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        if first_note == second_note {
            continue;
        }

        let semis = first_note.distance_to(second_note)
            .as_simple()
            .semitones()
            .0
            .abs();

        penalty += match semis {
            // unison to step
            0..=2 => 0,
            // min/maj thirds
            3..=4 => 1,
            // fourth / tritone
            5..=6 => 2,
            // fifths
            7 => 4,
            // larger
            8..=12 => 8,
            _ => unreachable!("simple intervals have semitone count in [0,12)"),
        };
    }

    penalty
}

pub fn score_common_tones(first: Voicing, second: Voicing, first_chord: RomanChord, second_chord: RomanChord, key: Key) -> u16 {
    assert!(
        check_completely_voiced(first, first_chord, key),
        "first chord must be completely voiced for common tone scoring",
    );

    assert!(
        check_completely_voiced(second, second_chord, key),
        "second chord must be completely voiced for common tone scoring",
    );

    let common_pcs = first_chord.pitch_class_set(key) & second_chord.pitch_class_set(key);

    if common_pcs.is_empty() {
        return 0;
    }

    let mut penalty = 0;

    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        let first_pc = first_note.as_pitch_class();

        if common_pcs.is_set(first_pc) {
            if first_note != second_note {
                penalty += 1;
            }
        }
    }

    penalty
}

pub fn score_unison(v: Voicing) -> u16 {
    let mut penalty = 0;

    for v1 in Voice::iter() {
        for v2 in Voice::iter() {
            if v2 > v1 && v[v1] == v[v2] {
                penalty += 1;
            }
        }
    }

    penalty
}
