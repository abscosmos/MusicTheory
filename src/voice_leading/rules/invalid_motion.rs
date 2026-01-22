use std::cmp::Ordering;
use strum::IntoEnumIterator;
use crate::{Interval, Note};
use crate::interval::{Number, Quality};
use crate::voice_leading::{Voice, Voicing};
use crate::voice_leading::motion::{get_motion_between, VoiceMotion};

pub fn parallel_interval(first: Voicing, second: Voicing, interval: Interval) -> Result<(), (Voice, Voice)> {
    for v1 in Voice::iter() {
        for v2 in Voice::iter() {
            if v2 <= v1 {
                continue;
            }

            let v1_first = first[v1];
            let v2_first = first[v2];
            let v1_second = second[v1];
            let v2_second = second[v2];

            if v1_first != v1_second // oblique is fine
                && (v1_first.distance_to(v2_first).as_simple() == interval)
                && (v1_second.distance_to(v2_second).as_simple() == interval)
            {
                return Err((v1, v2));
            }
        }
    }

    Ok(())
}

pub fn unequal_fifths(first: Voicing, second: Voicing) -> Result<(), (Voice, Voice)> {
    for v1 in Voice::iter() {
        for v2 in Voice::iter() {
            if v2 <= v1 {
                continue;
            }

            let v1_first = first[v1];
            let v2_first = first[v2];
            let v1_second = second[v1];
            let v2_second = second[v2];

            let first_interval = v1_first.distance_to(v2_first).as_simple();
            let second_interval = v1_second.distance_to(v2_second).as_simple();

            let is_perfect_to_dim = first_interval == Interval::PERFECT_FIFTH
                && second_interval == Interval::DIMINISHED_FIFTH;

            let is_dim_to_perfect = first_interval == Interval::DIMINISHED_FIFTH
                && second_interval == Interval::PERFECT_FIFTH;

            if is_perfect_to_dim || is_dim_to_perfect {
                return Err((v1, v2));
            }
        }
    }

    Ok(())
}

pub fn direct_fifths_octaves(first: Voicing, second: Voicing) -> Result<(), Voice> {
    for voice in Voice::iter().skip(1) {
        assert_ne!(
            voice, Voice::Soprano,
            "soprano shouldn't be checked against itself"
        );

        let soprano_first = first[Voice::Soprano];
        let soprano_second = second[Voice::Soprano];
        let other_second = second[voice];

        if get_motion_between(Voice::Soprano, voice, first, second) != VoiceMotion::Similar {
            continue;
        }

        let second_interval = soprano_second.distance_to(other_second).as_simple();

        // only if arriving at a perfect fifth or octave
        if !matches!(second_interval, Interval::PERFECT_FIFTH | Interval::PERFECT_OCTAVE) {
            continue;
        }

        let soprano_motion = soprano_first.distance_to(soprano_second).as_simple().abs();

        if soprano_motion.number() != Number::SECOND {
            return Err(voice);
        }
    }

    Ok(())
}

pub fn similar_into_unison(first: Voicing, second: Voicing) -> Result<(), (Voice, Voice)> {
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

pub fn melodic_intervals(first: Voicing, second: Voicing) -> Result<(), (Voice, Interval)> {
    for voice in Voice::iter() {
        let first_note = first[voice];
        let second_note = second[voice];

        if first_note == second_note {
            continue;
        }

        let interval = first_note.distance_to(second_note);

        match interval.quality() {
            Quality::Augmented(_) => return Err((voice, interval)),
            Quality::Diminished(_) if interval.abs() != Interval::DIMINISHED_FIFTH => return Err((voice, interval)),
            // okay
            Quality::Diminished(_) if interval.abs() == Interval::DIMINISHED_FIFTH => {}
            Quality::Major | Quality::Minor | Quality::Perfect => {}
            _ => unreachable!("all cases covered"),
        }
    }

    Ok(())
}