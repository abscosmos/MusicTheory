use strum::IntoEnumIterator;
use crate::interval::Number;
use crate::voice_leading::{Voice, Voicing};

pub fn range(v: Voicing) -> Result<(), Voice> {
    for voice in Voice::iter() {
        if !voice.range().contains(&v[voice]) {
            return Err(voice);
        }
    }

    Ok(())
}

pub fn spacing(v: Voicing) -> Result<(), (Voice, Voice, Number)> {
    let [s, a, t, b] = *v;

    let a_s = a.distance_to(s).number();

    if !a_s.is_ascending() || a_s > Number::OCTAVE {
        return Err((Voice::Alto, Voice::Soprano, a_s));
    }

    let t_a = t.distance_to(a).number();

    if !t_a.is_ascending() || t_a > Number::OCTAVE {
        return Err((Voice::Tenor, Voice::Alto, t_a));
    }

    let b_t = b.distance_to(t).number();

    if !b_t.is_ascending() || b_t > Number::TENTH {
        return Err((Voice::Bass, Voice::Tenor, b_t));
    }

    debug_assert!(
        s >= a && a >= t && t >= b,
        "voice ordering should've been caught by 0 boundary"
    );

    Ok(())
}