use crate::interval::Interval;
use crate::key::Key;
use crate::voice_leading::roman_chord::RomanChord;
use crate::voice_leading::{Voice, Voicing};
use crate::voice_leading::rules::{score_common_tones, score_melodic_intervals, score_outer_voice_motion};

pub struct VoiceLeadingError {
    pub kind: VoiceLeadingErrorKind,
    pub location: u16,
}

pub enum VoiceLeadingErrorKind {
    MismatchedSizes,
    OutOfRange(Voice),
    InvalidSpacing(Voice, Voice),
    IncompleteVoicing,
    InvalidBass,
    IllegalDoubling,
    IllegalParallel(Voice, Voice, Interval),
    UnequalFifths(Voice, Voice),
    DirectFifths(Voice),
    LeadingToneNotResolved(Voice),
    ChordalSeventhNotResolved(Voice),
    InvalidMelodicInterval(Voice, Interval),
}

pub fn score_single(voicing: Voicing, chord: RomanChord, key: Key) -> Result<u16, VoiceLeadingErrorKind> {
    use crate::voice_leading::rules::{
        check_bass_note,
        check_completely_voiced,
        check_no_illegal_doubling,
        check_range,
        check_six_four_doubling,
        check_spacing,
    };

    use VoiceLeadingErrorKind as Kind;

    // 1. range & spacing
    check_range(voicing).map_err(|voice| Kind::OutOfRange(voice))?;

    check_spacing(voicing).map_err(|(v1, v2)| Kind::InvalidSpacing(v1, v2))?;

    // 2. voicing
    if !check_completely_voiced(voicing, chord, key) {
        return Err(Kind::IncompleteVoicing);
    }

    if !check_bass_note(voicing, chord, key) {
        return Err(Kind::InvalidBass);
    }

    // 3. doubling
    if !check_no_illegal_doubling(voicing, chord, key) {
        return Err(Kind::IllegalDoubling);
    }

    if !check_six_four_doubling(voicing, chord, key) {
        return Err(Kind::IllegalDoubling);
    }
    
    Ok(0)
}

pub fn score_window(v_first: Voicing, v_second: Voicing, c_first: RomanChord, c_second: RomanChord, key: Key) -> Result<u16, VoiceLeadingErrorKind> {
    use crate::voice_leading::rules::{
        check_direct_fifths_octaves,
        check_parallel_fifths,
        check_parallel_octaves,
        check_unequal_fifths,
        check_leading_tone_resolution,
        check_chordal_seventh_resolution,
        check_melodic_intervals,
    };
    
    use VoiceLeadingErrorKind as Kind;

    // 4. parallels
    check_parallel_fifths(v_first, v_second).map_err(|(v1, v2)| Kind::IllegalParallel(v1, v2, Interval::PERFECT_FIFTH))?;

    check_parallel_octaves(v_first, v_second).map_err(|(v1, v2)| Kind::IllegalParallel(v1, v2, Interval::PERFECT_OCTAVE))?;

    // 5. fifths
    check_unequal_fifths(v_first, v_second).map_err(|(v1, v2)| Kind::UnequalFifths(v1, v2))?;

    check_direct_fifths_octaves(v_first, v_second).map_err(|voice| Kind::DirectFifths(voice))?;

    // 6. resolution
    check_leading_tone_resolution(v_first, v_second, c_second, key).map_err(|voice| Kind::LeadingToneNotResolved(voice))?;


    check_chordal_seventh_resolution(v_first, c_first, v_second, key).map_err(|voice| Kind::ChordalSeventhNotResolved(voice))?;

    // 7. intervals
    check_melodic_intervals(v_first, v_second).map_err(|(voice, interval)| Kind::InvalidMelodicInterval(voice, interval))?;
    
    // 8. scores
    let mut score = 0;
    
    score += score_outer_voice_motion(v_first, v_second) * 2;
    
    score += score_melodic_intervals(v_first, v_second) * 2;
    
    score += score_common_tones(v_first, v_second, c_first, c_second, key);
    
    Ok(score)
}

pub fn check_voice_leading(key: Key, progression: &[RomanChord], voicings: &[Voicing]) -> Result<u16, VoiceLeadingError> {
    if progression.len() != voicings.len() {
        return Err(VoiceLeadingError {
            kind: VoiceLeadingErrorKind::MismatchedSizes,
            location: 0,
        });
    }

    if progression.len() == 0 {
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