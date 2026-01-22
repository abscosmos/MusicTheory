use strum::IntoEnumIterator;
use crate::voice_leading::{get_motion_between, Voice, VoiceMotion, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct SimilarIntoUnison;

impl SimilarIntoUnison {
    pub fn evaluate(&self, first: Voicing, second: Voicing) -> Result<(), (Voice, Voice)> {
        for v1 in Voice::iter() {
            for v2 in Voice::iter() {
                if v2 > v1
                    && second[v1] == second[v2]
                    && get_motion_between(v1, v2, first, second) == VoiceMotion::Similar
                {
                    return Err((v1, v2));
                }
            }
        }

        Ok(())
    }
}
