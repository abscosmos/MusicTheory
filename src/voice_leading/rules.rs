use crate::interval::Interval;
use crate::voice_leading::{Voice, Voicing};

pub fn check_spacing(v: Voicing) -> Result<(), (Voice, Voice)> {
    let [s, a, t, b] = *v;

    // TODO: this should maybe check diatonically
    let octave_semis = Interval::PERFECT_OCTAVE.semitones();
    let tenth_semis = Interval::MAJOR_TENTH.semitones();


    if s.semitones_to(a) > octave_semis {
        return Err((Voice::Soprano, Voice::Alto));
    }

    if a.semitones_to(t) > octave_semis {
        return Err((Voice::Alto, Voice::Tenor));
    }

    if t.semitones_to(b) > tenth_semis {
        return Err((Voice::Tenor, Voice::Bass));
    }

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

            if v1_first.semitones_to(v2_first) == interval.semitones()
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

