use crate::Interval;
use crate::interval::Number;
use crate::voice_leading::{Voice, Voicing};

pub struct SpacingRule {
    alto_soprano: Number,
    tenor_alto: Number,
    bass_tenor: Number,
}

impl SpacingRule {
    pub fn new(alto_soprano: Number, tenor_alto: Number, bass_tenor: Number) -> Option<Self> {
        if alto_soprano.is_ascending() && tenor_alto.is_ascending() && bass_tenor.is_ascending() {
            Some(Self { alto_soprano, tenor_alto, bass_tenor })
        } else {
            None
        }
    }

    pub const fn between(&self, bottom: Voice, top: Voice) -> Option<Number> {
        match (bottom, top) {
            (Voice::Alto, Voice::Soprano) => Some(self.alto_soprano),
            (Voice::Tenor, Voice::Alto) => Some(self.tenor_alto),
            (Voice::Bass, Voice::Tenor) => Some(self.bass_tenor),
            _ => None,
        }
    }

    pub fn evaluate(&self, v: Voicing) -> Result<(), ((Voice, Voice), Number)> {
        // TODO: create function to just get the number between two notes / pitches
        let [s, a, t ,b] = *v;

        let a_s = a.distance_to(s).number();

        if !a_s.is_ascending() || a_s > self.alto_soprano {
            return Err(((Voice::Alto, Voice::Soprano), a_s));
        }

        let t_a = t.distance_to(a).number();

        if !t_a.is_ascending() || t_a > self.tenor_alto {
            return Err(((Voice::Tenor, Voice::Alto), t_a));
        }

        let b_t = b.distance_to(t).number();

        if !b_t.is_ascending() || b_t > self.bass_tenor {
            return Err(((Voice::Bass, Voice::Tenor), b_t));
        }

        Ok(())
    }
}