use std::cmp::Ordering;
use std::ops::Index;
use const_soft_float::soft_f32::SoftF32;
use serde::{Deserialize, Serialize};
use typed_floats::tf32::{self, StrictlyPositiveFinite};
use crate::note::Note;
use crate::pitch_class::PitchClass;
use crate::tuning::{Cents, Tuning};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JustIntonation {
    pub reference: Note,
    pub freq_hz: StrictlyPositiveFinite,
    pub ratios: JustIntonationRatios,
    pub base: PitchClass,
}

impl JustIntonation {
    pub const A4_440_LIMIT_5: Self = Self::new(Note::A4, 440.0, JustIntonationRatios::LIMIT_5, PitchClass::C)
        .expect("440 is in (0, inf)");

    pub const fn new(reference: Note, freq_hz: f32, ratios: JustIntonationRatios, base: PitchClass) -> Option<Self> {
        match StrictlyPositiveFinite::new(freq_hz) {
            Ok(freq_hz) => Some(Self { reference, freq_hz, ratios, base }),
            Err(_) => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct JustIntonationRatios([StrictlyPositiveFinite; 12]);

impl JustIntonationRatios {
    pub const LIMIT_5: Self = {
        let Ok(ratios) = Self::with_ratios(
            16.0/15.0,
            9.0/8.0,
            6.0/5.0,
            5.0/4.0,
            4.0/3.0,
            45.0/32.0,
            3.0/2.0,
            8.0/5.0,
            5.0/3.0,
            9.0/5.0,
            15.0/8.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    // this calls Self::with_ratios internally for a single source of truth
    pub const fn new(ratios: [StrictlyPositiveFinite; 12]) -> Result<Self, JustIntonationRatiosError> {
        let [
            unison,
            minor_second,
            major_second,
            minor_third,
            major_third,
            perfect_fourth,
            tritone,
            perfect_fifth,
            minor_sixth,
            major_sixth,
            minor_seventh,
            major_seventh,
        ] = ratios;

        // if unison != 1.0 (complicated because of const)
        if !matches!(SoftF32(unison.get()).cmp(SoftF32(1.0)), Some(Ordering::Equal)) {
            return Err(JustIntonationRatiosError::UnisonNotIdentity);
        }

        Self::with_ratios(
            minor_second.get(),
            major_second.get(),
            minor_third.get(),
            major_third.get(),
            perfect_fourth.get(),
            tritone.get(),
            perfect_fifth.get(),
            minor_sixth.get(),
            major_sixth.get(),
            minor_seventh.get(),
            major_seventh.get(),
        )
    }

    pub const fn with_ratios(
        minor_second: f32,
        major_second: f32,
        minor_third: f32,
        major_third: f32,
        perfect_fourth: f32,
        tritone: f32,
        perfect_fifth: f32,
        minor_sixth: f32,
        major_sixth: f32,
        minor_seventh: f32,
        major_seventh: f32,
    ) -> Result<Self, JustIntonationRatiosError> {
        let ratios = [
            1.0,
            minor_second,
            major_second,
            minor_third,
            major_third,
            perfect_fourth,
            tritone,
            perfect_fifth,
            minor_sixth,
            major_sixth,
            minor_seventh,
            major_seventh,
        ];

        let mut res = [tf32::MAX; 12];

        // check strictly ascending
        let mut i = 0;
        while i < ratios.len() {
            if i + 1 != ratios.len() &&
                matches!(
                    SoftF32(ratios[i]).cmp(SoftF32(ratios[i + 1])),
                    Some(Ordering::Greater | Ordering::Equal),
                )
            {
                return Err(JustIntonationRatiosError::NotStrictlyIncreasing);
            }

            res[i] = match StrictlyPositiveFinite::new(ratios[i]) {
                Ok(checked) => checked,
                _ => return Err(JustIntonationRatiosError::InvalidRatio),
            };

            i += 1;
        }

        // instead of checking if all are in [1.0, 2.0), since already checked strictly increasing
        // and first is 1.0, only need to check last!
        match SoftF32(*ratios.last().expect("should have 12 elems")).cmp(SoftF32(2.0)) {
            Some(Ordering::Less | Ordering::Equal) => Ok(Self(res)),
            Some(Ordering::Greater) => Err(JustIntonationRatiosError::InvalidRatio),
            None => panic!("unreachable!: uncomparable values already handled"),
        }
    }
}

impl Tuning for JustIntonation {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, Cents)> {
        let ref_offset = self.base.semitones_to(self.reference.as_pitch_class()).0 as usize;
        let ref_to_base = self.ratios[ref_offset];
        let base0_freq = self.freq_hz.get() * ref_to_base.recip().get() * 2.0_f32.powf(-self.reference.octave as _);

        let ratio_from_base0 = hz.get() / base0_freq;
        let octave = ratio_from_base0.log2().floor() as i16;
        let ratio_within_octave = StrictlyPositiveFinite::new(ratio_from_base0 / 2.0_f32.powf(octave as _))
            .expect("ratio shouldn't be negative, nan, or infinity (unless octave is very very large)");

        let best_pc = (0..12)
            .map(|c| PitchClass::from_repr(c).expect("in range"))
            .min_by_key(|&pc| {
                let offset = self.base.semitones_to(pc).0 as usize;
                (self.ratios[offset] - ratio_within_octave).abs()
            })?;

        let best_note = Note {
            pitch: best_pc.into(),
            octave,
        };

        let best_offset = self.base.semitones_to(best_pc).0 as usize;
        let cents = Cents::between_frequencies(self.ratios[best_offset], ratio_within_octave)?;

        debug_assert!(
            (cents.get() - Cents::from_note(best_note, hz, self).expect("should be in range").get()).abs() < 0.01,
            "using difference within an octave should be valid",
        );

        Some((best_note, cents))
    }

    fn note_to_freq_hz(&self, note: Note) -> Option<StrictlyPositiveFinite> {
        let pitch_offset = self.base.semitones_to(note.pitch.as_pitch_class()).0 as usize;
        let pitch_ratio = self.ratios[pitch_offset];

        let ref_offset = self.base.semitones_to(self.reference.pitch.as_pitch_class()).0 as usize;
        let ref_to_base = self.ratios[ref_offset];

        let octave_diff = (note.octave - self.reference.octave) as _;

        // reference_freq * (base / reference_pitch) * 2^(octave_diff) * pitch_ratio
        let hz = self.freq_hz.get()
            * ref_to_base.recip().get()
            * 2.0_f32.powf(octave_diff)
            * pitch_ratio.get();

        StrictlyPositiveFinite::new(hz).ok()
    }
}

impl Default for JustIntonation {
    fn default() -> Self {
        Self::A4_440_LIMIT_5
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum JustIntonationRatiosError {
    #[error("Ratio between unisons must be 1/1")]
    UnisonNotIdentity,
    #[error("Ratios must be strictly increasing order")]
    NotStrictlyIncreasing,
    #[error("The ratios were not in range [1.0, 2.0)")]
    InvalidRatio,
}

impl Index<usize> for JustIntonationRatios {
    type Output = StrictlyPositiveFinite;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pitch::Pitch;

    /// This test does not check that the returned frequencies are "correct",
    /// as I cannot find a reliable source of note to freq conversions for
    /// any sort of just intonation tuning.
    ///
    /// This simply tests that, between changes in implementation, the results are still the same.
    #[test]
    fn consistent_freq_to_hz() {
        let cases = [
            (Note::new(Pitch::A_SHARP, 1), 59.399998),
            (Note::new(Pitch::B, 2), 123.75),
            (Note::new(Pitch::C, 4), 264.0),
            (Note::new(Pitch::F_SHARP, 5), 742.5),
            (Note::new(Pitch::E, 12), 84480.0),
            (Note::new(Pitch::C_SHARP, 18), 4613734.5),
        ];

        let tuning = JustIntonation::A4_440_LIMIT_5;

        for (note, hz) in cases {
            let hz = StrictlyPositiveFinite::new(hz).expect("hz in range");

            assert_eq!(tuning.note_to_freq_hz(note), Some(hz));
        }
    }
}