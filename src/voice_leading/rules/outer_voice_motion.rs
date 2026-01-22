use crate::Interval;
use crate::voice_leading::{get_motion_between, Voice, VoiceMotion, Voicing};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct OuterVoiceMotion;

impl OuterVoiceMotion {
    pub fn evaluate(&self, first: Voicing, second: Voicing) -> u16 {
        match get_motion_between(Voice::Soprano, Voice::Bass, first, second) {
            VoiceMotion::Oblique => 0,
            VoiceMotion::Contrary => 1,
            VoiceMotion::Similar => 2,
            VoiceMotion::Parallel => 4,
        }
    }
}