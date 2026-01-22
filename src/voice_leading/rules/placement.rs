use strum::IntoEnumIterator;
use crate::Interval;
use crate::voice_leading::{Voice, Voicing};

pub fn range(v: Voicing) -> Result<(), Voice> {
    for voice in Voice::iter() {
        if !voice.range().contains(&v[voice]) {
            return Err(voice);
        }
    }

    Ok(())
}

pub fn spacing(v: Voicing) -> Result<(), (Voice, Voice, Interval)> {
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

    debug_assert!(
        s >= a && a >= t && t >= b,
        "voice ordering should've been caught by 0 boundary"
    );

    Ok(())
}