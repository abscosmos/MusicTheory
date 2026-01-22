use crate::Interval;
use crate::harmony::Key;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("Error in chord {location}: {kind}")]
pub struct VoiceLeadingError {
    pub kind: VoiceLeadingErrorKind,
    pub location: u16,
}

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VoiceLeadingErrorKind {
    #[error("The progression and voicings were different lengths")]
    MismatchedSizes,
    #[error("The {0:?} part was out of range")]
    OutOfRange(Voice),
    #[error("There was in invalid interval of {2} between {0:?} and {1:?}")]
    InvalidSpacing(Voice, Voice, Interval),
    #[error("The chord was not fully voiced")]
    IncompleteVoicing,
    #[error("The bass note was incorrect")]
    InvalidBass,
    #[error("An invalid note was doubled")]
    IllegalDoubling,
    #[error("There was a parallel {2} between {0:?} and {1:?}")]
    IllegalParallel(Voice, Voice, Interval),
    #[error("There were unequal fifths between {0:?} and {1:?}")]
    UnequalFifths(Voice, Voice),
    #[error("There were direct fifths or octaves between {:?} and {0:?}", Voice::Soprano)]
    DirectFifthsOrOctaves(Voice),
    #[error("The leading tone in {0:?} was not resolved")]
    LeadingToneNotResolved(Voice),
    #[error("The chordal seventh in {0:?} was not resolved")]
    ChordalSeventhNotResolved(Voice),
    #[error("There was an invalid melodic interval of {1:?} in {0:?}")]
    InvalidMelodicInterval(Voice, Interval),
    #[error("Both {0:?} and {1:?} moved to a unison by similar motion")]
    SimilarIntoUnison(Voice, Voice),
}

pub fn score_single(voicing: Voicing, chord: RomanChord, key: Key) -> Result<u16, VoiceLeadingErrorKind> {
    use crate::voice_leading::rules::{
        doubling,
        voicing,
        score_motion,
        placement,
    };

    use VoiceLeadingErrorKind as Kind;

    // 1. range & spacing
    placement::range(voicing).map_err(Kind::OutOfRange)?;

    placement::spacing(voicing).map_err(|(v1, v2, ivl)| Kind::InvalidSpacing(v1, v2, ivl))?;

    // 2. voicing
    if !voicing::completely_voiced(voicing, chord, key) {
        return Err(Kind::IncompleteVoicing);
    }

    if !voicing::bass_note(voicing, chord, key) {
        return Err(Kind::InvalidBass);
    }

    // 3. doubling
    if !doubling::leading_tone_not_doubled(voicing, chord, key) {
        return Err(Kind::IllegalDoubling);
    }

    if !doubling::root_position_doubling(voicing, chord, key) {
        return Err(Kind::IllegalDoubling);
    }

    if !doubling::six_four_doubling(voicing, chord, key) {
        return Err(Kind::IllegalDoubling);
    }

    let mut score = 0;

    score += score_motion::unison(voicing) * 3;

    Ok(score)
}

pub fn score_window(v_first: Voicing, v_second: Voicing, c_first: RomanChord, c_second: RomanChord, key: Key) -> Result<u16, VoiceLeadingErrorKind> {
    use crate::voice_leading::rules::{
        resolution,
        doubling,
        voicing,
        invalid_motion,
        score_motion,
        placement,
    };
    
    use VoiceLeadingErrorKind as Kind;

    // 4. parallels
    for interval in [Interval::PERFECT_UNISON, Interval::PERFECT_FIFTH, Interval::PERFECT_OCTAVE] {
        invalid_motion::parallel_interval(v_first, v_second, interval).map_err(|(v1, v2)| Kind::IllegalParallel(v1, v2, interval))?;
    }

    // 5. fifths
    invalid_motion::unequal_fifths(v_first, v_second).map_err(|(v1, v2)| Kind::UnequalFifths(v1, v2))?;

    invalid_motion::direct_fifths_octaves(v_first, v_second).map_err(Kind::DirectFifthsOrOctaves)?;

    // 6. resolution
    resolution::leading_tone_resolution(v_first, v_second, c_second, key).map_err(Kind::LeadingToneNotResolved)?;

    resolution::chordal_seventh_resolution(v_first, c_first, v_second, key).map_err(Kind::ChordalSeventhNotResolved)?;

    // 7. intervals
    invalid_motion::melodic_intervals(v_first, v_second).map_err(|(voice, interval)| Kind::InvalidMelodicInterval(voice, interval))?;

    invalid_motion::similar_into_unison(v_first, v_second).map_err(|(v1, v2)| Kind::SimilarIntoUnison(v1, v2))?;

    // 8. eliminated fifths TODO: this error displays in the wrong place
    if !voicing::eliminated_fifths(Some(c_first), c_second, v_second, key) {
        return Err(Kind::IncompleteVoicing);
    }

    // 9. scores
    let mut score = 0;
    
    score += score_motion::outer_voice_motion(v_first, v_second) * 2;
    
    score += score_motion::melodic_intervals(v_first, v_second) * 2;
    
    score += score_motion::common_tones(v_first, v_second, c_first, c_second, key);
    
    Ok(score)
}

pub fn check_voice_leading(key: Key, progression: &[RomanChord], voicings: &[Voicing]) -> Result<u16, VoiceLeadingError> {
    if progression.len() != voicings.len() {
        return Err(VoiceLeadingError {
            kind: VoiceLeadingErrorKind::MismatchedSizes,
            location: 0,
        });
    }

    if progression.is_empty() {
        return Ok(0);
    }

    let mut score = 0;
    
    let zip = voicings.iter().zip(progression.iter());

    for (loc, (voicing, chord)) in zip.clone().enumerate() {
        score += score_single(*voicing, *chord, key).map_err(|kind| VoiceLeadingError {
            kind,
            location: loc as _,
        })?;
    }

    if voicings.len() == 1 {
        return Ok(score);
    }



    for loc in 0..(voicings.len() - 1) {
        let c_first = progression[loc];
        let c_second = progression[loc + 1];
        let v_first = voicings[loc];
        let v_second = voicings[loc + 1];

        score += score_window(v_first, v_second, c_first, c_second, key)
            .map_err(|kind| VoiceLeadingError {
                kind,
                location: loc as _,
            })?;
    }


    Ok(score)
}