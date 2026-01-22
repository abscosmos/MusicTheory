use crate::Interval;
use crate::voice_leading::{Voice, Voicing};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VoiceMotion {
    Oblique,
    Contrary,
    Similar,
    Parallel,
}

pub fn get_motion_between(voice_1: Voice, voice_2: Voice, first: Voicing, second: Voicing) -> VoiceMotion {
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