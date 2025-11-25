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